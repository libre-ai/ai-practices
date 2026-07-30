import { expect, test } from "@playwright/test";

test("the activity baseline is usable without JavaScript", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByRole("heading", { level: 1 })).toContainText("Pratiques");
  await expect(page.getByText("Rien n'est transmis")).toBeVisible();
  await expect(page.getByTestId("activity-id")).toContainText(
    "urn:libre-ai:activity:learning-foundations",
  );
  // Interactive state-advance controls are enhanced-only: not operable without JS.
  await expect(page.getByRole("button", { name: "Terminer" })).toHaveCount(0);
  await expect(page.locator("html")).not.toHaveAttribute("data-hydrated", "true");
});
