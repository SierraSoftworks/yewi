import { expect, test } from "../fixtures";

test.describe("Duration", () => {
  test("a running duration counts up by itself", async ({ page, example }) => {
    await page.clock.install({ time: new Date("2026-01-15T11:59:00Z") });
    const stage = await example("duration/running");
    await page.clock.pauseAt(new Date("2026-01-15T12:00:00Z"));
    const status = stage.getByRole("status");
    await expect(status).toHaveText("Not started");

    await stage.getByRole("button", { name: "Start a run" }).click();
    await expect(status).toContainText("for 0s");

    await page.clock.runFor(5_000);
    await expect(status).toContainText("for 5s");
    await page.clock.runFor(60_000);
    await expect(status).toContainText("for 1m 05s");
    await expect(stage.locator("time")).toHaveAttribute("datetime", "PT65S");
  });
});
