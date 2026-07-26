#!/usr/bin/env bash
# Design-system consumption gate.
#
# Asserts that the product stylesheets *consume* the vendored Libre IA Design
# System instead of restating it. Six checks, all mechanical:
#
#   1. no colour literal in first-party CSS (hex / rgb / hsl / oklch / named)
#   2. no breakpoint in an absolute unit (px/pt/pc/cm/mm/in)
#   3. vendored DS artefacts match the SHA-256 recorded in their manifest
#   4. the two vendored copies (apps/web, crates/ui) stay byte-identical
#   5. colour literals that *cannot* be a CSS var (PWA manifest, <meta
#      theme-color>) still equal the token they mirror
#   6. WCAG contrast of the theme token pairs, recomputed from tokens.css
#
# Portability: pattern matching goes through `awk` on purpose. BSD grep (macOS)
# has no `-P`, and a `grep -P` that fails prints its usage to stderr and exits
# non-zero while producing no stdout — which reads exactly like "no match" and
# would make checks 1, 2 and 5 vacuously green. POSIX ERE via awk behaves the
# same on macOS and on ubuntu-latest.
#
# Why a scanned-file counter: a broken glob would likewise make every "no
# match" check vacuously green. The run fails if it scanned nothing.
#
# Usage: scripts/check-design-system.sh
set -euo pipefail

cd "$(dirname "$0")/.."

status=0
fail() { printf 'FAIL  %s\n' "$1" >&2; status=1; }
pass() { printf 'ok    %s\n' "$1"; }
lower() { printf '%s' "$1" | tr '[:upper:]' '[:lower:]'; }

# Reads the value of a custom property from the vendored token file.
tok() {
  awk -v k="--$1:" '
    index($0, k) == 1 || index($0, "  " k) == 1 {
      sub(/^[ \t]*--[a-zA-Z0-9-]+:[ \t]*/, ""); sub(/;[ \t]*$/, ""); print; exit
    }' crates/ui/assets/tokens.css
}

# Generated / vendored: these legitimately carry literals and are verified by
# digest (check 3), never by content.
is_vendored() {
  case "$1" in
    */tokens.css | */themes.css | */components.css | */libre-ia/*) return 0 ;;
    *) return 1 ;;
  esac
}

all_css=""
first_party_css=""
scanned=0
while IFS= read -r f; do
  # An empty `git ls-files` still yields one blank line through the heredoc;
  # counting it would report "1 scanned" for an empty tree and defeat the
  # zero-file guard below.
  [ -n "$f" ] || continue
  all_css="$all_css $f"
  is_vendored "$f" && continue
  first_party_css="$first_party_css $f"
  scanned=$((scanned + 1))
done <<EOF
$(git ls-files '*.css')
EOF

echo "== scope =="
printf 'first-party CSS scanned: %d\n' "$scanned"
for f in $first_party_css; do printf '  - %s\n' "$f"; done

if [ "$scanned" -eq 0 ]; then
  echo "::error::scanned 0 first-party CSS files — broken path or glob" >&2
  exit 1
fi

echo
echo "== 1. colour literals in first-party CSS =="
# Several notations, because a /#[0-9a-f]{6}/ rule alone under-counts: #fff,
# rgb(), hsl(), oklch() and the CSS named colours all express a colour too.
# The named list is the complete CSS colour keyword set on purpose — a partial
# list silently lets the ones it omits through, which is worse than no check.
named='aliceblue|antiquewhite|aqua|aquamarine|azure|beige|bisque|black|blanchedalmond|blue|blueviolet|brown|burlywood|cadetblue|chartreuse|chocolate|coral|cornflowerblue|cornsilk|crimson|cyan|darkblue|darkcyan|darkgoldenrod|darkgray|darkgreen|darkgrey|darkkhaki|darkmagenta|darkolivegreen|darkorange|darkorchid|darkred|darksalmon|darkseagreen|darkslateblue|darkslategray|darkslategrey|darkturquoise|darkviolet|deeppink|deepskyblue|dimgray|dimgrey|dodgerblue|firebrick|floralwhite|forestgreen|fuchsia|gainsboro|ghostwhite|gold|goldenrod|gray|green|greenyellow|grey|honeydew|hotpink|indianred|indigo|ivory|khaki|lavender|lavenderblush|lawngreen|lemonchiffon|lightblue|lightcoral|lightcyan|lightgoldenrodyellow|lightgray|lightgreen|lightgrey|lightpink|lightsalmon|lightseagreen|lightskyblue|lightslategray|lightslategrey|lightsteelblue|lightyellow|lime|limegreen|linen|magenta|maroon|mediumaquamarine|mediumblue|mediumorchid|mediumpurple|mediumseagreen|mediumslateblue|mediumspringgreen|mediumturquoise|mediumvioletred|midnightblue|mintcream|mistyrose|moccasin|navajowhite|navy|oldlace|olive|olivedrab|orange|orangered|orchid|palegoldenrod|palegreen|paleturquoise|palevioletred|papayawhip|peachpuff|peru|pink|plum|powderblue|purple|rebeccapurple|red|rosybrown|royalblue|saddlebrown|salmon|sandybrown|seagreen|seashell|sienna|silver|skyblue|slateblue|slategray|slategrey|snow|springgreen|steelblue|tan|teal|thistle|tomato|turquoise|violet|wheat|white|whitesmoke|yellow|yellowgreen'
found=0
for f in $first_party_css; do
  hits=$(awk -v F="$f" -v named="$named" '
    BEGIN { named_re = "(^|[^-a-zA-Z0-9#])(" named ")([^-a-zA-Z0-9]|$)" }
    { line = $0; sub(/\/\*.*\*\//, "", line) }
    line ~ /#[0-9a-fA-F][0-9a-fA-F][0-9a-fA-F]/ ||
    line ~ /(rgb|rgba|hsl|hsla|hwb|oklch|oklab|lab|lch|color-mix)[ \t]*\(/ ||
    line ~ named_re {
      printf "  %s:%d:%s\n", F, NR, $0
    }' "$f")
  if [ -n "$hits" ]; then found=1; printf '%s\n' "$hits"; fi
done
if [ "$found" -eq 0 ]; then
  pass "no colour literal outside the vendored token files"
else
  fail "colour literal found in first-party CSS — use a var(--…) token"
fi

echo
echo "== 2. breakpoints in absolute units =="
# Breakpoints, text sizes and spacing belong in relative units so they follow
# the reader's font-size preference. Borders, radii and hairlines in px are
# legitimate and are deliberately not matched here.
found=0
for f in $all_css; do
  hits=$(awk -v F="$f" '
    /@media/ && /(min|max)-(width|height)[ \t]*:[ \t]*[0-9.]+(px|pt|pc|cm|mm|in)[ \t]*\)/ {
      printf "  %s:%d:%s\n", F, NR, $0
    }' "$f")
  if [ -n "$hits" ]; then found=1; printf '%s\n' "$hits"; fi
done
if [ "$found" -eq 0 ]; then
  pass "every breakpoint uses a relative unit"
else
  fail "breakpoint declared in an absolute unit — use em"
fi

echo
echo "== 3. vendored DS artefacts vs manifest digests =="
manifest=crates/ui/assets/libre-ia/manifest.json
if [ ! -s "$manifest" ]; then
  fail "missing DS manifest: $manifest"
else
  manifest_digest() {
    awk -v want="$1" '
      index($0, "\"" want "\"") { hit = 1; next }
      hit && /sha256/ { gsub(/.*"sha256"[ \t]*:[ \t]*"/, ""); gsub(/".*/, ""); print; exit }
    ' "$manifest"
  }
  check_digest() {
    want=$(manifest_digest "$1"); file=$2
    if [ -z "$want" ]; then fail "no digest recorded for $1"; return; fi
    got=$(shasum -a 256 "$file" | cut -d' ' -f1)
    if [ "$want" = "$got" ]; then
      pass "digest $file"
    else
      fail "digest drift $file (manifest ${want}, actual ${got})"
    fi
  }
  for base in apps/web/assets crates/ui/assets; do
    check_digest "tokens/tokens.css" "$base/tokens.css"
    check_digest "tokens/themes.css" "$base/themes.css"
    check_digest "components/components.css" "$base/components.css"
  done
fi

echo
echo "== 4. the two vendored copies stay identical =="
for f in tokens.css themes.css components.css libre-ia-bridge.css fonts/fonts.css; do
  a="apps/web/assets/$f"; b="crates/ui/assets/$f"
  if [ ! -f "$a" ] || [ ! -f "$b" ]; then fail "missing copy for $f"; continue; fi
  if cmp -s "$a" "$b"; then pass "identical $f"; else fail "copies diverged: $f"; fi
done

echo
echo "== 5. literals that cannot be a CSS var =="
action=$(tok 'color-libre')
darkbg=$(tok 'color-theme-dark-background')
attr_value() { awk -v k="$1" '{ while (match($0, k "\"[^\"]*\"")) {
    s = substr($0, RSTART, RLENGTH); gsub(/.*"/, "", s)
    print substr(substr($0, RSTART, RLENGTH), index(substr($0, RSTART, RLENGTH), "\"") + 1, 100)
    exit } }' "$2"; }
meta_theme=$(awk '/name="theme-color"/ { n = index($0, "content=\""); if (n) { s = substr($0, n + 9); gsub(/".*/, "", s); print s; exit } }' apps/web/index.html)
mf_theme=$(awk '/"theme_color"/ { s = $0; sub(/.*"theme_color"[ \t]*:[ \t]*"/, "", s); sub(/".*/, "", s); print s; exit }' apps/web/assets/manifest.json)
mf_bg=$(awk '/"background_color"/ { s = $0; sub(/.*"background_color"[ \t]*:[ \t]*"/, "", s); sub(/".*/, "", s); print s; exit }' apps/web/assets/manifest.json)
assert_literal() {
  label=$1; want=$2; got=$3
  if [ -z "$got" ]; then fail "$label: value not found"; return; fi
  if [ "$(lower "$want")" = "$(lower "$got")" ]; then
    pass "$label mirrors its token ($want)"
  else
    fail "$label = $got but its token is $want"
  fi
}
assert_literal "<meta theme-color>" "$action" "$meta_theme"
assert_literal "manifest theme_color" "$action" "$mf_theme"
assert_literal "manifest background_color" "$darkbg" "$mf_bg"

echo
echo "== 6. WCAG contrast of the theme token pairs =="
# Recomputed from tokens.css rather than trusted from the vendored report.
# Thresholds: 4.5:1 body text, 3:1 large text and user-interface components.
while IFS='|' read -r label fgtok bgtok threshold; do
  [ -z "${label:-}" ] && continue
  fg=$(tok "$fgtok"); bg=$(tok "$bgtok")
  if [ -z "$fg" ] || [ -z "$bg" ]; then fail "contrast $label: token not found"; continue; fi
  result=$(awk -v a="$fg" -v b="$bg" -v t="$threshold" '
    function h2d(s,   i, c, n, d) {
      n = 0
      for (i = 1; i <= length(s); i++) {
        c = tolower(substr(s, i, 1)); d = index("0123456789abcdef", c) - 1
        n = n * 16 + d
      }
      return n
    }
    function chan(c) { c = c / 255; return (c <= 0.04045) ? c / 12.92 : ((c + 0.055) / 1.055) ^ 2.4 }
    function lum(x) { return 0.2126 * chan(h2d(substr(x, 2, 2))) \
                           + 0.7152 * chan(h2d(substr(x, 4, 2))) \
                           + 0.0722 * chan(h2d(substr(x, 6, 2))) }
    BEGIN {
      l1 = lum(a); l2 = lum(b)
      r = (l1 > l2) ? (l1 + 0.05) / (l2 + 0.05) : (l2 + 0.05) / (l1 + 0.05)
      printf "%.2f %s", r, (r + 0.005 >= t ? "PASS" : "FAIL")
    }')
  ratio=${result% *}; verdict=${result#* }
  if [ "$verdict" = "PASS" ]; then
    pass "contrast $label $fg on $bg = ${ratio}:1 (>= ${threshold}:1)"
  else
    fail "contrast $label $fg on $bg = ${ratio}:1 (< ${threshold}:1)"
  fi
done <<'PAIRS'
light-foreground|color-theme-light-foreground|color-theme-light-background|4.5
dark-foreground|color-theme-dark-foreground|color-theme-dark-background|4.5
light-muted|color-theme-light-muted|color-theme-light-background|4.5
dark-muted|color-theme-dark-muted|color-theme-dark-background|4.5
light-actionText|color-theme-light-actionText|color-theme-light-action|4.5
dark-actionText|color-theme-dark-actionText|color-theme-dark-action|4.5
light-focus|color-theme-light-focus|color-theme-light-background|3
dark-focus|color-theme-dark-focus|color-theme-dark-background|3
light-border|color-theme-light-border|color-theme-light-background|3
dark-border|color-theme-dark-border|color-theme-dark-background|3
PAIRS

echo
if [ "$status" -eq 0 ]; then
  printf 'design-system gate: PASS (%d first-party CSS files scanned)\n' "$scanned"
else
  printf 'design-system gate: FAIL (%d first-party CSS files scanned)\n' "$scanned" >&2
fi
exit "$status"
