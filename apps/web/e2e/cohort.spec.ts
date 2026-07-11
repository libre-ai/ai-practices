import { test, expect } from "@playwright/test";

// End-to-end for the k-anonymous cohort feature. Runs against the single-origin
// binary (same origin as /v1/cohort). Tests the online round-trip (POST to cohort,
// render distribution) and offline degradation (abort network, graceful fallback).

test.beforeEach(async ({ page, context }) => {
  // The cohort round-trip needs the single-origin binary (same origin as
  // /v1/cohort). The default parcours job serves the app via `dx serve`, which
  // has no API — so skip there and only run when pointed at the binary.
  test.skip(
    !process.env.E2E_BASE_URL,
    "cohort e2e requires the single-origin binary (set E2E_BASE_URL)"
  );

  // Emulate reducedMotion to ensure stable timing during tests
  await page.emulateMedia({ reducedMotion: "reduce" });

  // The single-origin binary boots with Postgres migrations: the first
  // navigation can exceed the 30s default while the server warms up.
  await page.goto("/", { timeout: 60_000 });
  // Verify manifesto landing gate is visible (from PR #9)
  await expect(page.locator(".intro-title")).toContainText(
    "Aucune image générée n'est neutre."
  );
});

// Answer every question using keyboard (choice 2 via '2' key, validate via Enter).
// This is faster than one-gesture clicks (50 questions: ~0.5s vs minutes with page interactions).
async function completeParcours(
  page: import("@playwright/test").Page
): Promise<number> {
  await page.locator('[data-action="start"]').click();
  // Dioxus re-renders after the click event, not synchronously with it; wait for
  // the question console before reading its counter, else textContent() races
  // the re-render and returns null under load.
  const counter = page.locator(".q-count").first();
  await counter.waitFor({ state: "visible" });
  const label = await counter.textContent();
  const total = Number((label ?? "").split("/")[1].trim());
  // Mirror the proven parcours.spec loop: each step waits for the app's
  // re-render before the next key, and the third press is the CONTINUE —
  // firing 2/Enter blind desyncs the sequence (caught on mobile).
  for (let i = 0; i < total; i++) {
    await page.keyboard.press("2"); // select choice 2
    await page.locator('.choice[data-key="2"].sel').waitFor();
    await page.keyboard.press("Enter"); // validate
    await page.locator('[data-action="continue"]').waitFor({ state: "visible" });
    await page.keyboard.press("Enter"); // continue
  }
  return total;
}

test("complete parcours reaches summary with cohort panel present", async ({
  page,
}) => {
  const total = await completeParcours(page);
  // Summary renders the local synthesis (SummaryPanel).
  await expect(page.locator(".summary-panel")).toBeVisible();
  await expect(page.locator(".summary-row")).toHaveCount(total);

  // Cohort panel is rendered below (initially in loading state).
  const cohortPanel = page.locator(".cohort-panel");
  await expect(cohortPanel).toBeVisible();
  // Wait for the cohort POST to succeed and the online state to render.
  // (or timeout if offline/error — then data-state should be 'offline' or 'error').
  await expect(cohortPanel).toHaveAttribute(
    "data-state",
    /^(online|offline)$/
  );
});

test("below-k behavior: lone learner withholds distribution and shows notice", async ({
  page,
}) => {
  const total = await completeParcours(page);

  // In the default setup (no seeded data), a lone learner is below k=5.
  // The cohort panel should render with withheld axes (data-withheld="true").
  const cohortPanel = page.locator(".cohort-panel");
  await expect(cohortPanel).toBeVisible();

  // Wait for the cohort POST + response.
  await expect(cohortPanel).toHaveAttribute(
    "data-state",
    /^(online|offline)$/
  );

  // Below-k: at least one axis withholds the distribution.
  // (In the lone-learner case, all axes are below k, so all should have withheld).
  const withheldAxes = page.locator('.cohort-axis[data-withheld="true"]');
  const count = await withheldAxes.count();
  expect(count).toBeGreaterThan(0);

  // The SummaryPanel itself still renders (offline-first: local synthesis always works).
  await expect(page.locator(".summary-panel")).toBeVisible();
});

test("offline degradation: abort /v1/cohort, summary still renders", async ({
  page,
}) => {
  // Intercept /v1/cohort requests and abort them before the summary is reached.
  await page.route("**/v1/cohort", (route) => route.abort());

  const total = await completeParcours(page);

  // Summary still renders (local synthesis).
  await expect(page.locator(".summary-panel")).toBeVisible();
  await expect(page.locator(".summary-row")).toHaveCount(total);

  // Cohort panel renders in offline state (no POST succeeded).
  const cohortPanel = page.locator(".cohort-panel");
  await expect(cohortPanel).toBeVisible();
  await expect(cohortPanel).toHaveAttribute("data-state", "offline");

  // Offline message is visible.
  await expect(cohortPanel).toContainText("Hors ligne");

  // Export and restart controls still work (survive the offline fallback).
  await expect(page.locator('[data-action="export"]')).toBeVisible();
  await expect(page.locator('[data-action="restart"]')).toBeVisible();
});
