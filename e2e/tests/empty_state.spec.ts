import { expect, test } from "../fixtures";

test.describe("EmptyState", () => {
  test("offers the next step to mouse and keyboard users", async ({ page, example }) => {
    const stage = await example("empty-state/basic");
    const status = stage.getByRole("status");
    const add = stage.getByRole("button", { name: "Add a cluster" });

    await expect(stage.getByText("No clusters yet")).toBeVisible();
    await add.click();
    await expect(status).toHaveText("Started adding 1 clusters");

    await add.focus();
    await page.keyboard.press("Enter");
    await expect(status).toHaveText("Started adding 2 clusters");

    await page.keyboard.press("Tab");
    await expect(stage.getByRole("link", { name: "Read the guide" })).toBeFocused();
  });

  test("is not announced as an alert", async ({ example }) => {
    const stage = await example("empty-state/tones");
    await expect(stage.getByText("No incidents reported")).toBeVisible();
    await expect(stage.getByRole("alert")).toHaveCount(0);
  });
});

test.describe("ErrorState", () => {
  test("is an alert that can be retried", async ({ page, example }) => {
    const stage = await example("empty-state/error");
    const alert = stage.getByRole("alert");
    const retry = alert.getByRole("button", { name: "Try again" });
    const status = stage.getByRole("status");

    await expect(alert).toContainText("The server could not be reached");
    await retry.click();
    await expect(status).toHaveText("Retried 1 times");
    await expect(retry).toHaveAttribute("aria-busy", "true");

    await retry.click({ force: true });
    await expect(status).toHaveText("Retried 1 times");

    await stage.getByRole("button", { name: "Fail the retry" }).click();
    await expect(retry).not.toHaveAttribute("aria-busy", "true");
    await retry.focus();
    await page.keyboard.press("Enter");
    await expect(status).toHaveText("Retried 2 times");
  });

  test("offers no retry when trying again cannot help", async ({ example }) => {
    const stage = await example("empty-state/error-no-retry");
    const refused = stage.getByRole("alert").filter({ hasText: "Not permitted" });

    await expect(refused.getByRole("button")).toHaveCount(0);
    await expect(refused.getByRole("link", { name: "Request access" })).toBeVisible();
  });
});
