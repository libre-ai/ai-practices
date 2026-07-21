import { describe, expect, test } from "bun:test";
import { type ActivityOutcome, createOutcome } from "../domain/activity-outcome";
import {
  createInMemoryOutcomeStore,
  deserializeActivityOutcome,
  serializeActivityOutcome,
} from "./local-outcome-store";

const NOW = "2030-01-01T00:00:00Z";

function outcome(localSessionId: string): ActivityOutcome {
  const created = createOutcome(
    "urn:libre-ai:activity-outcome:o1",
    { activityId: "urn:libre-ai:activity:a1", activityVersion: "1.0.0" },
    localSessionId,
    "in-progress",
    "a".repeat(64),
    ["rule-a"],
    NOW,
  );
  if (!created.ok) throw new Error(`fixture outcome refused: ${created.refusal}`);
  return created.value;
}

describe("serialize/deserialize round-trip", () => {
  test("a valid outcome survives the encode/decode path unchanged", () => {
    const original = outcome("session-one");
    const decoded = deserializeActivityOutcome(serializeActivityOutcome(original));
    expect(decoded.ok).toBe(true);
    if (!decoded.ok) return;
    expect(decoded.value).toEqual(original);
  });
});

describe("deserializeActivityOutcome — fail-closed", () => {
  test("malformed JSON is corrupt", () => {
    expect(deserializeActivityOutcome("{not json")).toEqual({
      ok: false,
      refusal: "practices.response_schema_invalid",
    });
  });

  test("a missing or wrong-typed field is corrupt", () => {
    const base = JSON.parse(serializeActivityOutcome(outcome("session-one")));
    for (const mutate of [
      (o: Record<string, unknown>) => delete o.id,
      (o: Record<string, unknown>) => {
        o.activity = "not-an-object";
      },
      (o: Record<string, unknown>) => {
        o.feedbackRuleIds = "not-an-array";
      },
      (o: Record<string, unknown>) => {
        o.recordedAt = 42;
      },
    ]) {
      const copy = structuredClone(base);
      mutate(copy);
      expect(deserializeActivityOutcome(JSON.stringify(copy)).ok).toBe(false);
    }
  });

  test("a tampered content value the domain rejects is corrupt (not silently loaded)", () => {
    const base = JSON.parse(serializeActivityOutcome(outcome("session-one")));
    // A non-sha256 digest and a non-URN id each fail the domain's content checks.
    expect(deserializeActivityOutcome(JSON.stringify({ ...base, responseDigest: "zzz" }))).toEqual({
      ok: false,
      refusal: "practices.response_schema_invalid",
    });
    expect(deserializeActivityOutcome(JSON.stringify({ ...base, id: "not-a-urn" }))).toEqual({
      ok: false,
      refusal: "practices.response_schema_invalid",
    });
    expect(deserializeActivityOutcome(JSON.stringify({ ...base, state: "paused" })).ok).toBe(false);
  });
});

describe("in-memory outcome store", () => {
  test("saves and loads an outcome by localSessionId, decoding through the domain", async () => {
    const store = createInMemoryOutcomeStore();
    const original = outcome("session-one");
    await store.save(original);
    const result = await store.load("session-one");
    expect(result.status).toBe("loaded");
    if (result.status !== "loaded") return;
    expect(result.outcome).toEqual(original);
  });

  test("an unknown localSessionId is empty", async () => {
    const store = createInMemoryOutcomeStore();
    expect(await store.load("session-absent")).toEqual({ status: "empty" });
  });

  test("lists every stored session and clears them all", async () => {
    const store = createInMemoryOutcomeStore();
    await store.save(outcome("session-one"));
    await store.save(outcome("session-two"));
    expect([...(await store.list())].sort()).toEqual(["session-one", "session-two"]);
    await store.clear();
    expect(await store.list()).toEqual([]);
    expect(await store.load("session-one")).toEqual({ status: "empty" });
  });
});
