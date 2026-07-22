import { readFile } from "node:fs/promises";
import { expect, test } from "@playwright/test";

test("exports the activity outcome to a local file with zero transmission", async ({ page }) => {
  const crossOrigin: string[] = [];
  await page.goto("/");
  const origin = new URL(page.url()).origin;
  page.on("request", (r) => {
    const url = r.url();
    // A blob:/data: download is not a network request; only real cross-origin
    // traffic would break the no-transmission invariant.
    if (url.startsWith("blob:") || url.startsWith("data:")) return;
    if (new URL(url).origin !== origin) crossOrigin.push(`${r.method()} ${url}`);
  });

  await expect(page.locator("html")).toHaveAttribute("data-hydrated", "true");
  await page.getByRole("button", { name: "Terminer" }).click();
  await expect(page.getByTestId("state")).toContainText("Terminée");

  const [download] = await Promise.all([
    page.waitForEvent("download"),
    page.getByRole("button", { name: "Télécharger mon activité" }).click(),
  ]);

  expect(download.suggestedFilename()).toBe("practices-activite.json");
  const path = await download.path();
  const doc = JSON.parse(await readFile(path, "utf8"));
  expect(doc.schemaVersion).toBe("libre-ai.activity-outcome.v1");
  expect(doc.state).toBe("completed");
  // Digest only — raw responses never accompany the export.
  expect(doc.responseDigest).toMatch(/^[0-9a-f]{64}$/);
  expect(doc.responses).toBeUndefined();

  // The export produced a file entirely on-device: no cross-origin request.
  expect(crossOrigin).toEqual([]);
});

test("deletes the stored outcome after in-page confirmation and resets on reload", async ({
  page,
}) => {
  await page.goto("/");
  await expect(page.locator("html")).toHaveAttribute("data-hydrated", "true");
  await page.getByRole("button", { name: "Terminer" }).click();
  await expect(page.getByTestId("state")).toContainText("Terminée");

  await page.getByRole("button", { name: "Supprimer mon activité" }).click();
  await page.getByRole("button", { name: "Confirmer la suppression" }).click();
  await expect(page.getByTestId("delete-done")).toBeVisible();

  // The stored outcome is cleared: a reload finds an empty store and starts fresh
  // at the fixture state ("En cours"), the completed state gone.
  await page.reload();
  await expect(page.getByTestId("state")).toContainText("En cours");
});

test("cancelling the delete confirmation keeps the outcome", async ({ page }) => {
  await page.goto("/");
  await expect(page.locator("html")).toHaveAttribute("data-hydrated", "true");
  await page.getByRole("button", { name: "Terminer" }).click();
  await expect(page.getByTestId("state")).toContainText("Terminée");

  await page.getByRole("button", { name: "Supprimer mon activité" }).click();
  await page.getByRole("button", { name: "Annuler" }).click();

  await expect(page.getByTestId("state")).toContainText("Terminée");
});
