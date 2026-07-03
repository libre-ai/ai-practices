// Service worker for rumble-ai-practices — offline app-shell cache.
//
// NOTE (distribution): a service worker only controls URLs under its own path,
// so this file must be SERVED AT THE SITE ROOT (`/sw.js`) to cache the shell.
// Under `dx`, hashed assets land in `/assets/`, so wiring root-scoped offline
// is a deploy-time step (copy this file to the web root, or set
// `Service-Worker-Allowed: /`). Until then registration degrades gracefully
// and the app stays installable via the manifest. See docs/deploy.md.

const CACHE = "raip-shell-v1";
const SHELL = ["./", "./index.html"];

self.addEventListener("install", (event) => {
  event.waitUntil(caches.open(CACHE).then((c) => c.addAll(SHELL)).catch(() => {}));
  self.skipWaiting();
});

self.addEventListener("activate", (event) => {
  event.waitUntil(
    caches.keys().then((keys) =>
      Promise.all(keys.filter((k) => k !== CACHE).map((k) => caches.delete(k))),
    ),
  );
  self.clients.claim();
});

self.addEventListener("fetch", (event) => {
  if (event.request.method !== "GET") return;
  event.respondWith(
    caches.match(event.request).then((hit) => hit || fetch(event.request)),
  );
});
