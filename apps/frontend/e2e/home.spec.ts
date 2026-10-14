import { expect, test } from "@playwright/test";

test("loads the application", async ({ page }) => {
  await page.goto("/");

  await expect(page).toHaveTitle("Rust Template");
  await expect(page.getByRole("heading", { name: "Rust Template" })).toBeVisible();
});
