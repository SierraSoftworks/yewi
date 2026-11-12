import { expect, test } from "../fixtures";

test.describe("Countdown", () => {
  test("counts down by itself and reports when it reaches zero", async ({ page, example }) => {
    await page.clock.install({ time: new Date("2026-01-15T11:59:00Z") });
    const stage = await example("countdown/live");
    await page.clock.pauseAt(new Date("2026-01-15T12:00:00Z"));
    const status = stage.getByRole("status");
    const timer = stage.getByRole("timer");
    await expect(status).toHaveText("Not started");

    await stage.getByRole("button", { name: "Start a 10 second bake" }).click();
    await expect(timer).toHaveText("0:10");
    await expect(status).toHaveText("Baking");

    await page.clock.runFor(4_000);
    await expect(timer).toHaveText("0:06");
    await page.clock.runFor(6_000);
    await expect(timer).toHaveText("0:00");
    await expect(status).toHaveText("Bake finished");
  });

  test("the tooltip gives the exact time it ends", async ({ page, example }) => {
    await page.clock.install({ time: new Date("2026-01-15T11:59:00Z") });
    const stage = await example("countdown/live");
    await page.clock.pauseAt(new Date("2026-01-15T12:00:00Z"));

    await stage.getByRole("button", { name: "Start a 10 second bake" }).click();
    await page.keyboard.press("Tab");
    await expect(page.getByRole("tooltip")).toContainText("Thursday 15 January 2026, 12:00:10");
  });

  test("a passed moment shows its done content", async ({ example }) => {
    const stage = await example("countdown/done");
    await expect(stage.getByRole("timer").first()).toHaveText("0:00");
    await expect(stage.getByRole("timer").last()).toHaveText("expired");
  });
});
