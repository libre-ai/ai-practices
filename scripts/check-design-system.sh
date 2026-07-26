#!/usr/bin/env bash
# Design-system consumption gate.
#
# Asserts that the product stylesheets *consume* the vendored Libre IA Design
# System instead of restating it. Nine checks, all mechanical:
#
#   1. no colour literal in first-party CSS (hex / rgb / hsl / oklch / named)
#   2. no breakpoint in an absolute unit (px/pt/pc/cm/mm/in)
#   3. vendored DS artefacts match the SHA-256 recorded in their manifest
#   4. the two vendored copies (apps/web, crates/ui) stay byte-identical
#   5. colour literals that *cannot* be a CSS var (PWA manifest, <meta
#      theme-color>) still equal the token they mirror
#   6. WCAG contrast of the theme token pairs, recomputed from tokens.css
#   7. WCAG contrast of the *resolved product vocabulary*, in every theme the
#      product can render — including the default one
#   8. every focus ring paints the design system's focus token
#   9. the first-party default-theme block is complete
#
# Checks 7-9 exist because the light theme is now the product default: what used
# to be a state a user had to opt into is the nominal render, so every pair has
# to hold there, and the pairs that only appear in a state (focus, hover,
# selection, fills) count as much as the resting ones.
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
    # Comments are stripped with a state machine, not a single-line sub(): a
    # /* … */ block spanning several lines used to leave its prose exposed, so
    # any comment that merely *named* a colour ("white text on white") was
    # reported as a hard-coded literal. Only declarations are of interest here,
    # and a declaration is never inside a comment.
    {
      line = $0
      if (incomment) {
        p = index(line, "*/")
        if (p == 0) next
        line = substr(line, p + 2); incomment = 0
      }
      while ((s = index(line, "/*")) > 0) {
        rest = substr(line, s + 2); e = index(rest, "*/")
        if (e == 0) { line = substr(line, 1, s - 1); incomment = 1; break }
        line = substr(line, 1, s - 1) " " substr(rest, e + 2)
      }
    }
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
echo "== 7. WCAG contrast of the *resolved product vocabulary*, per theme =="
# Check 6 proves the design system's own token pairs are sound. That is not the
# same claim as "the product renders legibly", and the gap between the two is
# where a 1:1 light theme lived undetected: every pair in check 6 was green
# while the light theme painted #FFFFFF text on a #FFFFFF background.
#
# Two independent mechanisms produced it, and either alone is enough:
#
#   a) `--color-text: var(--color-foreground)` declared on :root only. A custom
#      property containing var() is substituted at computed-value time on the
#      element the declaration matches; only the result inherits. themes.css
#      switches --color-foreground on [data-theme="light"], which matches
#      div.app-root — never :root. So --color-text froze to the dark value and
#      inherited that literal into the light theme.
#   b) a background painted with a token that has no theme variant at all
#      (tokens.css carries a flat `--color-surface: #000000` alongside the
#      themed `--color-surface-active`).
#
# So this check resolves the vocabulary the way the browser does — following
# aliases, and freezing them at :root when the bridge is not declared on the
# themed element — and measures the pairs the stylesheet actually paints. The
# tokens are read out of styles.css rather than restated here, so the check
# follows the product instead of drifting from it.
#
# Four themes, not two. The light theme is now the product default, so the
# render that matters most is the one nobody chose:
#
#   default          no data-theme value, OS silent or set to light
#   default-os-dark  no data-theme value, OS set to dark
#   light / dark     the toggle's two explicit positions
#
# `default` and `default-os-dark` are declared in first-party CSS
# (libre-ia-bridge.css), not in the generated adapter, and are read from there.
#
# Two thresholds, not one. 4.5:1 is the floor for body text; 3:1 is the floor
# for user-interface components and their states (WCAG 1.4.11) — borders that
# delimit a control, focus rings, the mark that says "this one is selected".
# Those pairs never ran while the light theme was unreachable in practice.

# Is the product vocabulary declared on the element that carries the theme?
#
# The selector is located by the block that declares `--color-text`, not by
# taking the first block in the file: the bridge now opens with the
# default-theme block, and reading *that* selector reported "follows the themed
# element: no" — which silently re-froze every aliased token at :root and made
# whole themes resolve to the wrong colours. The vocabulary is what this
# question is about, so the vocabulary is what is looked up.
bridge=apps/web/assets/libre-ia-bridge.css
bridge_selector=$(awk '
  { line = $0
    if (inc) { p = index(line, "*/"); if (p == 0) next; line = substr(line, p + 2); inc = 0 }
    while ((s = index(line, "/*")) > 0) {
      rest = substr(line, s + 2); e = index(rest, "*/")
      if (e == 0) { line = substr(line, 1, s - 1); inc = 1; break }
      line = substr(line, 1, s - 1) " " substr(rest, e + 2)
    }
    if (line ~ /^[ \t]*$/) next
    if (index(line, "{") > 0) {
      head = substr(line, 1, index(line, "{") - 1); gsub(/^[ \t]+|[ \t]+$/, "", head)
      if (head ~ /^@media/) { pending = "" } else { sel = pending head; pending = "" }
      next
    }
    if (index(line, "}") > 0) { next }
    if (line ~ /,[ \t]*$/ && index(line, ";") == 0) {
      t = line; gsub(/^[ \t]+|[ \t]+$/, "", t); pending = pending t " "; next
    }
    if (line ~ /^[ \t]*--color-text:/) { print sel; exit }
  }' "$bridge")
case "$bridge_selector" in
  *'[data-theme]'*) bridge_themed=yes ;;
  *) bridge_themed=no ;;
esac
printf 'product vocabulary declared on: %s\n' "$(printf '%s' "$bridge_selector" | tr -s ' \n' ' ')"
printf 'follows the themed element: %s\n\n' "$bridge_themed"

# First `var(--…)` used by property $2 in the rule block of selector $1.
# The var() is looked for anywhere in the declaration's value, not only right
# after the colon: borders and outlines are written as shorthands
# (`border: 1px solid var(--color-border)`), and a rule that only matched
# `prop: var(…)` silently skipped every one of them — which is precisely where
# the interface-component pairs live.
painted() {
  awk -v sel="$1" -v prop="$2" '
    index($0, sel " {") == 1 { inb = 1; next }
    inb && index($0, "}") == 1 { inb = 0 }
    inb && match($0, "^[ \t]*" prop ":") {
      v = substr($0, RSTART + RLENGTH)
      # The *last* var() in the value, not the first: a border shorthand puts
      # the width first (`border: var(--border-default) solid var(--color-…)`)
      # and taking the first one returned a length where a colour was expected.
      found = ""
      while (match(v, /var\(--[a-zA-Z0-9-]+\)/)) {
        s = substr(v, RSTART, RLENGTH); sub(/^var\(/, "", s); sub(/\)$/, "", s)
        found = s; v = substr(v, RSTART + RLENGTH)
      }
      if (found != "") { print found; exit }
    }' apps/web/assets/styles.css
}

# Resolve a token to a #rrggbb the way it computes on the themed element.
#
# The bridge is parsed block by block, not line by line, because it now carries
# two kinds of declaration that must not be confused:
#   - the default-theme blocks (`html:root, [data-theme=""]`, plus its
#     `prefers-color-scheme: dark` variant) map *semantic* tokens, exactly like
#     themes.css does for the explicit themes;
#   - the vocabulary block (`:root, [data-theme]`) maps *product* names onto
#     those semantic tokens.
# Reading the first kind as if it were the second would alias --color-background
# to the light primitive for every theme, and the dark rows would silently
# report light values.
# A token may be asked for inside a verdict context, written `juste:--v`: the
# four `[data-verdict=…]` blocks rebind --v and --v-ink, and `juste` binds --v
# to the brand colour, which is the pair that behaves differently per theme.
resolve() {
  awk -v want="$1" -v theme="$2" -v themed="$bridge_themed" '
    function resolve(tok, th,   vctx, p) {
      p = index(tok, ":")
      if (p > 0) { vctx = substr(tok, 1, p - 1); tok = substr(tok, p + 1)
                   if ((vctx, tok) in vsem) return resolve(vsem[vctx, tok], th) }
      # A frozen vocabulary resolves at :root, which is the default theme.
      if (tok in alias) return resolve(alias[tok], (themed == "yes") ? th : "default")
      if ((th, tok) in sem) return prim[sem[th, tok]]
      # A semantic token the default block leaves out falls through to the
      # adapter'"'"'s :root, i.e. the dark variant. Check 9 forbids that, but the
      # measurement must stay truthful if it ever happens.
      if (th ~ /^default/ && ("dark", tok) in sem) return prim[sem["dark", tok]]
      if (tok in prim) return prim[tok]
      return ""
    }
    FILENAME ~ /tokens\.css$/ {
      if (match($0, /--[a-zA-Z0-9-]+:/)) {
        k = substr($0, RSTART, RLENGTH - 1); v = substr($0, RSTART + RLENGTH)
        gsub(/^[ \t]+/, "", v); sub(/[ \t]*;[ \t]*$/, "", v)
        if (v ~ /^#/) prim[k] = v
      }
      next
    }
    FILENAME ~ /themes\.css$/ {
      if ($0 ~ /\[data-theme="light"\]/) cur = "light"
      else if ($0 ~ /\[data-theme="dark"\]/ || $0 ~ /^:root/) cur = "dark"
      if (match($0, /--[a-zA-Z0-9-]+:[ \t]*var\(--[a-zA-Z0-9-]+\)/)) {
        s = substr($0, RSTART, RLENGTH)
        k = substr(s, 1, index(s, ":") - 1)
        t = s; sub(/.*var\(/, "", t); sub(/\).*/, "", t)
        sem[cur, k] = t
      }
      next
    }
    # --- the bridge, block-aware and comment-stripped ---
    {
      line = $0
      if (inc) { p = index(line, "*/"); if (p == 0) next; line = substr(line, p + 2); inc = 0 }
      while ((s = index(line, "/*")) > 0) {
        rest = substr(line, s + 2); e = index(rest, "*/")
        if (e == 0) { line = substr(line, 1, s - 1); inc = 1; break }
        line = substr(line, 1, s - 1) " " substr(rest, e + 2)
      }
      if (line ~ /^[ \t]*$/) next

      if (index(line, "{") > 0) {
        head = substr(line, 1, index(line, "{") - 1)
        gsub(/^[ \t]+|[ \t]+$/, "", head)
        if (head ~ /^@media/) {
          inmedia = (head ~ /prefers-color-scheme[ \t]*:[ \t]*dark/) ? "dark" : "other"
          pending = ""
        } else {
          sel = pending head; pending = ""
          if (sel ~ /\[data-theme=""\]/) block = (inmedia == "dark") ? "default-os-dark" : "default"
          else if (sel ~ /\[data-theme\]/) block = "vocabulary"
          else if (match(sel, /\[data-verdict="[a-z]+"\]/)) {
            block = "verdict"
            vname = substr(sel, RSTART, RLENGTH); sub(/.*="/, "", vname); sub(/".*/, "", vname)
          }
          else block = "other"
        }
        depth++
      }
      if (index(line, "}") > 0) { depth--; if (depth <= 0) { depth = 0; inmedia = ""; block = "" } ; next }
      # A selector list spread over several lines: `html:root,` is a selector,
      # not a declaration. Ending in "," with no ";" is the discriminator —
      # testing for the absence of ":" would misread `html:root,` as a value.
      if (line ~ /,[ \t]*$/ && index(line, ";") == 0) {
        trimmed = line; gsub(/^[ \t]+|[ \t]+$/, "", trimmed)
        pending = pending trimmed " "; next
      }
      if (line !~ /:/) next

      if (match(line, /--[a-zA-Z0-9-]+:[ \t]*var\(--[a-zA-Z0-9-]+\)/)) {
        s = substr(line, RSTART, RLENGTH)
        k = substr(s, 1, index(s, ":") - 1); gsub(/^[ \t]+/, "", k)
        t = s; sub(/.*var\(/, "", t); sub(/\).*/, "", t)
        if (block == "default" || block == "default-os-dark") sem[block, k] = t
        else if (block == "vocabulary" && !(k in alias)) alias[k] = t
        else if (block == "verdict") vsem[vname, k] = t
      }
    }
    END { print resolve(want, theme) }
  ' crates/ui/assets/tokens.css crates/ui/assets/themes.css "$bridge"
}

contrast() {
  awk -v a="$1" -v b="$2" -v t="$3" '
    function h2d(s,   i, n) {
      n = 0
      for (i = 1; i <= length(s); i++) n = n * 16 + index("0123456789abcdef", tolower(substr(s, i, 1))) - 1
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
    }'
}

# The per-pair loop is fed by a pipe, so it runs in a subshell and cannot set
# `status` for the parent. Verdicts are accumulated in files instead.
tmpfail=$(mktemp); tmpcount=$(mktemp)
trap 'rm -f "$tmpfail" "$tmpcount"' EXIT

page_fg=$(painted '.app-root' 'color')
page_bg=$(painted '.app-root' 'background')
soft_fg=$(painted '.app-mast .brandmark' 'color')
panel_bg=$(painted '.summary-panel' 'background')
notice_bg=$(painted '.notice' 'background')
cap_fg=$(painted '.theme-toggle' 'color')
cap_bg=$(painted '.theme-toggle' 'background')
# interface components and states — never exercised while the light theme was
# unreachable in practice.
console_bg=$(painted '.console' 'background')
choice_fg=$(painted '.choice' 'color')
choice_edge=$(painted '.choice' 'border')
keycap_edge=$(painted '.cap' 'border')
panel_edge=$(painted '.notice' 'border')
toggle_edge=$(painted '.theme-toggle' 'border')
focus_page=$(painted '.theme-toggle:focus-visible' 'outline')
focus_panel=$(painted '.choice:focus-visible' 'outline')
action_bg=$(painted '.validate-btn' 'background')
action_fg=$(painted '.validate-btn' 'color')
verdict_edge=$(painted '.summary-row' 'border-left')
tag_edge=$(painted '.verdict-tag' 'border')
hint_fg=$(painted '.calm-hint' 'color')

pairs="page text|$page_fg|$page_bg|4.5
secondary text on page|$soft_fg|$page_bg|4.5
summary panel text|$page_fg|$panel_bg|4.5
secondary text on panel|$soft_fg|$panel_bg|4.5
notice text|$page_fg|$notice_bg|4.5
control label on keycap|$cap_fg|$cap_bg|4.5
console text|$choice_fg|$console_bg|4.5
calm hint (secondary on console)|$hint_fg|$console_bg|4.5
action label on action fill|$action_fg|$action_bg|4.5
panel border on page|$panel_edge|$page_bg|3
control border on page|$toggle_edge|$page_bg|3
choice border on console|$choice_edge|$console_bg|3
keycap border on console|$keycap_edge|$console_bg|3
verdict tag border on panel|$tag_edge|$panel_bg|3
focus ring on page|$focus_page|$page_bg|3
focus ring on console|$focus_panel|$console_bg|3
verdict rule (partiel) on panel|partiel:$verdict_edge|$panel_bg|3
verdict rule (risque) on panel|risque:$verdict_edge|$panel_bg|3
verdict rule (faux) on panel|faux:$verdict_edge|$panel_bg|3"

# Pairs that do NOT meet their threshold and whose only remedy is a palette
# decision. They are not waived: each one is pinned to the ratio it measures
# today, per theme, and the run fails if a ratio moves — up or down — or if the
# pair stops resolving. A new failure therefore cannot hide behind this list,
# and the list itself is the arbitration, written down where CI prints it.
#
# All of them are one fact: the brand action colour (--color-action, shared by
# both themes) has 9.22:1 against the dark background and 2.28:1 against the
# light one. Making them pass means changing a design-system colour or minting
# an action-on-light stroke token — an owner decision, upstream in the design
# system, not something this repository can take.
exceptions="selection mark on console|$(painted '.choice.sel' 'border-color')|$console_bg|3
hover mark on page|$(painted '.theme-toggle:hover' 'border-color')|$page_bg|3
action fill on console|$action_bg|$console_bg|3
progress mark on its track|$(painted '.q-progress i.on' 'background')|$(painted '.q-progress i' 'background')|3
verdict rule (juste) on panel|juste:$verdict_edge|$panel_bg|3"

pinned="dark|selection mark on console|9.22
light|selection mark on console|2.28
default|selection mark on console|2.28
default-os-dark|selection mark on console|9.22
dark|hover mark on page|9.22
light|hover mark on page|2.28
default|hover mark on page|2.28
default-os-dark|hover mark on page|9.22
dark|action fill on console|9.22
light|action fill on console|2.28
default|action fill on console|2.28
default-os-dark|action fill on console|9.22
dark|progress mark on its track|2.12
light|progress mark on its track|2.12
default|progress mark on its track|2.12
default-os-dark|progress mark on its track|2.12
dark|verdict rule (juste) on panel|9.22
light|verdict rule (juste) on panel|2.28
default|verdict rule (juste) on panel|2.28
default-os-dark|verdict rule (juste) on panel|9.22"

# A token that failed to extract would silently drop its pair and make the
# check vacuously green — the same failure mode the zero-file guard exists for.
for tokvar in page_fg page_bg soft_fg panel_bg notice_bg cap_fg cap_bg \
  console_bg choice_fg choice_edge keycap_edge panel_edge toggle_edge \
  focus_page focus_panel action_bg action_fg verdict_edge tag_edge hint_fg; do
  eval "v=\$$tokvar"
  [ -n "$v" ] || fail "check 7: could not read the token for '$tokvar' from styles.css"
done

asserted=0
for theme in dark light default default-os-dark; do
  printf '  -- theme: %s --\n' "$theme"
  printf '%s\n' "$pairs" | while IFS='|' read -r label fgtok bgtok threshold; do
    [ -n "${label:-}" ] || continue
    fg=$(resolve "$fgtok" "$theme"); bg=$(resolve "$bgtok" "$theme")
    if [ -z "$fg" ] || [ -z "$bg" ]; then
      printf 'FAIL  %s/%s: unresolved (%s=%s, %s=%s)\n' "$theme" "$label" \
        "$fgtok" "${fg:-?}" "$bgtok" "${bg:-?}" >&2
      echo x >>"$tmpfail"; continue
    fi
    result=$(contrast "$fg" "$bg" "$threshold")
    ratio=${result% *}; verdict=${result#* }
    if [ "$verdict" = "PASS" ]; then
      printf 'ok    %s/%s %s on %s = %s:1 (>= %s:1)\n' "$theme" "$label" "$fg" "$bg" "$ratio" "$threshold"
    else
      printf 'FAIL  %s/%s %s (%s) on %s (%s) = %s:1 (< %s:1)\n' "$theme" "$label" \
        "$fgtok" "$fg" "$bgtok" "$bg" "$ratio" "$threshold" >&2
      echo x >>"$tmpfail"
    fi
    echo x >>"$tmpcount"
  done

  printf '%s\n' "$exceptions" | while IFS='|' read -r label fgtok bgtok threshold; do
    [ -n "${label:-}" ] || continue
    fg=$(resolve "$fgtok" "$theme"); bg=$(resolve "$bgtok" "$theme")
    if [ -z "$fg" ] || [ -z "$bg" ]; then
      printf 'FAIL  %s/%s: unresolved (%s=%s, %s=%s)\n' "$theme" "$label" \
        "$fgtok" "${fg:-?}" "$bgtok" "${bg:-?}" >&2
      echo x >>"$tmpfail"; continue
    fi
    result=$(contrast "$fg" "$bg" "$threshold")
    ratio=${result% *}
    want=$(printf '%s\n' "$pinned" | awk -F'|' -v t="$theme" -v l="$label" '$1 == t && $2 == l { print $3; exit }')
    if [ -z "$want" ]; then
      printf 'FAIL  %s/%s is in the arbitration list with no pinned ratio\n' "$theme" "$label" >&2
      echo x >>"$tmpfail"
    elif [ "$want" = "$ratio" ]; then
      printf 'pin   %s/%s %s on %s = %s:1 (threshold %s:1 — OWNER ARBITRATION, pinned)\n' \
        "$theme" "$label" "$fg" "$bg" "$ratio" "$threshold"
    else
      printf 'FAIL  %s/%s moved: pinned %s:1, now %s:1 (%s on %s)\n' \
        "$theme" "$label" "$want" "$ratio" "$fg" "$bg" >&2
      echo x >>"$tmpfail"
    fi
    echo x >>"$tmpcount"
  done
done
asserted=$(wc -l <"$tmpcount" | tr -d ' ')
if [ -s "$tmpfail" ]; then
  fail "resolved product vocabulary fails WCAG AA in at least one theme"
fi
# 4 themes x (19 gated + 5 pinned) — a table that silently shrinks is the same
# vacuous-green failure the zero-file guard exists for, so the count is checked
# against its expected value, not merely against zero.
expected_pairs=96
if [ "$asserted" -ne "$expected_pairs" ]; then
  echo "::error::check 7 asserted $asserted contrast pairs, expected $expected_pairs — extraction broke" >&2
  status=1
else
  printf '\ncontrast pairs asserted: %d (4 themes: dark, light, default, default-os-dark)\n' "$asserted"
fi

echo
echo "== 8. every focus ring paints the focus token =="
# A focus indicator is the one affordance a keyboard user cannot do without, and
# the only threshold that has no editorial escape hatch. The design system
# publishes a token for exactly this — `--color-focus`, ink on the light theme,
# white on the dark one, asserted at 3:1 by check 6 in both — but the product
# painted five of its six focus rings with the brand action colour instead
# (2.28:1 once the background is light) and the sixth with --color-text.
#
# Rather than re-measure each ring, this asserts the structural rule: an
# `outline` in first-party CSS paints `var(--color-focus)`, full stop. It is
# cheaper than a contrast pair and it holds for rings added later.
outlines=0
bad_outlines=0
for f in $first_party_css; do
  while IFS= read -r hit; do
    [ -n "$hit" ] || continue
    outlines=$((outlines + 1))
    case "$hit" in
      *'var(--color-focus)'*) ;;
      *) bad_outlines=$((bad_outlines + 1)); printf '  %s:%s\n' "$f" "$hit" >&2 ;;
    esac
  done <<EOF
$(awk '/^[ \t]*outline:/ && $0 !~ /none/ { gsub(/^[ \t]+/, ""); printf "%d:%s\n", NR, $0 }' "$f")
EOF
done
printf 'outline declarations scanned: %d\n' "$outlines"
if [ "$outlines" -eq 0 ]; then
  echo "::error::check 8 scanned 0 outline declarations — extraction broke" >&2
  status=1
elif [ "$bad_outlines" -eq 0 ]; then
  pass "every focus ring paints var(--color-focus)"
else
  fail "$bad_outlines focus ring(s) paint something other than var(--color-focus)"
fi

echo
echo "== 9. the first-party default-theme block is complete =="
# The default theme is declared in first-party CSS, so nothing regenerates it
# when the design system publishes a new semantic token. A token left out does
# not break loudly: it falls through to the adapter's :root, which is the *dark*
# variant, and the default theme quietly renders one colour from the wrong
# theme — the same class of defect as the 1:1 light theme, one token at a time.
#
# So: every semantic token themes.css publishes must be declared in both default
# blocks (the base one and the prefers-color-scheme: dark one), and each must
# point at the matching variant.
semantic=$(awk '
  /\[data-theme="light"\]/ { inlight = 1; next }
  inlight && /^}/ { exit }
  inlight && match($0, /--[a-zA-Z0-9-]+:/) { s = substr($0, RSTART, RLENGTH - 1); gsub(/^[ \t]+/, "", s); print s }
' crates/ui/assets/themes.css)
semantic_count=$(printf '%s\n' "$semantic" | grep -c . || true)
printf 'semantic tokens published by the adapter: %d\n' "$semantic_count"
if [ "$semantic_count" -eq 0 ]; then
  echo "::error::check 9 read 0 semantic tokens from themes.css — extraction broke" >&2
  status=1
else
  missing=0
  for tok in $semantic; do
    for variant in light dark; do
      case "$variant" in
        light) want="default" ;;
        dark) want="default-os-dark" ;;
      esac
      got=$(awk -v k="$tok" -v want="$want" '
        { line = $0
          if (inc) { p = index(line, "*/"); if (p == 0) next; line = substr(line, p + 2); inc = 0 }
          while ((s = index(line, "/*")) > 0) {
            rest = substr(line, s + 2); e = index(rest, "*/")
            if (e == 0) { line = substr(line, 1, s - 1); inc = 1; break }
            line = substr(line, 1, s - 1) " " substr(rest, e + 2)
          }
          if (line ~ /^[ \t]*$/) next
          if (index(line, "{") > 0) {
            head = substr(line, 1, index(line, "{") - 1); gsub(/^[ \t]+|[ \t]+$/, "", head)
            if (head ~ /^@media/) { inmedia = (head ~ /prefers-color-scheme[ \t]*:[ \t]*dark/) ? "dark" : "other"; pending = "" }
            else {
              sel = pending head; pending = ""
              block = (sel ~ /\[data-theme=""\]/) ? ((inmedia == "dark") ? "default-os-dark" : "default") : "other"
            }
            depth++
          }
          if (index(line, "}") > 0) { depth--; if (depth <= 0) { depth = 0; inmedia = ""; block = "" }; next }
          if (line ~ /,[ \t]*$/ && index(line, ";") == 0) { t = line; gsub(/^[ \t]+|[ \t]+$/, "", t); pending = pending t " "; next }
          if (block != want) next
          if (match(line, k ":[ \t]*var\\(--[a-zA-Z0-9-]+\\)")) {
            s = substr(line, RSTART, RLENGTH); sub(/.*var\(/, "", s); sub(/\).*/, "", s); print s; exit
          }
        }' "$bridge")
      # What the adapter itself maps this token to for that variant. Read from
      # themes.css rather than reconstructed from the token's name: the naming
      # is not mechanical (`--color-surface-active` maps to
      # `--color-theme-<variant>-surface`, `--color-action-text` to
      # `…-actionText`), and a rule that rebuilt the name would fail on exactly
      # those two while looking authoritative.
      expected=$(awk -v k="$tok" -v v="$variant" '
        index($0, "[data-theme=\"" v "\"]") > 0 { inb = 1; next }
        inb && /^}/ { exit }
        inb && match($0, k ":[ \t]*var\\(--[a-zA-Z0-9-]+\\)") {
          s = substr($0, RSTART, RLENGTH); sub(/.*var\(/, "", s); sub(/\).*/, "", s); print s; exit
        }' crates/ui/assets/themes.css)
      if [ -z "$expected" ]; then
        fail "check 9: the adapter does not map $tok for the $variant variant"
        missing=$((missing + 1))
      elif [ -z "$got" ]; then
        fail "check 9: $tok is not declared in the $want block — it would fall back to the dark variant"
        missing=$((missing + 1))
      elif [ "$got" != "$expected" ]; then
        fail "check 9: $tok in the $want block points at $got, the adapter's $variant variant is $expected"
        missing=$((missing + 1))
      fi
    done
  done
  if [ "$missing" -eq 0 ]; then
    pass "both default-theme blocks map all $semantic_count semantic tokens to the right variant"
  fi
fi

echo
if [ "$status" -eq 0 ]; then
  printf 'design-system gate: PASS (%d first-party CSS files scanned)\n' "$scanned"
else
  printf 'design-system gate: FAIL (%d first-party CSS files scanned)\n' "$scanned" >&2
fi
exit "$status"
