import { expect, test } from "../fixtures";

test.describe("Loading", () => {
  test("announces what is loading, with or without a visible label", async ({ example }) => {
    const inline = await example("loading/basic");
    await expect(inline.getByRole("status").filter({ hasText: "Loading hosts" })).toBeVisible();

    const hidden = await example("loading/label-hidden");
    const status = hidden.getByRole("status").filter({ hasText: "Loading chart" });
    await expect(status).toBeVisible();
    const label = status.getByText("Loading chart");
    expect((await label.boundingBox())?.width).toBeLessThanOrEqual(1);
  });

  test("stands in for a section until its content arrives", async ({ example }) => {
    const stage = await example("loading/block");
    const section = stage.getByRole("region", { name: "Rollouts" });
    await expect(section.getByRole("status")).toHaveText("Loading rollouts");
    await expect(section.getByRole("status")).toBeVisible();

    await stage.getByRole("button", { name: "Finish loading" }).click();
    await expect(section.getByRole("status")).toHaveCount(0);
    await expect(section).toContainText("2 of 31 hosts done");
  });

  test("an overlay blocks its content while refreshing and releases it after", async ({ page, example }) => {
    const stage = await example("loading/overlay");
    const restart = stage.getByRole("button", { name: "Restart agent", includeHidden: true });
    const restarts = stage.getByRole("status").filter({ hasText: "Restarted" });
    const refreshing = stage.getByRole("status").filter({ hasText: "Refreshing hosts" });

    await expect(refreshing).toBeVisible();
    await expect(stage.locator("[aria-busy=true]")).toContainText("edge-ams-01");
    await restart.click({ force: true });
    await restart.focus();
    await expect(restart).not.toBeFocused();
    await expect(restarts).toHaveText("Restarted 0 times");

    await stage.getByRole("button", { name: "Finish refreshing" }).click();
    await expect(refreshing).toHaveCount(0);
    await restart.click();
    await expect(restarts).toHaveText("Restarted 1 times");

    await stage.getByRole("button", { name: "Refresh", exact: true }).click();
    await expect(refreshing).toBeVisible();
    await page.keyboard.press("Shift+Tab");
    await expect(restart).not.toBeFocused();
  });
});
