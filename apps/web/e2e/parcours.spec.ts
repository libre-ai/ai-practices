import { test, expect } from "@playwright/test";
import { readFileSync } from "node:fs";

// End-to-end for the reflex parcours. These reproduce the manual browser checks
// (reactivity, keyboard, in-place reveal, idk, export) as CI-gateable tests.

test.beforeEach(async ({ page }) => {
  await page.goto("/");
  await expect(page.locator(".intro-title")).toBeVisible();
});

test("opens on the onboarding gate, not a question", async ({ page }) => {
  await expect(page.locator(".intro-title")).toContainText("Aucune image générée n'est neutre.");
  await expect(page.getByText("Pas une évaluation RH")).toBeVisible();
  await expect(page.locator('[role="radiogroup"]')).toHaveCount(0);
});

test("Enter launches the parcours from the menu", async ({ page }) => {
  await page.locator('[data-action="start"]').focus();
  await page.keyboard.press("Enter");
  await expect(page.locator('.console [role="radiogroup"]')).toBeVisible();
  await expect(page.locator(".q-count")).toHaveText(/^1 \/ \d+$/);
});

test("one-gesture touch validates in place, without scrolling the page", async ({ page }) => {
  // Measure scroll position after the manifesto gate is visible but before starting.
  // This captures the scroll position after the gate, which becomes our baseline.
  await expect(page.locator(".intro-title")).toContainText("Aucune image générée n'est neutre.");
  const baselineScroll = await page.evaluate(() => window.scrollY);

  await page.locator('[data-action="start"]').click();
  await expect(page.locator('.console [role="radiogroup"]')).toBeVisible();

  const before = await page.evaluate(() => window.scrollY);
  const c2 = page.locator('.choice[data-key="2"]');
  await c2.click(); // first tap selects
  await expect(page.locator(".choice.sel")).toBeVisible();
  await c2.click(); // second tap on the same choice validates
  await expect(page.locator(".answered")).toBeVisible();
  await expect(page.locator(".pinned .verdict-tag")).toBeVisible();
  const after = await page.evaluate(() => window.scrollY);
  expect(after).toBe(before); // in-place reveal: no reflow, no scroll during verdict
});

test("keyboard: a number selects, Enter validates then continues", async ({ page }) => {
  await page.locator('[data-action="start"]').click();
  await page.keyboard.press("2");
  await expect(page.locator('.choice[data-key="2"].sel')).toBeVisible();
  await page.keyboard.press("Enter"); // validate
  await expect(page.locator(".feedback-panel")).toBeVisible();
  await page.keyboard.press("Enter"); // continue
  await expect(page.locator(".q-count")).toHaveText(/^2 \/ \d+$/);
});

// Answer every question (pick choice 2, one-gesture validate) up to the summary.
async function completeParcours(page: import("@playwright/test").Page): Promise<number> {
  await page.locator('[data-action="start"]').click();
  const label = await page.locator(".q-count").first().textContent();
  const total = Number((label ?? "").split("/")[1].trim());
  for (let i = 0; i < total; i++) {
    const choice = page.locator('.choice[data-key="2"]');
    await choice.click();
    await choice.click();
    // Wait for the continue button to appear (verdict rendered) before clicking
    await page.locator('[data-action="continue"]').waitFor({ state: "visible" });
    await page.locator('[data-action="continue"]').click();
  }
  return total;
}

test('"je ne sais pas" (Space) is an honest submission with guidance', async ({ page }) => {
  await page.locator('[data-action="start"]').click();
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
  await expect(page.locator(".intro-title")).toBeVisible();
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
