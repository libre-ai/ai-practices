# AGENTS.md

Canonical agent-context surface for this repository. `CLAUDE.md` is a minimal adapter that imports this file.

## Purpose

AI Practices trains professional reflexes in the use of AI. The core risk is editorial rather than technical: an application that is correct in every engineering respect but carries a biased or false corpus is a failure.

## Scope / Non-scope

- **Reserved home.** This repository is the public reserved home of AI Practices. The product is being rebuilt in the canonical base repository [`libre-ai/libre-ai`](https://github.com/libre-ai/libre-ai) (multi-repo topology, [ADR-0008](https://github.com/libre-ai/libre-ai/blob/main/docs/adr/0008-multi-repo-target-topology-and-brand.md)); it reopens as the real product repository when the owner activates it (wave 4).
- The implementation carried here (Rust workspace `crates/{domain,content,audit,session,store,api,cli,ui}` and `apps/web`, content corpus, schemas, audit tooling) is **frozen for reference**.
- Non-scope: new product development in this repository until activation.

## Editorial doctrine (frozen for reference)

These rules governed the product built here. They are recorded because they explain why the corpus has the shape it has — not as instructions to build against today.

- **Content-as-data** — questions are versioned files, never text hard-coded in the UI.
- **Human validation is mandatory** — no AI-generated question or correction is published without review.
- **Nuance over binary** — contextualised scenarios rather than simplistic true/false.
- **Pedagogical feedback** — explain the risk and the recommended action, not just right/wrong.
- **No implicit HR use** — no named leaderboard, no humiliating global score, no hidden profiling.
- **AI media under control** — no AI-generated human face outside a justified and audited case.
- **Sovereignty** — self-hostable, local build and localhost review by default; no cloud platform in the nominal flow.
- **Evidence over promise** — every increment leaves a reproducible verification command.

## Commands

Verified against `Cargo.toml`, `justfile` and `scripts/`:

- Rust workspace: `cargo test --workspace` (members: `crates/domain`, `crates/content`, `crates/audit`, `crates/session`, `crates/store`, `crates/api`, `crates/cli`, `crates/ui`, `apps/web`).
- Lint: `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- Dependency policy: `cargo deny check` (`deny.toml`).
- Local serve: `just serve-local`.
- Build: `scripts/build-all.sh`. Disposable Postgres for tests: `scripts/test-postgres-disposable.sh`.

## CI gates

The four required checks, by the name that appears in `required_status_checks`:

- `No private identifiers or machine-local paths` (`.github/workflows/context-hygiene.yml`)
- `REUSE compliance` (`.github/workflows/licensing.yml`)
- `Repository hygiene` (`.github/workflows/hygiene.yml`) — policy files, secret
  smoke, `scripts/check-design-system.sh`, and the web application build gate
  (pinned Dioxus CLI, `dx build --platform web`, plus assertions that the bundle
  is real). The build lives in this job rather than a workflow of its own so
  that it is required without a branch-settings change.
- `Database inspection gate` (`.github/workflows/db-inspection.yml`)

Not covered by any gate: `cargo test --workspace`, which is red on `main` today
— `crates/content` still asserts 109 media-review records that were withdrawn
with the visual corpus, and the `crates/api` tests need a `DATABASE_URL`
(`scripts/test-postgres-disposable.sh`). `cargo clippy` is green but ungated.

## Links

- [README](README.md) · [Français](README.fr.md)
- [docs/README.md](docs/README.md) — documentation index
- [docs/architecture.md](docs/architecture.md), [docs/api-contracts.md](docs/api-contracts.md)
- [docs/adrs/](docs/adrs/) — ten accepted ADRs, including `0003-content-governance-and-no-rh-scoring`, `0004-media-ai-bias-review` and `0009-cohorte-per-item-k-anon`
- [SECURITY.md](SECURITY.md)

## Modification rules

- Read the relevant docs and files before editing.
- Prefer small, reversible changes.
- Record any structural decision in `docs/adrs/`.
- Never add a major dependency without a licence, sovereignty, maintenance and rejected-alternatives justification.
- Never introduce personal-data collection without updating the security and GDPR documentation.
- Never publish pedagogical content without review metadata.
- Never explain away a media bias defensively: when in doubt, mark it `blocked`.
