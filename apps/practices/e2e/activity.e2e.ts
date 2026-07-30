import { expect, test } from "@playwright/test";

test("activity outcome persists on-device and is restored on reload, with zero transmission", async ({
  page,
}) => {
  const requests: string[] = [];

  // First navigate to establish the origin
  await page.goto("/");

  // Now set up request interception for cross-origin detection
  const origin = new URL(page.url()).origin;
  page.on("request", (r) => {
    if (new URL(r.url()).origin !== origin) {
      requests.push(`${r.method()} ${r.url()}`);
    }
  });

  // Verify hydration completed
  await expect(page.locator("html")).toHaveAttribute("data-hydrated", "true");

  // Click "Terminer" to advance the state
  await page.getByRole("button", { name: "Terminer" }).click();

  // Verify the state changed to "Terminée"
  await expect(page.getByTestId("state")).toContainText("Terminée");

  // Reload: the state should be restored from IndexedDB
  await page.reload();
  await expect(page.getByTestId("state")).toContainText("Terminée");

  // Verify no cross-origin request was made (local-only operation)
  expect(requests).toEqual([]);
});
