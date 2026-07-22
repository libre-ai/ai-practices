import type { DocumentDescriptor } from "@libre-ai/web-platform";
import { ActivityApp } from "../ui/activity-app";

// SSR renders the fixture activity (no store on the server); the clientModule
// hydrates it into the interactive, IndexedDB-backed app.
export function practicesDocument(): DocumentDescriptor {
  return {
    app: <ActivityApp />,
    clientModule: "/assets/app.js",
    description: "Libre AI — Pratiques d'apprentissage entièrement sur votre appareil.",
    lang: "fr",
    manifest: "/manifest.webmanifest",
    stylesheets: ["/assets/styles.css"],
    title: "Libre AI — Pratiques",
  };
}
