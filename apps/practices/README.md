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

## What is built (Increment 3)

**IndexedDB adapter: `src/persistence/indexed-db-outcome-store.ts`**

The on-device implementation of `LocalOutcomeStore`, persisting the encoded outcome
string keyed by `localSessionId`. The `IDBFactory` is **injected** (a real
`indexedDB` in the browser, a fake in tests), so the adapter never reaches for an
ambient global and is testable off-browser. It mirrors the notebook adapter's
transaction discipline: completion handlers are attached synchronously with the
request, so a transaction can never auto-commit before completion is observed.

The fail-closed decode stays in the port — the adapter only reads the stored string
and hands it to `deserializeActivityOutcome`, so a corrupt or tampered record
surfaces as a `corrupt` result, never a rehydrated invalid outcome.

Verified against `fake-indexeddb` (real round-trip, not stubs): save/load by session,
re-save overwrites, list/clear, a seeded corrupt record loads as `corrupt`, and an
un-openable factory rejects.

## What is built (Increment 4 — client shell, PR #188)

The client-first walking skeleton: accessible SSR baseline (no-JS usable),
`hydrateDocument` hydration, Terminer/Arrêter state advance persisting to
IndexedDB and restored on reload, PWA shell (hash-versioned service worker,
shell-only fetch — the documented `check-no-transmission` allowlist entry),
three-engine e2e (no-js, pwa offline, round-trip + zero cross-origin assertion).

## What is built (Increment 5 — data-ownership controls)

An enhanced-only "Mes données" region (`src/ui/data-ownership.tsx`) surfaces the
export and delete behaviour the domain already carries:

- **Export** — `useActivity().exportData()` returns the non-identifying
  `activity-outcome.v1` document (digest only — raw responses never accompany the
  export); the component writes it to a local `practices-activite.json` via a
  `Blob` + object URL + anchor download. This touches **no network primitive**, so
  `check-no-transmission` stays green with no new allowlist entry; the helper
  lives inside the app so the guard scans it.
- **Delete** — `useActivity().deleteAll()` clears the stored outcome and resets to
  the fixture; it resolves only once the store has durably cleared, and the
  in-page two-step confirmation (never `window.confirm`) announces success via
  `aria-live` only after that point — a deletion claim must be durable.

The region is `lai-enhanced-only`, so the no-JS baseline is unchanged.
Three-engine e2e assert the download (filename + parsed document + **zero
cross-origin traffic**), the confirmed delete round-trip (reload resets to the
fixture), and the cancel path.

## What is deferred

- HTTP API and networking (practices.v1.yaml publishing flow)
- Review/approval workflow and RLS — publication is gated by the
  `activity-content-and-privacy-review` humanGate
- Deterministic feedback explanation rules (TypeScript)
- Model-based grading (external, advisory, not part of domain)
- Rust/WASM scoring boundary (reserved; requires real activity invariant)
- Response capture + hashing UI (the digest is fixture-provided today)
- Browser feature gates

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
