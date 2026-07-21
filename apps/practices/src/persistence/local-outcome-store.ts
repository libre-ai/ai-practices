// Practices local persistence — the storage-agnostic core that lets an activity
// outcome survive a reload without ever leaving the device (docs/apps/practices.md:
// an exported outcome is for LOCAL persistence only; nothing is transmitted).
// Serialization is a plain local encoding; deserialization is fail-closed and
// reconstructs the outcome THROUGH the domain (`createOutcome`), so a corrupt or
// tampered local envelope can never rehydrate into an invalid state
// (practices.response_schema_invalid). The concrete IndexedDB adapter is a thin
// boundary deferred to a later increment; this module carries the port and an
// in-memory adapter that exercises the exact same encode/decode path. Mirrors the
// boussole local-response-store.

import {
  type ActivityOutcome,
  type ActivityRef,
  createOutcome,
  type Outcome,
  type RefusalCode,
  type SessionState,
} from "../domain/activity-outcome";

const REFUSAL: RefusalCode = "practices.response_schema_invalid";

/**
 * Encode an activity outcome to a local string. Device-local form only — never a
 * network payload. The fields carry everything needed to rehydrate the outcome.
 */
export function serializeActivityOutcome(outcome: ActivityOutcome): string {
  return JSON.stringify({
    id: outcome.id,
    activity: outcome.activity,
    localSessionId: outcome.localSessionId,
    state: outcome.state,
    responseDigest: outcome.responseDigest,
    feedbackRuleIds: outcome.feedbackRuleIds,
    recordedAt: outcome.recordedAt,
  });
}

function isObject(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function readActivityRef(value: unknown): ActivityRef | undefined {
  if (!isObject(value)) return undefined;
  const { activityId, activityVersion } = value;
  if (typeof activityId !== "string" || typeof activityVersion !== "string") return undefined;
  return { activityId, activityVersion };
}

function readStringArray(value: unknown): readonly string[] | undefined {
  if (!Array.isArray(value) || value.some((entry) => typeof entry !== "string")) return undefined;
  return value as string[];
}

/**
 * Rebuild an activity outcome from its stored string, fail-closed. Shape is guarded
 * here; the content invariants (URN id, sourced activity version, in-scale state,
 * sha256 digest, bounded unique feedback rule ids, valid timestamp) are re-enforced
 * by `createOutcome`, so any corruption is refused rather than silently loaded.
 */
export function deserializeActivityOutcome(raw: string): Outcome<ActivityOutcome> {
  let parsed: unknown;
  try {
    parsed = JSON.parse(raw);
  } catch {
    return { ok: false, refusal: REFUSAL };
  }
  if (!isObject(parsed)) return { ok: false, refusal: REFUSAL };

  const activity = readActivityRef(parsed.activity);
  const feedbackRuleIds = readStringArray(parsed.feedbackRuleIds);
  if (
    typeof parsed.id !== "string" ||
    activity === undefined ||
    typeof parsed.localSessionId !== "string" ||
    typeof parsed.state !== "string" ||
    typeof parsed.responseDigest !== "string" ||
    feedbackRuleIds === undefined ||
    typeof parsed.recordedAt !== "string"
  ) {
    return { ok: false, refusal: REFUSAL };
  }

  return createOutcome(
    parsed.id,
    activity,
    parsed.localSessionId,
    parsed.state as SessionState,
    parsed.responseDigest,
    feedbackRuleIds,
    parsed.recordedAt,
  );
}

export type LoadOutcomeResult =
  | { readonly status: "empty" }
  | { readonly status: "loaded"; readonly outcome: ActivityOutcome }
  | { readonly status: "corrupt"; readonly refusal: RefusalCode };

/**
 * The device-local persistence port. Async to match the real IndexedDB adapter.
 * Outcomes are keyed by `localSessionId`. `load` never throws on corruption: a
 * malformed or tampered store surfaces as a `corrupt` result, keeping the
 * fail-closed contract at the storage seam.
 */
export interface LocalOutcomeStore {
  save(outcome: ActivityOutcome): Promise<void>;
  load(localSessionId: string): Promise<LoadOutcomeResult>;
  list(): Promise<readonly string[]>;
  clear(): Promise<void>;
}

/**
 * In-memory adapter for tests and deterministic previews. It stores the encoded
 * string (not the object), so `load` runs the true decode path and rejects
 * injected corruption exactly as a persistent adapter would.
 */
export function createInMemoryOutcomeStore(): LocalOutcomeStore {
  const entries = new Map<string, string>();
  return {
    async save(outcome: ActivityOutcome): Promise<void> {
      entries.set(outcome.localSessionId, serializeActivityOutcome(outcome));
    },
    async load(localSessionId: string): Promise<LoadOutcomeResult> {
      const raw = entries.get(localSessionId);
      if (raw === undefined) return { status: "empty" };
      const outcome = deserializeActivityOutcome(raw);
      return outcome.ok
        ? { status: "loaded", outcome: outcome.value }
        : { status: "corrupt", refusal: outcome.refusal };
    },
    async list(): Promise<readonly string[]> {
      return [...entries.keys()];
    },
    async clear(): Promise<void> {
      entries.clear();
    },
  };
}
