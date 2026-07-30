// Practices local outcome domain — the on-device activity result
// (docs/apps/practices.md §Domain protocol, contracts/schemas/
// activity-outcome.v1.schema.json). Pure and offline: an outcome binds a
// specific activity version, records the learner response, and captures which
// feedback rules applied. Transmission guarantee: this module does not
// transmit — it exposes no network path and imports nothing. An exported
// outcome is a plain serializable value for LOCAL persistence only; a caller
// must not upload it without explicit consent. The deterministic feedback
// application (explanation, feedback rule selection) is a candidate boundary
// for TypeScript. Model-based grading stays advisory and human-reviewed.

export interface ActivityRef {
  readonly activityId: string;
  readonly activityVersion: string;
}

export type SessionState = "in-progress" | "completed" | "stopped";

export interface ActivityOutcome {
  readonly id: string;
  readonly activity: ActivityRef;
  readonly localSessionId: string;
  readonly state: SessionState;
  readonly responseDigest: string;
  readonly feedbackRuleIds: readonly string[];
  readonly recordedAt: string;
}

export type RefusalCode =
  | "practices.activity_unpublished"
  | "practices.response_schema_invalid"
  | "practices.feedback_unsourced"
  | "practices.review_missing"
  | "practices.generated_content_unreviewed"
  | "practices.nominative_aggregate_forbidden"
  | "practices.version_stale";

export type Outcome<T> =
  | { readonly ok: true; readonly value: T }
  | { readonly ok: false; readonly refusal: RefusalCode };

// Patterns for validation (from common.v1.schema.json)
const URN = /^urn:libre-ai:[a-z][a-z0-9-]*:[A-Za-z0-9._~-]+$/;
const IDENTIFIER = /^[a-z][a-z0-9_-]{2,127}$/;
const SHA256 = /^[a-f0-9]{64}$/;
const SEMVER = /^[0-9]+\.[0-9]+\.[0-9]+$/;
// RFC 3339 time-secfrac is "." 1*DIGIT — any number of fractional digits, not exactly 3.
const TIMESTAMP = /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:\d{2})$/;

const MAX_FEEDBACK_RULE_IDS = 100;

function refuse<T>(refusal: RefusalCode): Outcome<T> {
  return { ok: false, refusal };
}

function validActivityRef(activity: ActivityRef): boolean {
  return URN.test(activity.activityId) && SEMVER.test(activity.activityVersion);
}

function validSessionId(id: string): boolean {
  return IDENTIFIER.test(id);
}

function validState(state: unknown): state is SessionState {
  return state === "in-progress" || state === "completed" || state === "stopped";
}

function validFeedbackRuleIds(ids: readonly string[]): boolean {
  if (ids.length > MAX_FEEDBACK_RULE_IDS) return false;
  // Check uniqueness
  const seen = new Set<string>();
  for (const id of ids) {
    if (!IDENTIFIER.test(id) || seen.has(id)) return false;
    seen.add(id);
  }
  return true;
}

function validTimestamp(ts: string): boolean {
  return TIMESTAMP.test(ts) && !Number.isNaN(new Date(ts).getTime());
}

/**
 * Create a new activity outcome with validated input. Fail-closed: a malformed
 * activity reference, invalid session id, state, response digest, feedback
 * rule ids, or timestamp is refused.
 */
export function createOutcome(
  id: string,
  activity: ActivityRef,
  localSessionId: string,
  state: SessionState,
  responseDigest: string,
  feedbackRuleIds: readonly string[],
  recordedAt: string,
): Outcome<ActivityOutcome> {
  if (!URN.test(id)) return refuse("practices.response_schema_invalid");
  if (!validActivityRef(activity)) return refuse("practices.response_schema_invalid");
  if (!validSessionId(localSessionId)) return refuse("practices.response_schema_invalid");
  if (!validState(state)) return refuse("practices.response_schema_invalid");
  if (!SHA256.test(responseDigest)) return refuse("practices.response_schema_invalid");
  if (!validFeedbackRuleIds(feedbackRuleIds)) return refuse("practices.response_schema_invalid");
  if (!validTimestamp(recordedAt)) return refuse("practices.response_schema_invalid");

  return {
    ok: true,
    value: Object.freeze({
      id,
      activity: Object.freeze({ ...activity }),
      localSessionId,
      state,
      responseDigest,
      feedbackRuleIds: Object.freeze([...feedbackRuleIds]),
      recordedAt,
    }),
  };
}

/**
 * Advance the outcome state from in-progress to completed or stopped. Fail-closed
 * if the new state is not a valid transition.
 */
export function advanceState(
  outcome: ActivityOutcome,
  newState: SessionState,
): Outcome<ActivityOutcome> {
  // Reject transitions to in-progress or invalid state
  if (newState === "in-progress" || !validState(newState)) {
    return refuse("practices.version_stale");
  }
  // Reject backwards transitions (completed/stopped → in-progress already caught above)
  if (
    (outcome.state === "completed" && newState !== "completed") ||
    (outcome.state === "stopped" && newState !== "stopped")
  ) {
    return refuse("practices.version_stale");
  }

  return {
    ok: true,
    value: Object.freeze({
      ...outcome,
      state: newState,
    }),
  };
}

/**
 * Append feedback rule ids to the outcome, rejecting duplicates and exceeding
 * the maximum total. Each rule id is validated.
 */
export function addFeedbackRules(
  outcome: ActivityOutcome,
  ruleIds: readonly string[],
): Outcome<ActivityOutcome> {
  // Validate each incoming rule id
  for (const id of ruleIds) {
    if (!IDENTIFIER.test(id)) return refuse("practices.feedback_unsourced");
  }

  // Merge and deduplicate
  const merged = new Set(outcome.feedbackRuleIds);
  for (const id of ruleIds) {
    merged.add(id);
  }

  // Check total count
  if (merged.size > MAX_FEEDBACK_RULE_IDS) return refuse("practices.feedback_unsourced");

  return {
    ok: true,
    value: Object.freeze({
      ...outcome,
      feedbackRuleIds: Object.freeze(Array.from(merged)),
    }),
  };
}

/**
 * Update the response digest, typically after recording a new learner response.
 * Digest must be a valid SHA256 hash.
 */
export function updateResponseDigest(
  outcome: ActivityOutcome,
  digest: string,
): Outcome<ActivityOutcome> {
  if (!SHA256.test(digest)) return refuse("practices.response_schema_invalid");

  return {
    ok: true,
    value: Object.freeze({
      ...outcome,
      responseDigest: digest,
    }),
  };
}

export interface ExportedActivityOutcome {
  readonly schemaVersion: "libre-ai.activity-outcome.v1";
  readonly id: string;
  readonly activityId: string;
  readonly activityVersion: string;
  readonly localSessionId: string;
  readonly state: SessionState;
  readonly responseDigest: string;
  readonly feedbackRuleIds: readonly string[];
  readonly recordedAt: string;
}

/**
 * Serialize the outcome to an activity-outcome.v1 document for local export.
 * The document preserves the full outcome state for local persistence. Export
 * is a local-only value — raw responses never accompany the export.
 */
export function exportOutcome(outcome: ActivityOutcome): ExportedActivityOutcome {
  return Object.freeze({
    schemaVersion: "libre-ai.activity-outcome.v1",
    id: outcome.id,
    activityId: outcome.activity.activityId,
    activityVersion: outcome.activity.activityVersion,
    localSessionId: outcome.localSessionId,
    state: outcome.state,
    responseDigest: outcome.responseDigest,
    feedbackRuleIds: Object.freeze([...outcome.feedbackRuleIds]),
    recordedAt: outcome.recordedAt,
  });
}
