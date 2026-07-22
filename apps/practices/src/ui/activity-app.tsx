import { StatusMessage } from "@libre-ai/ui";
import { useActivity } from "../client/use-activity";
import type { LocalOutcomeStore } from "../persistence/local-outcome-store";
import { Activity } from "./activity";

// The controller's handlers are passed unconditionally (they fold through the
// domain and persist via the optional store), so the server render (no store) and
// the client's first render (store present) produce IDENTICAL markup — event
// handlers are not serialized into HTML, and no other output depends on the store.
// This keeps hydration a byte-for-byte match; interactivity is inert without
// JavaScript because the controls live in `lai-enhanced-only`.
export function ActivityApp({ store }: { readonly store?: LocalOutcomeStore }) {
  const controller = useActivity(store);
  return (
    <>
      {controller.status === "corrupt" ? (
        <StatusMessage className="lai-status" data-testid="corrupt-notice">
          Votre activité locale est illisible. Recommencez.
        </StatusMessage>
      ) : null}
      <Activity
        outcome={controller.outcome}
        onComplete={controller.complete}
        onStop={controller.stop}
      />
    </>
  );
}
