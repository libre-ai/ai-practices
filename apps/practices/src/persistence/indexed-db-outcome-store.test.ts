import { describe, expect, test } from "bun:test";
import { IDBFactory } from "fake-indexeddb";
import { type ActivityOutcome, createOutcome } from "../domain/activity-outcome";
import { createIndexedDbOutcomeStore } from "./indexed-db-outcome-store";

const NOW = "2030-01-01T00:00:00Z";
const DB = "practices-test";

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
  if (!created.ok) throw new Error(`fixture refused: ${created.refusal}`);
  return created.value;
}

// Seed a raw record directly, bypassing the domain, to simulate on-disk corruption.
function seedRaw(factory: IDBFactory, localSessionId: string, raw: string): Promise<void> {
  return new Promise((resolve, reject) => {
    const request = factory.open(DB, 1);
    request.onupgradeneeded = () => {
      const database = request.result;
      if (!database.objectStoreNames.contains("outcomes")) {
        database.createObjectStore("outcomes", { keyPath: "localSessionId" });
      }
    };
    request.onsuccess = () => {
      const database = request.result;
      const transaction = database.transaction("outcomes", "readwrite");
      transaction.objectStore("outcomes").put({ localSessionId, raw });
      transaction.oncomplete = () => {
        database.close();
        resolve();
      };
      transaction.onerror = () => {
        database.close();
        reject(transaction.error ?? new Error("seed failed"));
      };
    };
    request.onerror = () => reject(request.error ?? new Error("seed open failed"));
  });
}

describe("IndexedDB outcome store — round-trip on fake-indexeddb", () => {
  test("saves and loads an outcome by localSessionId", async () => {
    const store = createIndexedDbOutcomeStore(new IDBFactory(), DB);
    const original = outcome("session-one");
    await store.save(original);
    const result = await store.load("session-one");
    expect(result.status).toBe("loaded");
    if (result.status !== "loaded") return;
    expect(result.outcome).toEqual(original);
  });

  test("an unknown localSessionId is empty", async () => {
    const store = createIndexedDbOutcomeStore(new IDBFactory(), DB);
    expect(await store.load("session-absent")).toEqual({ status: "empty" });
  });

  test("overwrites the record for the same session on re-save", async () => {
    const store = createIndexedDbOutcomeStore(new IDBFactory(), DB);
    await store.save(outcome("session-one"));
    const advanced = createOutcome(
      "urn:libre-ai:activity-outcome:o1",
      { activityId: "urn:libre-ai:activity:a1", activityVersion: "1.0.0" },
      "session-one",
      "completed",
      "b".repeat(64),
      ["rule-a", "rule-b"],
      NOW,
    );
    if (!advanced.ok) throw new Error("advanced fixture refused");
    await store.save(advanced.value);
    const result = await store.load("session-one");
    expect(result.status).toBe("loaded");
    if (result.status !== "loaded") return;
    expect(result.outcome.state).toBe("completed");
    expect(result.outcome.feedbackRuleIds).toEqual(["rule-a", "rule-b"]);
    expect(await store.list()).toEqual(["session-one"]);
  });

  test("lists stored sessions and clears them", async () => {
    const store = createIndexedDbOutcomeStore(new IDBFactory(), DB);
    await store.save(outcome("session-one"));
    await store.save(outcome("session-two"));
    expect([...(await store.list())].sort()).toEqual(["session-one", "session-two"]);
    await store.clear();
    expect(await store.list()).toEqual([]);
    expect(await store.load("session-one")).toEqual({ status: "empty" });
  });

  test("a corrupt stored record surfaces as corrupt, not a rehydrated outcome", async () => {
    const factory = new IDBFactory();
    await seedRaw(factory, "session-corrupt", "{ not valid json");
    const store = createIndexedDbOutcomeStore(factory, DB);
    expect(await store.load("session-corrupt")).toEqual({
      status: "corrupt",
      refusal: "practices.response_schema_invalid",
    });
  });

  test("save rejects when the factory cannot open", async () => {
    const broken = {
      open: () => {
        throw new Error("no indexeddb");
      },
    } as unknown as IDBFactory;
    const store = createIndexedDbOutcomeStore(broken, DB);
    await expect(store.save(outcome("session-one"))).rejects.toThrow();
  });
});
