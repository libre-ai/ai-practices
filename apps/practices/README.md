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

## What is deferred

- Session/activity binding and local persistence (IndexedDB, LocalStorage)
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
