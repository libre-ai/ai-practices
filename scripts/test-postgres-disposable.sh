#!/usr/bin/env bash
set -euo pipefail

for command in initdb pg_ctl createdb cargo python3; do
  command -v "$command" >/dev/null || {
    echo "missing required command: $command" >&2
    exit 1
  }
done

root="$(mktemp -d "${TMPDIR:-/tmp}/ai-practices-postgres.XXXXXX")"
data="$root/data"
socket_dir="$root/socket"
mkdir -p "$socket_dir"
umask 077

cleanup() {
  pg_ctl -D "$data" -m fast stop >/dev/null 2>&1 || true
  rm -rf "$root"
}
trap cleanup EXIT INT TERM

initdb -D "$data" -A trust -U postgres --no-locale --encoding=UTF8 >/dev/null
pg_ctl -D "$data" \
  -o "-k $socket_dir -c listen_addresses=''" \
  -w start >/dev/null
createdb -h "$socket_dir" -U postgres ai_practices_root
encoded_socket="$(python3 -c \
  'import sys, urllib.parse; print(urllib.parse.quote(sys.argv[1], safe=""))' \
  "$socket_dir")"

repository="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repository"
# `--include-ignored` is what makes this script worth running: the `#[sqlx::test]`
# cases are marked `#[ignore]` so that a plain `cargo test` stays green on a
# machine with no database instead of failing on a missing DATABASE_URL. Without
# this flag the script would start up a PostgreSQL instance and then skip every
# test that needs it — passing while proving nothing.
SQLX_OFFLINE=true \
DATABASE_URL="postgres://postgres@localhost/ai_practices_root?host=${encoded_socket}" \
cargo test --workspace --all-targets --all-features -- --include-ignored

echo "AI Practices disposable PostgreSQL suite: PASS"
