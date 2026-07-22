import type { ActivityOutcome } from "../domain/activity-outcome";
import { createOutcome } from "../domain/activity-outcome";

// A deterministic activity fixture for the read/display view.
// Per the runtime boundary this is a contract fixture; no real activity dataset is fetched
// and no feedback scoring is computed (deferred). Built via createOutcome and throws if
// the domain refuses it, so the fixture is domain-valid.
export const SESSION_ID = "session_learning_001";

const outcomeResult = createOutcome(
  "urn:libre-ai:outcome:fixture-walking-skeleton",
  {
    activityId: "urn:libre-ai:activity:learning-foundations",
    activityVersion: "1.0.0",
  },
  SESSION_ID,
  "in-progress",
  "a".repeat(64), // responseDigest: 64-char hex
  [],
  new Date("2026-07-22T10:00:00Z").toISOString(),
);

if (!outcomeResult.ok) throw new Error(`practices.fixture_invalid: ${outcomeResult.refusal}`);
export const ACTIVITY_FIXTURE: ActivityOutcome = outcomeResult.value;
