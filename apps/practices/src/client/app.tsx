import { hydrateDocument } from "@libre-ai/web-platform/client";
import { createIndexedDbOutcomeStore } from "../persistence/indexed-db-outcome-store";
import { practicesDocument } from "../shared/document";
import { ActivityApp } from "../ui/activity-app";

// Hydrate with the real IndexedDB-backed app. The document's `app` node is replaced
// by the store-backed wrapper; its initial render (fixture activity) matches the
// server markup, then the mount effect loads the persisted outcome. Local-only: the
// store is the ONLY sink for state advances; nothing is transmitted.
const store = createIndexedDbOutcomeStore(globalThis.indexedDB);
hydrateDocument({
  ...practicesDocument(),
  app: <ActivityApp store={store} />,
});

if ("serviceWorker" in navigator) {
  window.addEventListener("load", () => {
    void navigator.serviceWorker.register("/sw.js", { scope: "/" }).catch(() => undefined);
  });
}
