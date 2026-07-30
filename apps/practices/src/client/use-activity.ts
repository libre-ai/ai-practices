import { useEffect, useState } from "react";
import {
  type ActivityOutcome,
  advanceState,
  type ExportedActivityOutcome,
  exportOutcome,
} from "../domain/activity-outcome";
import type { LocalOutcomeStore } from "../persistence/local-outcome-store";
import { ACTIVITY_FIXTURE, SESSION_ID } from "../ui/fixture";

export type ActivityStatus = "loading" | "ready" | "corrupt";

export interface ActivityController {
  readonly outcome: ActivityOutcome;
  readonly status: ActivityStatus;
  readonly complete: () => void;
  readonly stop: () => void;
  // Data-ownership: `exportData` returns the outcome as its non-identifying export
  // document (digest only — raw responses never accompany the export); the caller
  // turns it into a LOCAL file download — never a network upload. `deleteAll`
  // erases the stored outcome and resets to the fixture; it resolves only once the
  // store has durably cleared, so the caller can truthfully announce deletion
  // (unlike the optimistic answer/skip saves, a deletion claim must be durable).
  readonly exportData: () => ExportedActivityOutcome;
  readonly deleteAll: () => Promise<void>;
}

// The interactive controller. Without a store (SSR) it stays at the fixture outcome with
// status "ready" (the SSR baseline). With a store, the initial status is "loading"
// until the mount effect loads the persisted outcome; it then persists every mutation.
// A corrupt local store is surfaced fail-closed (never rehydrated).
export function useActivity(store?: LocalOutcomeStore): ActivityController {
  const [outcome, setOutcome] = useState<ActivityOutcome>(ACTIVITY_FIXTURE);
  const [status, setStatus] = useState<ActivityStatus>(store ? "loading" : "ready");

  useEffect(() => {
    if (!store) return;
    let cancelled = false;
    store.load(SESSION_ID).then((result) => {
      if (cancelled) return;
      if (result.status === "loaded") setOutcome(result.outcome);
      setStatus(result.status === "corrupt" ? "corrupt" : "ready");
    });
    return () => {
      cancelled = true;
    };
  }, [store]);

  function commit(next: ActivityOutcome): void {
    setOutcome(next);
    void store?.save(next);
  }

  return {
    outcome,
    status,
    complete() {
      const next = advanceState(outcome, "completed");
      if (next.ok) commit(next.value);
    },
    stop() {
      const next = advanceState(outcome, "stopped");
      if (next.ok) commit(next.value);
    },
    exportData() {
      return exportOutcome(outcome);
    },
    async deleteAll() {
      // Persist first, mutate UI state after: if the clear rejects, the visible
      // state must keep matching the (unchanged) store — never show a reset
      // activity whose old outcome would come back on reload.
      await store?.clear();
      setOutcome(ACTIVITY_FIXTURE);
    },
  };
}
