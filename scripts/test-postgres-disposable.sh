#!/usr/bin/env bash
set -euo pipefail

for command in initdb pg_ctl createdb psql cargo python3; do
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

# Apply the migrations to the base database before compiling. `crates/store`
# uses `sqlx::query!`, which verifies each statement against a live schema at
# COMPILE time when DATABASE_URL is set; the per-test databases created by
# `#[sqlx::test]` are migrated too late to help, since the crate must compile
# first. This mirrors the CI step exactly, so the two cannot drift.
for migration in crates/store/migrations/*.sql; do
  psql -h "$socket_dir" -U postgres -d ai_practices_root \
    -v ON_ERROR_STOP=1 -f "$migration" >/dev/null
done

# SQLX_OFFLINE is deliberately NOT set. It would disable the compile-time query
# verification above — turning the checking off rather than satisfying it.
#
# `--include-ignored` is what makes this script worth running: the `#[sqlx::test]`
# cases are marked `#[ignore]` so that a plain `cargo test` stays green on a
# machine with no database instead of failing on a missing DATABASE_URL. Without
# this flag the script would start up a PostgreSQL instance and then skip every
# test that needs it — passing while proving nothing.
DATABASE_URL="postgres://postgres@localhost/ai_practices_root?host=${encoded_socket}" \
cargo test --workspace --all-targets \
  --features rumble-ai-practices-web/ssr -- --include-ignored

echo "AI Practices disposable PostgreSQL suite: PASS"
