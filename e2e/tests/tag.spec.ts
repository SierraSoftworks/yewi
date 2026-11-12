import { expect, test } from "../fixtures";

test.describe("Tag", () => {
  test("is removed with the mouse", async ({ example }) => {
    const stage = await example("tag/removable");
    const status = stage.getByRole("status");

    await expect(status).toHaveText("3 filters");
    await stage.getByRole("button", { name: "Remove region eu-west" }).click();
    await expect(status).toHaveText("Removed eu-west, 2 filters left");
    await expect(stage.getByText("eu-west", { exact: true })).toHaveCount(0);
    await expect(stage.getByText("production")).toBeVisible();
  });

  test("is removed with the keyboard", async ({ page, example }) => {
    const stage = await example("tag/removable");
    const status = stage.getByRole("status");

    await page.keyboard.press("Tab");
    const first = stage.getByRole("button", { name: "Remove env production" });
    await expect(first).toBeFocused();
    await expect(first).not.toHaveCSS("outline-style", "none");
    await page.keyboard.press("Enter");
    await expect(status).toHaveText("Removed production, 2 filters left");

    await stage.getByRole("button", { name: "Remove os linux" }).focus();
    await page.keyboard.press("Space");
    await expect(status).toHaveText("Removed linux, 1 filters left");

    await stage.getByRole("button", { name: "Reset" }).click();
    await expect(status).toHaveText("3 filters");
  });

  test("cannot be removed while disabled", async ({ example }) => {
    const stage = await example("tag/removable");
    const locked = stage.getByRole("button", { name: "Remove owner platform" });

    await expect(locked).toBeDisabled();
    await locked.click({ force: true });
    await expect(stage.getByText("platform")).toBeVisible();
    await expect(stage.getByRole("status")).toHaveText("3 filters");
  });

  test("with href links its label and keeps the remove button separate", async ({ page, example }) => {
    const stage = await example("tag/link");
    const link = stage.getByRole("link", { name: "main" });

    await expect(link).toHaveAttribute("href", "#branch-main");
    await expect(stage.getByRole("button", { name: "Remove main" })).toBeVisible();

    await stage.getByRole("link", { name: "linux" }).focus();
    await page.keyboard.press("Enter");
    await expect(page).toHaveURL(/#os-linux$/);
  });
});
