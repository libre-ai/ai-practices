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
  smoke, `scripts/check-design-system.sh`,
  `scripts/check-content-media-references.sh`, the dependency policy gate, the
  workspace test suite, and the web application build gate (pinned Dioxus CLI,
  `dx build --platform web`, plus assertions that the bundle is real). These
  live in this job rather than in workflows of their own so that they are
  required without a branch-settings change.
- `Database inspection gate` (`.github/workflows/db-inspection.yml`)

Correctness gates, all inside `Repository hygiene`:

- `cargo test --workspace --all-targets --features rumble-ai-practices-web/ssr
-- --include-ignored`, plus the same command with `--doc` because
  `--all-targets` excludes doctests. A pinned PostgreSQL service backs the ten
  `#[sqlx::test]` cases in `crates/store` and `crates/api`; those cases carry
  `#[ignore]` so that a bare `cargo test` stays green on a machine with no
  database, and `scripts/test-postgres-disposable.sh` passes `--include-ignored`
  to run them locally.
  **Not `--all-features`**: `apps/web` declares `web`, `desktop` and `mobile` as
  mutually exclusive render targets, so `--all-features` enables a combination
  the application does not support and pulls the Dioxus desktop stack (GTK /
  WebKit system libraries), which fails on a clean Linux runner while passing on
  a macOS workstation. Both selections run the same 68 tests plus 10 ignored;
  the render targets are covered by the `dx build --platform web` gate.
- `cargo deny --offline check bans licenses sources` blocks the merge. It is a
  pure function of `Cargo.lock` and `deny.toml`, so it cannot redden without a
  commit.
- `cargo deny check advisories` runs as a **signal only** and never blocks: it
  fetches the RustSec database over the network, so a new upstream advisory
  would otherwise turn `main` red with no change to the tree. A vulnerability
  disclosed in a dependency will therefore not stop a merge here — triage is a
  review duty. It is surfaced as a warning and in the job summary.

Still ungated: `cargo clippy --workspace --all-targets --all-features -- -D
warnings`, which is green today.

Known-open, and deliberately not decided by any test: 108 questions
(`bias-visual`, `deepfakes`, `profiles`) still carry `media:` references whose
files were withdrawn by `40c0e8e`, so `validate_content` reports 108 dangling
references and the content report is not "successful". Every one of those
questions is a draft, so nothing is publication-blocking. Whether the media are
restored, the exercises hidden, or the drafts left as they are is an editorial
decision for the owner; the test
`withdrawn_media_corpus_leaves_only_dangling_draft_references` records the
current state so that any of those outcomes turns it red on purpose.

Measured symptom, so the decision is taken on numbers rather than impressions: a
session draws 50 drills, of which **38 carry a dead `media:` reference and render
a broken image**; the 12 that render correctly are the `situations.yml` drills,
which carry no `media:` key at all. **No media file exists anywhere in the tree**
— `apps/web/assets/media/` is absent, so zero of the 108 references resolve.
There is no subset of "still working" references to preserve.

Why the references were not simply deleted: all 108 are `media_review`
interactions whose prompt, scenario _and_ feedback describe the specific image
(`"Cette image est…"`, `"Cet avatar généré est-il une représentation neutre ?"`,
feedback such as `"Elle réduit un homme africain au village, à la hutte et au
pagne"`). Stripping `media:` would leave 108 questions asking the learner to
judge an image that is not there, and would collapse `deepfakes` and `profiles`
into 27 and 31 identical unanswerable items. That is an editorial rewrite, not a
technical correction.

`scripts/check-content-media-references.sh` pins the set meanwhile: it resolves
every content media reference against the tree, logs how many it examined (and
fails if that is zero), distinguishes _conforme_ / _référence morte_ /
_incapable de chercher_, and compares the dead set against
`scripts/known-dead-media-references.tsv` as an exact match that fails in both
directions — a 109th dead reference fails, and so does a ledger entry that stops
being dead.

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
