import { expect, test } from "../fixtures";

test.describe("Panel", () => {
  test("opens and closes with the mouse", async ({ example }) => {
    const stage = await example("panel/collapsible");
    const status = stage.getByRole("status");
    const detail = stage.getByText("A host must report healthy");

    await expect(detail).toBeHidden();
    await expect(status).toHaveText("Advanced settings are hidden");

    await stage.getByText("Advanced", { exact: true }).click();
    await expect(detail).toBeVisible();
    await expect(status).toHaveText("Advanced settings are shown");

    await stage.getByText("Advanced", { exact: true }).click();
    await expect(detail).toBeHidden();
    await expect(status).toHaveText("Advanced settings are hidden");
  });

  test("opens and closes with the keyboard, with a visible focus indicator", async ({ page, example }) => {
    const stage = await example("panel/collapsible");
    const schedule = stage.getByText("Rollouts start inside the maintenance window");
    const summary = stage.locator("summary").first();

    await expect(schedule).toBeVisible();
    await page.keyboard.press("Tab");
    await expect(summary).toBeFocused();
    await expect(summary).not.toHaveCSS("outline-style", "none");

    await page.keyboard.press("Enter");
    await expect(schedule).toBeHidden();
    await page.keyboard.press("Space");
    await expect(schedule).toBeVisible();
  });

  test("a fixed fill opts out of the alternation", async ({ example }) => {
    const stage = await example("panel/fills");
    const fill = (name: string) =>
      stage.locator("section", { has: stage.page().getByRole("heading", { name }) }).last().evaluate((el) => getComputedStyle(el).backgroundColor);
    expect(await fill("Strong")).not.toBe(await fill("Soft"));
  });
});
