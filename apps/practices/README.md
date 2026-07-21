# Practices

Local activity outcome domain for Libre AI practices (WP-G3-P01).

## What is built (Increment 1)

**Pure domain module: `src/domain/activity-outcome.ts`**

A fail-closed, immutable TypeScript domain for managing learner activity outcomes that conform to the `activity-outcome.v1.schema.json` contract (docs/apps/practices.md, contracts/schemas/).

Capabilities:

- **createOutcome**: Validates and initializes an outcome from activity reference, session id, state, response digest, feedback rule ids, and timestamp.
- **advanceState**: Transitions outcome state (in-progress → completed/stopped), preventing invalid or backward transitions.
- **addFeedbackRules**: Appends feedback rule ids with validation, deduplication, and maximum count enforcement.
- **updateResponseDigest**: Replaces the response digest with validation.
- **exportOutcome**: Serializes the outcome to the canonical `activity-outcome.v1` export format for local persistence.

Validation:

- All inputs are validated against the contract (URN, identifier, SHA256, semantic version, timestamp patterns).
- Refusal codes match the refusal matrix: `practices.response_schema_invalid`, `practices.feedback_unsourced`, `practices.version_stale`.
- Deep immutability via `Object.freeze` — all objects and arrays are frozen against mutation.
- Every refusal path is tested; boundary values and schema conformance are verified.

## What is built (Increment 2)

**Local persistence port: `src/persistence/local-outcome-store.ts`**

The storage-agnostic core that lets an activity outcome survive a reload without ever
leaving the device — mirroring the boussole local-response-store. It is protected by
the `check-no-transmission` CI gate (the local-only invariant).

- **serializeActivityOutcome** — encodes an outcome to a device-local string (never a
  network payload).
- **deserializeActivityOutcome** — fail-closed: shape is guarded, then the outcome is
  reconstructed **through the domain** (`createOutcome`), so a corrupt or tampered
  envelope is refused (`practices.response_schema_invalid`) rather than rehydrated
  into an invalid state.
- **LocalOutcomeStore** — the async port (`save` / `load(localSessionId)` / `list` /
  `clear`), outcomes keyed by `localSessionId`; `load` surfaces corruption as a
  `corrupt` result and never throws.
- **createInMemoryOutcomeStore** — the in-memory adapter storing the encoded string,
  so it exercises the exact same decode path a persistent adapter would. The concrete
  IndexedDB adapter is a thin boundary deferred to a later increment.

Verified: encode/decode round-trip; malformed JSON, missing/wrong-typed fields, and
tampered content values (non-sha256 digest, non-URN id, invalid state) each surface
as `corrupt`; save/load/list/clear behave keyed and fail-closed.

## What is deferred

- The concrete IndexedDB adapter (the port + in-memory adapter ship in Increment 2)
- HTTP API and networking (practices.v1.yaml publishing flow)
- Review/approval workflow and RLS
- Deterministic feedback explanation rules (TypeScript; planned for Increment 2)
- Model-based grading (external, advisory, not part of domain)
- Rust/WASM scoring boundary (reserved; requires real activity invariant)
- UI/React components and accessibility qualification
- Offline mode, export/delete, and browser feature gates

## References

- docs/apps/practices.md — app specification, domain protocol, refusal matrix, evidence requirements
- contracts/schemas/activity-outcome.v1.schema.json — canonical outcome contract
- contracts/schemas/activity-definition.v1.schema.json — activity definition contract
- contracts/schemas/practice-progress-export.v1.schema.json — progress export contract
- Work Package: WP-G3-P01

## Testing

```bash
bun test apps/practices
```

Comprehensive test suite covers:

- Valid and malformed inputs
- All refusal paths
- Schema boundary values
- Immutability guarantees
- State transitions
- Feedback rule id management
- Round-trip serialization
