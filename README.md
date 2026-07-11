<p align="center">
  <img src=".github/assets/repository-card.svg" alt="Libre AI Practices, represented by a reviewed learning path connecting scenarios, sources and feedback." width="100%">
</p>

# Libre AI Practices

Professional training for sourced, responsible AI practice—**not a general-knowledge quiz and not HR scoring**.

## Status

| | |
| --- | --- |
| Maturity | **Dojo** — runnable Rust API/PWA and governed corpus |
| Works today | corpus validation/audit, fixture sessions, local API and PWA proof |
| Not scale-ready | shared session runtime, production operations and broad platform convergence |
| Historical IDs | `rumble-ai-practices-*` remain current crate identifiers |

## Principles

The product trains confidentiality, verification, sourcing, bias awareness, GDPR, security and professional responsibility. It rejects named rankings, disciplinary use, unsourced corrections and automatically published generated questions.

Every published question should carry an explanation, sources, risks and a review date. Generated media requires explicit provenance and human bias review.

## Quickstart

```bash
cargo test --workspace
cargo run -p rumble-ai-practices-cli -- validate-corpus --content content/questions
cargo run -p rumble-ai-practices-cli -- audit-corpus \
  --content content/questions --media content/media --out reports/audit.json
cargo run -p rumble-ai-practices-cli -- run-session \
  --fixture fixtures/session-basic.json \
  --content content/questions --media content/media \
  --out reports/session-basic.json
cargo run -p rumble-ai-practices-cli -- serve --bind 127.0.0.1:3000
```

Then open <http://127.0.0.1:3000>. Health and PWA proofs include `/readyz`, `/manifest.webmanifest` and `/sw.js`.

## Database inspection pilot

[`db-security-manifest.json`](db-security-manifest.json) records the anonymous cohort storage classifications from ADR 0006. Run the local evidence check with no database connection or secret:

```bash
wrench-db-inspect run \
  --manifest db-security-manifest.json \
  --schema-dump crates/store/migrations/0001_anonymous_cohort.sql \
  --profile protected_branch \
  --report-json target/db-inspect/report.json
```

The current corpus passes with zero parser errors and zero unclassified tables. This is a reproducible local pilot; no global or protected-branch gate is enabled by this repository change.

## Architecture

The Rust workspace separates domain rules, governed content, audit, session state, storage ports, API, CLI and UI. Dioxus/PWA is the current proof path; native targets remain conditional on evidence.

Key documentation:

- [Product vision](docs/vision.md)
- [Architecture](docs/architecture.md)
- [Content governance](docs/content-governance.md)
- [Security and GDPR](docs/security-rgpd.md)
- [Human review gate](docs/local-review.md)
- [Testing strategy](docs/testing-strategy.md)

## Success criteria

An MVP requires a reviewed corpus, enforceable schemas, a complete private learning path and non-RH feedback. A runnable demo alone is not enough.

## Contributing

Read [`AGENTS.md`](AGENTS.md) and the governance documents before changing content, scoring or media.

## License

[MIT](LICENSE).
