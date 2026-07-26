#!/usr/bin/env bash
# Content media-reference gate.
#
# One question: does every `media:` reference in the content corpus point at a
# file that is actually in the tree? The web client turns `media[0]` into the
# literal URL `/assets/media/<file>` (`artifact_for`, apps/web/src/lib.rs), which
# the bundle serves from `apps/web/assets/media/`. A reference with no file
# behind it is a 404 in a learner's face, and nothing in this repository noticed
# it: 40c0e8e withdrew the visual corpus and the four required checks stayed
# green.
#
# Three verdicts, never two — "no dead reference found" and "could not look" must
# not print the same thing:
#
#   conforme            the reference resolves to a file in the tree
#   référence morte     the reference resolves to nothing
#   incapable de chercher   the file holds `media` in a shape this gate cannot
#                       read (inline flow sequence, or JSON-form content with no
#                       `jq` to read it). Fails, loudly. A gate that silently
#                       skips what it cannot parse is worse than no gate.
#
# Why a counter, and why zero is a failure: every "no dead reference" verdict
# here is an absence of matches, so a broken glob or a renamed directory would
# make the whole run vacuously green. The run fails if it examined nothing.
#
# Portability: extraction goes through `awk`, never `grep -P`. BSD grep (macOS)
# has no `-P`; a `grep -P` that fails prints its usage to stderr, exits non-zero
# and produces no stdout — indistinguishable from "no match", which is exactly
# how a check like this goes vacuously green. This bit this repository before,
# and `scripts/check-design-system.sh` carries the same note.
#
# THE LEDGER, and why this gate is green while 108 references are dead.
#
# `scripts/known-dead-media-references.tsv` records the dead references that are
# known and owner-pending. 40c0e8e withdrew 108 visuals deliberately — the
# owner's own work — and every one of the 108 questions still carrying a
# reference is a `media_review` drill whose prompt, scenario and feedback all
# describe the specific image. Removing the reference would leave the question
# asking "Cette image est…" with no image; restoring the files reverses a
# deliberate withdrawal. Both are editorial calls that belong to the owner, so
# this gate does not make either of them.
#
# What it does instead is pin the set. The comparison is an EXACT SET match and
# fails in BOTH directions:
#
#   a dead reference that is not in the ledger   -> new breakage, fails
#   a ledger entry that is no longer dead        -> the debt moved, fails
#
# So it cannot be used to accumulate more dead references, and it cannot go
# stale behind a fix: whichever way the owner decides, this turns red and the
# ledger has to be updated consciously. That is the same discipline as the test
# `withdrawn_media_corpus_leaves_only_dangling_draft_references`, which records
# the count for the same reason.
#
# This gate is about FILES ON DISK. The other half — whether a referenced medium
# has an approved media-review record — is `validate_content` in crates/content,
# which reports the same 108 as `fail` findings.
#
# Usage:
#   scripts/check-content-media-references.sh            # verify (CI)
#   scripts/check-content-media-references.sh --record   # rewrite the ledger
set -euo pipefail

cd "$(dirname "$0")/.."

CONTENT_ROOT="content"
MEDIA_ROOT="apps/web/assets/media"
LEDGER="scripts/known-dead-media-references.tsv"

record=0
if [ "${1:-}" = "--record" ]; then
  record=1
fi

status=0
fail() { printf 'FAIL  %s\n' "$1" >&2; status=1; }
pass() { printf 'ok    %s\n' "$1"; }

workdir="$(mktemp -d)"
trap 'rm -rf "$workdir"' EXIT

refs="$workdir/refs.tsv"       # REF <file> <question-id> <media>
unparsed="$workdir/unparsed"   # one line per shape the gate cannot read
: > "$refs"
: > "$unparsed"

# ---------------------------------------------------------------------------
# Extraction.
#
# Block form, which is what the corpus uses:
#
#     - id: q-biasvisual-001
#       media:
#         - rumble_asset_0244.webp
#
# `media: []` is an empty sequence and contributes nothing. Any other inline
# flow form (`media: [a, b]`) is reported as unreadable rather than skipped.
# ---------------------------------------------------------------------------
extract_yaml() {
  awk '
    FNR == 1 { qid = ""; inmedia = 0 }

    /^-[ \t]*id:[ \t]/ {
      qid = $0
      sub(/^-[ \t]*id:[ \t]*/, "", qid)
      sub(/[ \t]*$/, "", qid)
      gsub(/^"|"$/, "", qid)
      gsub(/^\047|\047$/, "", qid)
      inmedia = 0
      next
    }

    /^[ \t]*media:[ \t]*\[/ {
      body = $0
      sub(/^[ \t]*media:[ \t]*/, "", body)
      if (body !~ /^\[[ \t]*\][ \t]*$/) {
        printf "%s\tinline flow sequence: %s\n", FILENAME, $0 > "/dev/stderr"
      }
      inmedia = 0
      next
    }

    /^[ \t]*media:[ \t]*$/ { inmedia = 1; next }

    inmedia == 1 {
      if ($0 ~ /^[ \t]*-[ \t]*[^ \t]/) {
        val = $0
        sub(/^[ \t]*-[ \t]*/, "", val)
        sub(/[ \t]*$/, "", val)
        gsub(/^"|"$/, "", val)
        gsub(/^\047|\047$/, "", val)
        printf "REF\t%s\t%s\t%s\n", FILENAME, qid, val
        next
      }
      if ($0 ~ /^[ \t]*$/) { next }
      inmedia = 0
    }
  ' "$1" 2>>"$unparsed"
}

# JSON-form content (batch-001.yml is a JSON array in a .yml file — YAML is a
# superset, so the Rust loader reads it, and a line-oriented parser would walk
# straight past a `"media"` key in it).
extract_json() {
  local file="$1"
  if ! command -v jq >/dev/null 2>&1; then
    printf '%s\tJSON-form content file and jq is unavailable\n' "$file" >> "$unparsed"
    return 0
  fi
  jq -r --arg f "$file" '
    (if type == "array" then .[] else . end)
    | select(.media != null)
    | . as $o
    | ($o.media[]? | "REF\t\($f)\t\($o.id)\t\(.)")
  ' "$file" >> "$refs"
}

scanned=0
while IFS= read -r file; do
  scanned=$((scanned + 1))
  first="$(awk 'NF { sub(/^[ \t]*/, ""); print substr($0, 1, 1); exit }' "$file")"
  case "$first" in
    "[" | "{") extract_json "$file" ;;
    *) extract_yaml "$file" >> "$refs" ;;
  esac
done < <(find "$CONTENT_ROOT" -type f \( -name '*.yml' -o -name '*.yaml' -o -name '*.json' \) | sort)

if [ "$scanned" -eq 0 ]; then
  fail "scanned 0 content files under ${CONTENT_ROOT}/ — the glob is broken, not the corpus"
  exit 1
fi
pass "scanned ${scanned} content files under ${CONTENT_ROOT}/"

# ---------------------------------------------------------------------------
# Verdict 3 first: anything the gate could not read invalidates the rest.
# ---------------------------------------------------------------------------
if [ -s "$unparsed" ]; then
  while IFS= read -r line; do
    fail "incapable de chercher — ${line}"
  done < "$unparsed"
fi

examined=0
resolved=0
dead="$workdir/dead.tsv"
: > "$dead"

while IFS=$'\t' read -r _tag file qid media; do
  [ -n "${media:-}" ] || continue
  examined=$((examined + 1))
  if [ -f "${MEDIA_ROOT}/${media}" ]; then
    resolved=$((resolved + 1))
  else
    printf '%s\t%s\t%s\n' "$qid" "$media" "$file" >> "$dead"
  fi
done < "$refs"

if [ "$examined" -eq 0 ]; then
  fail "examined 0 media references — extraction is broken, not the corpus"
  exit 1
fi
pass "examined ${examined} media references against ${MEDIA_ROOT}/"
pass "${resolved} resolve to a file present in the tree"

dead_count="$(wc -l < "$dead" | tr -d ' ')"
sort -o "$dead" "$dead"

if [ "$record" -eq 1 ]; then
  {
    echo "# Known dead content media references — owner-pending, see"
    echo "# scripts/check-content-media-references.sh for why this file exists."
    echo "#"
    echo "# Regenerate: scripts/check-content-media-references.sh --record"
    echo "# Columns: question-id <TAB> media-file <TAB> declaring-file"
    cat "$dead"
  } > "$LEDGER"
  pass "recorded ${dead_count} dead references in ${LEDGER}"
  exit 0
fi

if [ ! -f "$LEDGER" ]; then
  fail "ledger ${LEDGER} is missing; regenerate it with --record"
  exit 1
fi

known="$workdir/known.tsv"
grep -v '^#' "$LEDGER" | grep -v '^[[:space:]]*$' | sort > "$known" || true
known_count="$(wc -l < "$known" | tr -d ' ')"

# Exact set match, both directions.
new_dead="$(comm -23 "$dead" "$known" || true)"
fixed="$(comm -13 "$dead" "$known" || true)"

if [ -n "$new_dead" ]; then
  while IFS=$'\t' read -r qid media file; do
    [ -n "${qid:-}" ] || continue
    fail "référence morte — question \`${qid}\` references \`${media}\` (${file}), absent from ${MEDIA_ROOT}/"
  done <<< "$new_dead"
  fail "the references above are not in ${LEDGER}: a content reference must point at a file in the tree"
fi

if [ -n "$fixed" ]; then
  while IFS=$'\t' read -r qid media file; do
    [ -n "${qid:-}" ] || continue
    fail "stale ledger entry — question \`${qid}\` / \`${media}\` (${file}) is no longer dead"
  done <<< "$fixed"
  fail "the debt moved: rerun scripts/check-content-media-references.sh --record and review the diff"
fi

if [ -z "$new_dead" ] && [ -z "$fixed" ]; then
  if [ "$dead_count" -eq 0 ]; then
    pass "no dead media reference"
  else
    pass "${dead_count} dead references, all known and owner-pending (${known_count} in the ledger)"
    printf 'note  these are the withdrawn visual corpus of 40c0e8e; the editorial decision is the owner'"'"'s\n'
  fi
fi

exit "$status"
