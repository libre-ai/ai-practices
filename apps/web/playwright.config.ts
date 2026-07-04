import { defineConfig, devices } from "@playwright/test";

// Durable e2e for the web PWA (browser-tooling rule: `playwright test` in-repo
// is the canonical tool for durable e2e). Runs against a served build; locally
// it reuses a running `dx serve` on :8080, in CI it builds+serves the release.
export default defineConfig({
  testDir: "./e2e",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  // one retry absorbs transient dev-server latency under parallel load; a real
  // failure fails both attempts. Cap local workers so the dev server isn't
  // hammered by 14 cold loads at once (CI serves a fast static build).
  retries: 1,
  workers: process.env.CI ? undefined : 4,
  reporter: process.env.CI ? "github" : "list",
  use: {
    baseURL: "http://localhost:8080",
    trace: "on-first-retry",
  },
  projects: [
    { name: "chromium", use: { ...devices["Desktop Chrome"] } },
    { name: "mobile", use: { ...devices["Pixel 7"] } },
  ],
  webServer: {
    command: "dx serve --platform web",
    url: "http://localhost:8080",
    reuseExistingServer: true,
    timeout: 240_000,
  },
});
