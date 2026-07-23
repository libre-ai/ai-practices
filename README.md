**English** · [Français](README.fr.md)

> [!NOTE]
> **Reserved · future home of AI Practices** — rebuilt in the canonical base repository [`libre-ai/libre-ai`](https://github.com/libre-ai/libre-ai) ([multi-repo topology, ADR-0008](https://github.com/libre-ai/libre-ai/blob/main/docs/adr/0008-multi-repo-target-topology-and-brand.md)).
> This repository will reopen as the real product repository when the owner activates it, consuming the base as a versioned dependency. The foundations described below are **being built now** — with links to the code that already exists.

# AI Practices

**Professional training for sourced and responsible AI practice.** Help learners exercise bounded, explicit judgment in realistic AI scenarios. Learners own their progress offline; reviewers approve activity versions; publishers release curated content — never automatedly ranked, never without human approval, never as a compliance checklist.

The canonical use: _a professional team completes a scenario involving model choice, data sensitivity and jurisdictional constraints, receives sourced non-punitive feedback, and verifies their reasoning against documented sources and known limitations._

## Why it's different

- **Owned offline, reviewed once.** Learner progress is local and portable; activity versions are immutable after approval. No continuous surveillance, no server-side learner profile.
- **Sourced feedback, not model opinion.** Every hint references an approved source rule (policy, documentation, case study) — never uses unverified model inference as an answer key.
- **Bounded scenarios, not open-ended chat.** Activities are schema-constrained; learner responses must satisfy the defined interface. Ambiguity is documented, not hidden.
- **Non-punitive, audit-focused.** The outcome is _in-progress_, _completed_, or _stopped_ — not a pass/fail score. Feedback preserves the learner's response and offers retry, never fabricates success.
- **Deterministic, degraded gracefully.** If a feedback provider (model, external service) is unavailable, deterministic hints remain available; unsourced feedback is marked explicitly, never silently upgraded to success.

## Status — spec-published, product skeleton complete

AI Practices is being built from locked contracts and a domain model. It is **not released yet**; local offline practice is working and proven, and the review/publish surface is next:

| Foundation                                                | State    | Evidence                                                                                                                                                            |
| --------------------------------------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Activity Definition & Outcome** — domain model          | ✅ built | Schema + TypeScript domain, contract fixtures ([#163](https://github.com/libre-ai/libre-ai/pull/163))                                                               |
| **Local outcome persistence** — in-memory & IndexedDB     | ✅ built | Adapters with unit tests, offline-first architecture ([#183](https://github.com/libre-ai/libre-ai/pull/183), [#184](https://github.com/libre-ai/libre-ai/pull/184)) |
| **Data ownership & durable delete** — learner controls    | ✅ built | Export/reset, no residue in server-side index ([#190](https://github.com/libre-ai/libre-ai/pull/190))                                                               |
| **Client-first UI** — activity and progress               | ✅ built | Walking skeleton, keyboard-accessible, React 19 PWA ([#188](https://github.com/libre-ai/libre-ai/pull/188))                                                         |
| **Review & publish API** — activity versioning & approval | ⏳ next  | Biscuit-gated, immutable versions, RLS for tenant isolation                                                                                                         |
| **Deterministic feedback engine** — rule-based scoring    | ⏳ next  | TypeScript validator, schema-conformant responses, accessibility proof                                                                                              |

This repository is a public reserved home, intentionally without product code until activation (wave 4). **Benchmark target:** professional learning platforms (e.g. DataCamp, Coursera) — differentiated by offline-first ownership, human-reviewed content, and explicit sourcing rather than algorithmic recommendation.

## How it works

1. **Install offline** — learner downloads an approved activity version (schema + instructions + scenarios), stored locally.
2. **Practice with feedback** — learner submits a bounded response; engine validates the schema and applies deterministic feedback rules linked to documented sources. Failed rules show which rule broke and why. Retry is always available.
3. **Export or reset** — learner exports a portable progress bundle (review-ready evidence) or deletes all local data without server involvement.
4. **Publish curated content** — reviewer validates source, licence, accessibility and safety evidence; publisher promotes the immutable reviewed version to the activity index.

## Architecture — built from interoperable bricks

AI Practices is a product assembled from independently versioned bricks; each is usable and testable on its own, and the product is their composition (the multi-repo target of [ADR-0008](https://github.com/libre-ai/libre-ai/blob/main/docs/adr/0008-multi-repo-target-topology-and-brand.md)).

| Brick                                             | Role                                    | Interface it exposes / consumes                                                                             |
| ------------------------------------------------- | --------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| **Activity Definition v1 & Outcome v1** (schemas) | Bounded contracts for activities        | `activity-definition.v1.schema.json`, `activity-outcome.v1.schema.json`, contract fixtures                  |
| **`@libre-ai/ui`** (React 19)                     | Keyboard-accessible UI components       | Activity app, data-ownership flows, composable with web-platform                                            |
| **`@libre-ai/web-platform`**                      | SSR / Bun BFF foundation                | Request handler, server-rendered document, environment for the local-first client                           |
| **Local outcome persistence** (TypeScript)        | Offline-first storage adapters          | In-memory adapter for tests; IndexedDB adapter for browser persistence; port-based (testable independently) |
| **Progress Export v1** (schema)                   | Portable evidence format                | `practice-progress-export.v1.schema.json` — learner data without server identity, reviewable                |
| **OpenAPI: practices.v1.yaml** (reserved)         | Review & publish surface (not yet live) | Activity versioning, approval gates, publisher isolation                                                    |

The local-first client holds no server identity; it exchanges only immutable activity versions and sourced feedback rules. Any consumer that speaks the activity contracts can render and practice; the host environment (server) is responsible for publication gates.

## Where the work happens

All active development is in the base repository, under:

- `apps/practices` — the product host (offline PWA, server coordination)
- `packages/contracts/schemas` — the locked activity definition, outcome, and progress export schemas
- `contracts/openapi/practices.v1.yaml` — the review/publish API surface
- [`docs/apps/practices.md`](https://github.com/libre-ai/libre-ai/blob/main/docs/apps/practices.md) — the full product brief

To follow progress or contribute, open issues and pull requests in [`libre-ai/libre-ai`](https://github.com/libre-ai/libre-ai). This repository stays reserved until activation.

## License

EUPL-1.2.
