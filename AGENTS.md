# AI Practices Canonical Agent Rules

## Purpose

Couche-1 product: training professionals in sourced, responsible AI practice
through verifiable exercises, without compliance quizzes or hidden HR
scoring.
Doctrine lives upstream: https://raw.githubusercontent.com/libre-ai/governance/main/docs/README.md

## Domain doctrine

- No HR scoring surface and no data transmission — feedback is advisory
  only, never autonomous grading (kill predicate in `project.v1.yaml`).
- Local-only operation; no telemetry, no employer-facing certification path.
- Bricks this repo depends on (`libre-ai/ui`, `libre-ai/web-platform`,
  `libre-ai/contracts`, `libre-ai/governance`) are consumed pinned by SHA,
  never redefined here.

## Commands

- `bun install` — install dependencies.
- `bun run test` — app test suite (`apps/practices`).
- `bun run lint` — Biome, strict.
- `bun run check` — full gate chain (toolchain, app tests, secret scan,
  personal-data boundary, no-transmission, lint); run before pushing.

## Working here

- Security > quality > performance > completeness, in that order on conflict.
- Check real state before editing: `git status --short` and `bun run check`.
- English for code, comments and this file; French stays the human
  conversation language elsewhere.
- Never commit a machine-local absolute path (e.g. `/Users/...`); use
  repo-relative paths or `~`.
