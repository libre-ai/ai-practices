import { test, expect } from "@playwright/test";
import { readFileSync } from "node:fs";

// End-to-end for the reflex parcours. These reproduce the manual browser checks
// (reactivity, keyboard, in-place reveal, idk, export) as CI-gateable tests.
//
// "Jeu en avant": there is no blocking onboarding gate anymore. The first
// situation is judgeable the instant the page loads — the manifesto thesis sits
// above it and its facts sit below as reachable context, never a wall. So every
// flow starts by interacting with situation 1 directly, not by clicking a start.

test.beforeEach(async ({ page }) => {
  await page.goto("/");
  await expect(page.locator(".intro-title")).toBeVisible();
});

test("opens on the manifesto with a playable first situation (jeu en avant)", async ({
  page,
}) => {
  // the thesis meets you at the door…
  await expect(page.locator(".intro-title")).toContainText("Aucune image générée");
  // …and the first situation is immediately judgeable — no blocking gate
  await expect(page.locator('.console [role="radiogroup"]')).toBeVisible();
  await expect(page.locator(".q-count")).toHaveText(/^1 \/ \d+$/);
  // the positioning stands below as reachable context, not a wall
  await expect(page.getByText("jamais un classement")).toBeVisible();
});

test("the first situation validates in place, without scrolling the page", async ({
  page,
}) => {
  const before = await page.evaluate(() => window.scrollY);
  const c2 = page.locator('.choice[data-key="2"]');
  await c2.click(); // first tap selects
  await expect(page.locator(".choice.sel")).toBeVisible();
  await c2.click(); // second tap on the same choice validates
  await expect(page.locator(".answered")).toBeVisible();
  await expect(page.locator(".pinned .verdict-tag")).toBeVisible();
  const after = await page.evaluate(() => window.scrollY);
  expect(after).toBe(before); // in-place reveal: no reflow, no scroll
});

test("keyboard: a number selects, Enter validates then continues", async ({ page }) => {
  await page.keyboard.press("2");
  await expect(page.locator('.choice[data-key="2"].sel')).toBeVisible();
  await page.keyboard.press("Enter"); // validate
  await expect(page.locator(".feedback-panel")).toBeVisible();
  await page.keyboard.press("Enter"); // continue
  await expect(page.locator(".q-count")).toHaveText(/^2 \/ \d+$/);
});

// Answer every situation (pick choice 2, one-gesture validate) up to the summary.
// Situation 1 is already on screen (jeu en avant), so no start click is needed.
async function completeParcours(page: import("@playwright/test").Page): Promise<number> {
  const label = await page.locator(".q-count").first().textContent();
  const total = Number((label ?? "").split("/")[1].trim());
  for (let i = 0; i < total; i++) {
    const choice = page.locator('.choice[data-key="2"]');
    await choice.click();
    await choice.click();
    await page.locator('[data-action="continue"]').click();
  }
  return total;
}

test('"je ne sais pas" (Space) is an honest submission with guidance', async ({ page }) => {
  await page.keyboard.press(" ");
  const panel = page.locator('.feedback-panel[data-verdict="idk"]');
  await expect(panel).toContainText("Réponse non tranchée");
  await expect(page.locator(".idk-action")).not.toBeEmpty();
});

test("full parcours reaches a per-category synthesis; R restarts", async ({ page }) => {
  const total = await completeParcours(page);
  await expect(page.locator(".summary-panel")).toBeVisible();
  await expect(page.locator(".summary-row")).toHaveCount(total);
  await expect(page.locator(".summary-panel")).toContainText("Aucun classement nominatif");
  await page.keyboard.press("r");
  // restart returns to the door: the manifesto thesis and a fresh situation 1
  await expect(page.locator(".intro-title")).toBeVisible();
  await expect(page.locator(".q-count")).toHaveText(/^1 \/ \d+$/);
});

test("local export downloads the synthesis as JSON", async ({ page }) => {
  const total = await completeParcours(page);
  const [download] = await Promise.all([
    page.waitForEvent("download"),
    page.locator('[data-action="export"]').click(),
  ]);
  expect(download.suggestedFilename()).toBe("rumble-ai-practices-synthese.json");

  const payload = JSON.parse(readFileSync(await download.path(), "utf8"));
  expect(payload.answered_count).toBe(total);
  expect(payload.outcomes).toHaveLength(total);
  // the RUM selection->validation metric rides along in the export
  expect(Array.isArray(payload.rum.select_to_validate_ms)).toBe(true);
  expect(payload.rum.select_to_validate_ms.length).toBeGreaterThan(0);
  expect(typeof payload.rum.median_ms).toBe("number");
});
