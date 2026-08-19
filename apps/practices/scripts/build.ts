import { mkdir, rm } from "node:fs/promises";
import { join } from "node:path";
import { buildTailwindUtilities } from "@libre-ai/ui/tailwind";
import { renderStaticDocument } from "@libre-ai/web-platform";
import { practicesDocument } from "../src/shared/document";
import { renderServiceWorker } from "./build-service-worker";

const root = join(import.meta.dir, "..");
const dist = join(root, "dist");
const assets = join(dist, "assets");

await rm(dist, { force: true, recursive: true });
await mkdir(join(dist, "static"), { recursive: true });
await mkdir(assets, { recursive: true });

const clientBuild = await Bun.build({
  // @libre-ai/ui and @libre-ai/web-platform are consumed as source git-deps (no
  // publish step produced their `dist/*` export targets), so the browser-target
  // default conditions ("import"/"default" -> dist/*) do not resolve. The "bun"
  // condition maps to their TypeScript source instead, which the bundler can
  // still transpile for a browser output regardless of this condition's name.
  conditions: ["bun"],
  define: { "process.env.NODE_ENV": JSON.stringify("production") },
  entrypoints: [join(root, "src/client/app.tsx")],
  minify: true,
  naming: "app.js",
  outdir: assets,
  sourcemap: "none",
  target: "browser",
});
if (!clientBuild.success) {
  throw new Error("web.client_build_failed");
}

// @libre-ai/ui is consumed as a pinned external package (github git-dep), not a
// local monorepo workspace member — its foundation stylesheet ships inside the
// installed package itself.
const foundationCss = await Bun.file(join(root, "node_modules/@libre-ai/ui/src/styles.css")).text();
const utilityCss = await buildTailwindUtilities([]);
await Bun.write(join(assets, "styles.css"), `${foundationCss}\n${utilityCss}`);
await Bun.write(join(assets, "icon.svg"), Bun.file(join(root, "public/icon.svg")));
await Bun.write(join(dist, "static/index.html"), renderStaticDocument(practicesDocument()));
await Bun.write(
  join(dist, "manifest.webmanifest"),
  `${JSON.stringify(
    {
      background_color: "#f7f6f0",
      display: "standalone",
      icons: [{ sizes: "any", src: "/assets/icon.svg", type: "image/svg+xml" }],
      id: "/",
      lang: "fr",
      name: "Libre AI — Pratiques",
      scope: "/",
      short_name: "Pratiques",
      start_url: "/static",
      theme_color: "#075e54",
    },
    null,
    2,
  )}\n`,
);
const cachedAssets = [
  "/assets/app.js",
  "/assets/icon.svg",
  "/assets/styles.css",
  "/manifest.webmanifest",
  "/static",
] as const;
const cacheHasher = new Bun.CryptoHasher("sha256");
for (const path of cachedAssets) {
  const relativePath = path === "/static" ? "static/index.html" : path.slice(1);
  cacheHasher.update(`${relativePath}\0`);
  cacheHasher.update(await Bun.file(join(dist, relativePath)).arrayBuffer());
}
const cacheDigest = cacheHasher.digest("hex");
await Bun.write(join(dist, "sw.js"), renderServiceWorker(cachedAssets, cacheDigest));

console.log("Built SSR client, static document and local PWA assets");
