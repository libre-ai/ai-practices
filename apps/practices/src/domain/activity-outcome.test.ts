import { describe, expect, test } from "bun:test";

import {
  type ActivityOutcome,
  type ActivityRef,
  addFeedbackRules,
  advanceState,
  createOutcome,
  exportOutcome,
  type Outcome,
  type RefusalCode,
  updateResponseDigest,
} from "./activity-outcome";

// Narrowing helpers so assertions type-check against the discriminated Outcome.
function refusalOf<T>(outcome: Outcome<T>): RefusalCode | undefined {
  return outcome.ok ? undefined : outcome.refusal;
}

function unwrap<T>(outcome: Outcome<T>): T {
  if (!outcome.ok) throw new Error(`unexpected refusal: ${outcome.refusal}`);
  return outcome.value;
}

const ACTIVITY: ActivityRef = {
  activityId: "urn:libre-ai:activity:ethical-ai-2026",
  activityVersion: "1.0.0",
};

const VALID_ID = "urn:libre-ai:outcome:session-abc123";
const VALID_SESSION_ID = "local-session-xyz789";
const VALID_DIGEST = "a".repeat(64);
const VALID_TIMESTAMP = "2026-07-21T10:30:00Z";

function created(): ActivityOutcome {
  const outcome = createOutcome(
    VALID_ID,
    ACTIVITY,
    VALID_SESSION_ID,
    "in-progress",
    VALID_DIGEST,
    [],
    VALID_TIMESTAMP,
  );
  if (!outcome.ok) throw new Error(`fixture create refused: ${outcome.refusal}`);
  return outcome.value;
}

describe("createOutcome", () => {
  test("creates an outcome with valid inputs", () => {
    const outcome = created();
    expect(outcome.id).toBe(VALID_ID);
    expect(outcome.activity).toEqual(ACTIVITY);
    expect(outcome.localSessionId).toBe(VALID_SESSION_ID);
    expect(outcome.state).toBe("in-progress");
    expect(outcome.responseDigest).toBe(VALID_DIGEST);
    expect(outcome.feedbackRuleIds).toEqual([]);
    expect(outcome.recordedAt).toBe(VALID_TIMESTAMP);
  });

  test("refuses id not a urn", () => {
    const outcome = createOutcome(
      "not-a-urn",
      ACTIVITY,
      VALID_SESSION_ID,
      "in-progress",
      VALID_DIGEST,
      [],
      VALID_TIMESTAMP,
    );
    expect(outcome.ok).toBe(false);
    expect(refusalOf(outcome)).toBe("practices.response_schema_invalid");
  });

  test("refuses id with invalid urn format", () => {
    const outcome = createOutcome(
      "urn:OTHER:outcome:x",
      ACTIVITY,
      VALID_SESSION_ID,
      "in-progress",
      VALID_DIGEST,
      [],
      VALID_TIMESTAMP,
    );
    expect(outcome.ok).toBe(false);
    expect(refusalOf(outcome)).toBe("practices.response_schema_invalid");
  });

  test("refuses activity id invalid", () => {
    const outcome = createOutcome(
      VALID_ID,
      { ...ACTIVITY, activityId: "invalid" },
      VALID_SESSION_ID,
      "in-progress",
      VALID_DIGEST,
      [],
      VALID_TIMESTAMP,
    );
    expect(outcome.ok).toBe(false);
    expect(refusalOf(outcome)).toBe("practices.response_schema_invalid");
  });

  test("refuses activity version invalid", () => {
    const outcome = createOutcome(
      VALID_ID,
      { ...ACTIVITY, activityVersion: "1.0" },
      VALID_SESSION_ID,
      "in-progress",
      VALID_DIGEST,
      [],
      VALID_TIMESTAMP,
    );
    expect(outcome.ok).toBe(false);
    expect(refusalOf(outcome)).toBe("practices.response_schema_invalid");
  });

  test("refuses session id uppercase", () => {
    const outcome = createOutcome(
      VALID_ID,
      ACTIVITY,
      "Local-session-xyz789",
      "in-progress",
      VALID_DIGEST,
      [],
      VALID_TIMESTAMP,
    );
    expect(outcome.ok).toBe(false);
    expect(refusalOf(outcome)).toBe("practices.response_schema_invalid");
  });

  test("refuses session id too short", () => {
    const outcome = createOutcome(
      VALID_ID,
      ACTIVITY,
      "ab",
      "in-progress",
      VALID_DIGEST,
      [],
      VALID_TIMESTAMP,
    );
    expect(outcome.ok).toBe(false);
    expect(refusalOf(outcome)).toBe("practices.response_schema_invalid");
  });

  test("refuses digest not sha256", () => {
    const outcome = createOutcome(
      VALID_ID,
      ACTIVITY,
      VALID_SESSION_ID,
      "in-progress",
      "a".repeat(63),
      [],
      VALID_TIMESTAMP,
    );
    expect(outcome.ok).toBe(false);
    expect(refusalOf(outcome)).toBe("practices.response_schema_invalid");
  });

  test("refuses digest has uppercase", () => {
    const outcome = createOutcome(
      VALID_ID,
      ACTIVITY,
      VALID_SESSION_ID,
      "in-progress",
      "A".repeat(64),
      [],
      VALID_TIMESTAMP,
    );
    expect(outcome.ok).toBe(false);
    expect(refusalOf(outcome)).toBe("practices.response_schema_invalid");
  });

  test("refuses timestamp invalid", () => {
    const outcome = createOutcome(
      VALID_ID,
      ACTIVITY,
      VALID_SESSION_ID,
      "in-progress",
      VALID_DIGEST,
      [],
      "not-a-timestamp",
    );
    expect(outcome.ok).toBe(false);
    expect(refusalOf(outcome)).toBe("practices.response_schema_invalid");
  });

  test("accepts timestamps in ISO 8601 formats", () => {
    const timestamps = [
      "2026-07-21T10:30:00Z",
      "2026-07-21T10:30:00.123Z",
      "2026-07-21T10:30:00+02:00",
      "2026-07-21T10:30:00.456-05:00",
    ];
    for (const ts of timestamps) {
      const outcome = createOutcome(
        VALID_ID,
        ACTIVITY,
        VALID_SESSION_ID,
        "in-progress",
        VALID_DIGEST,
        [],
        ts,
      );
      expect(outcome.ok).toBe(true);
    }
  });

  test("accepts all three valid session states", () => {
    for (const state of ["in-progress", "completed", "stopped"] as const) {
      const outcome = createOutcome(
        VALID_ID,
        ACTIVITY,
        VALID_SESSION_ID,
        state,
        VALID_DIGEST,
        [],
        VALID_TIMESTAMP,
      );
      expect(outcome.ok).toBe(true);
    }
  });

  test("refuses invalid session state", () => {
    const outcome = createOutcome(
      VALID_ID,
      ACTIVITY,
      VALID_SESSION_ID,
      "invalid-state" as never,
      VALID_DIGEST,
      [],
      VALID_TIMESTAMP,
    );
    expect(outcome.ok).toBe(false);
    expect(refusalOf(outcome)).toBe("practices.response_schema_invalid");
  });

  test("accepts feedback rule ids and validates them", () => {
    const outcome = createOutcome(
      VALID_ID,
      ACTIVITY,
      VALID_SESSION_ID,
      "in-progress",
      VALID_DIGEST,
      ["rule-one", "rule-two"],
      VALID_TIMESTAMP,
    );
    expect(outcome.ok).toBe(true);
    if (outcome.ok) {
      expect(outcome.value.feedbackRuleIds).toEqual(["rule-one", "rule-two"]);
    }
  });

  test("refuses more than 100 feedback rule ids", () => {
    const many = Array.from({ length: 101 }, (_v, i) => `rule-${i}`);
    const outcome = createOutcome(
      VALID_ID,
      ACTIVITY,
      VALID_SESSION_ID,
      "in-progress",
      VALID_DIGEST,
      many,
      VALID_TIMESTAMP,
    );
    expect(outcome.ok).toBe(false);
    expect(refusalOf(outcome)).toBe("practices.response_schema_invalid");
  });

  test("refuses malformed feedback rule id", () => {
    const outcome = createOutcome(
      VALID_ID,
      ACTIVITY,
      VALID_SESSION_ID,
      "in-progress",
      VALID_DIGEST,
      ["INVALID"],
      VALID_TIMESTAMP,
    );
    expect(outcome.ok).toBe(false);
    expect(refusalOf(outcome)).toBe("practices.response_schema_invalid");
  });
});

describe("advanceState", () => {
  test("transitions from in-progress to completed", () => {
    const outcome = created();
    const advanced = advanceState(outcome, "completed");
    expect(advanced.ok).toBe(true);
    if (!advanced.ok) return;
    expect(advanced.value.state).toBe("completed");
  });

  test("transitions from in-progress to stopped", () => {
    const outcome = created();
    const advanced = advanceState(outcome, "stopped");
    expect(advanced.ok).toBe(true);
    if (!advanced.ok) return;
    expect(advanced.value.state).toBe("stopped");
  });

  test("refuses transition back to in-progress", () => {
    let outcome = created();
    const completed = advanceState(outcome, "completed");
    expect(completed.ok).toBe(true);
    if (!completed.ok) return;
    outcome = completed.value;

    const backward = advanceState(outcome, "in-progress");
    expect(backward.ok).toBe(false);
    expect(refusalOf(backward)).toBe("practices.version_stale");
  });

  test("idempotent: completed → completed is ok", () => {
    let outcome = created();
    const completed = advanceState(outcome, "completed");
    expect(completed.ok).toBe(true);
    if (!completed.ok) return;
    outcome = completed.value;

    const idempotent = advanceState(outcome, "completed");
    expect(idempotent.ok).toBe(true);
    if (!idempotent.ok) return;
    expect(idempotent.value.state).toBe("completed");
  });

  test("refuses transition from completed to stopped", () => {
    let outcome = created();
    const completed = advanceState(outcome, "completed");
    expect(completed.ok).toBe(true);
    if (!completed.ok) return;
    outcome = completed.value;

    const stopped = advanceState(outcome, "stopped");
    expect(stopped.ok).toBe(false);
    expect(refusalOf(stopped)).toBe("practices.version_stale");
  });
});

describe("addFeedbackRules", () => {
  test("appends feedback rule ids", () => {
    const outcome = created();
    const added = addFeedbackRules(outcome, ["rule-one", "rule-two"]);
    expect(added.ok).toBe(true);
    if (!added.ok) return;
    expect(added.value.feedbackRuleIds).toContain("rule-one");
    expect(added.value.feedbackRuleIds).toContain("rule-two");
  });

  test("deduplicates when appending existing rule ids", () => {
    let outcome = created();
    const first = addFeedbackRules(outcome, ["rule-one"]);
    expect(first.ok).toBe(true);
    if (!first.ok) return;
    outcome = first.value;

    const second = addFeedbackRules(outcome, ["rule-one", "rule-two"]);
    expect(second.ok).toBe(true);
    if (!second.ok) return;
    expect(second.value.feedbackRuleIds.length).toBe(2);
    expect(second.value.feedbackRuleIds).toContain("rule-one");
    expect(second.value.feedbackRuleIds).toContain("rule-two");
  });

  test("refuses invalid feedback rule id", () => {
    const outcome = created();
    const added = addFeedbackRules(outcome, ["INVALID"]);
    expect(added.ok).toBe(false);
    expect(refusalOf(added)).toBe("practices.feedback_unsourced");
  });

  test("refuses exceeding maximum total feedback rule ids", () => {
    let outcome = created();
    const ninety = Array.from({ length: 90 }, (_v, i) => `rule-${i}`);
    const first = addFeedbackRules(outcome, ninety);
    expect(first.ok).toBe(true);
    if (!first.ok) return;
    outcome = first.value;

    const toomany = addFeedbackRules(outcome, [
      "rule-a",
      "rule-b",
      "rule-c",
      "rule-d",
      "rule-e",
      "rule-f",
      "rule-g",
      "rule-h",
      "rule-i",
      "rule-j",
      "rule-k",
    ]);
    expect(toomany.ok).toBe(false);
    expect(refusalOf(toomany)).toBe("practices.feedback_unsourced");
  });
});

describe("updateResponseDigest", () => {
  test("replaces the response digest", () => {
    const outcome = created();
    const newDigest = "b".repeat(64);
    const updated = updateResponseDigest(outcome, newDigest);
    expect(updated.ok).toBe(true);
    if (!updated.ok) return;
    expect(updated.value.responseDigest).toBe(newDigest);
  });

  test("refuses invalid digest", () => {
    const outcome = created();
    const updated = updateResponseDigest(outcome, "a".repeat(63));
    expect(updated.ok).toBe(false);
    expect(refusalOf(updated)).toBe("practices.response_schema_invalid");
  });

  test("refuses uppercase in digest", () => {
    const outcome = created();
    const updated = updateResponseDigest(outcome, "A".repeat(64));
    expect(updated.ok).toBe(false);
    expect(refusalOf(updated)).toBe("practices.response_schema_invalid");
  });
});

describe("exportOutcome", () => {
  test("serializes to activity-outcome.v1 document", () => {
    let outcome = created();
    const withRules = addFeedbackRules(outcome, ["rule-alpha", "rule-beta"]);
    expect(withRules.ok).toBe(true);
    if (!withRules.ok) return;
    outcome = withRules.value;

    const exported = exportOutcome(outcome);
    expect(exported.schemaVersion).toBe("libre-ai.activity-outcome.v1");
    expect(exported.id).toBe(VALID_ID);
    expect(exported.activityId).toBe(ACTIVITY.activityId);
    expect(exported.activityVersion).toBe(ACTIVITY.activityVersion);
    expect(exported.localSessionId).toBe(VALID_SESSION_ID);
    expect(exported.state).toBe("in-progress");
    expect(exported.responseDigest).toBe(VALID_DIGEST);
    expect(exported.feedbackRuleIds).toEqual(["rule-alpha", "rule-beta"]);
    expect(exported.recordedAt).toBe(VALID_TIMESTAMP);
  });

  test("exported document is fully frozen", () => {
    const outcome = created();
    const exported = exportOutcome(outcome);
    expect(Object.isFrozen(exported)).toBe(true);
    expect(Object.isFrozen(exported.feedbackRuleIds)).toBe(true);
    expect(() => {
      (exported as { state: string }).state = "stopped";
    }).toThrow();
  });
});

describe("immutability", () => {
  test("creating an outcome freezes the result", () => {
    const outcome = created();
    expect(Object.isFrozen(outcome)).toBe(true);
    expect(Object.isFrozen(outcome.activity)).toBe(true);
    expect(Object.isFrozen(outcome.feedbackRuleIds)).toBe(true);
  });

  test("advanceState does not mutate the original", () => {
    const outcome = created();
    const advanced = advanceState(outcome, "completed");
    expect(advanced.ok).toBe(true);
    if (!advanced.ok) return;
    expect(outcome.state).toBe("in-progress");
    expect(advanced.value.state).toBe("completed");
  });

  test("addFeedbackRules does not mutate the original", () => {
    const outcome = created();
    const added = addFeedbackRules(outcome, ["rule-one"]);
    expect(added.ok).toBe(true);
    if (!added.ok) return;
    expect(outcome.feedbackRuleIds).toEqual([]);
    expect(added.value.feedbackRuleIds).toContain("rule-one");
  });

  test("updateResponseDigest does not mutate the original", () => {
    const outcome = created();
    const newDigest = "b".repeat(64);
    const updated = updateResponseDigest(outcome, newDigest);
    expect(updated.ok).toBe(true);
    if (!updated.ok) return;
    expect(outcome.responseDigest).toBe(VALID_DIGEST);
    expect(updated.value.responseDigest).toBe(newDigest);
  });
});

describe("round-trip serialization", () => {
  test("exported outcome preserves all state", () => {
    let outcome = created();
    outcome = unwrap(advanceState(outcome, "completed"));
    outcome = unwrap(addFeedbackRules(outcome, ["rule-one", "rule-two"]));
    outcome = unwrap(updateResponseDigest(outcome, "c".repeat(64)));

    const exported = exportOutcome(outcome);
    expect(exported).toEqual({
      schemaVersion: "libre-ai.activity-outcome.v1",
      id: VALID_ID,
      activityId: ACTIVITY.activityId,
      activityVersion: ACTIVITY.activityVersion,
      localSessionId: VALID_SESSION_ID,
      state: "completed",
      responseDigest: "c".repeat(64),
      feedbackRuleIds: ["rule-one", "rule-two"],
      recordedAt: VALID_TIMESTAMP,
    });
  });
});
