import type { Locator, Page } from "@playwright/test";
import { expect, test, type Theme } from "../fixtures";

/** The area around an example that its tooltips can reach. */
async function around(page: Page, stage: Locator) {
  const box = (await stage.boundingBox())!;
  const viewport = page.viewportSize()!;
  return { x: 0, y: 0, width: Math.min(box.x + box.width, viewport.width), height: box.y + box.height + 72 };
}

// The site stops its clock at 12:00 UTC on Thursday 15 January 2026.
test.describe("Timestamp", () => {
  test.describe("for a viewer in Berlin", () => {
    test.use({ timezoneId: "Europe/Berlin" });

    test("hover shows the exact local time, then the UTC time", async ({ page, example }) => {
      const stage = await example("timestamp/basic");
      const tooltip = page.getByRole("tooltip");

      await stage.getByText("35 minutes ago").hover();
      await expect(tooltip).toContainText("Thursday 15 January 2026, 12:25:00");
      await expect(tooltip).toContainText(/11:25:00\s*UTC/);

      await page.mouse.move(0, 0);
      await expect(tooltip).toBeHidden();
    });

    for (const theme of ["light", "dark"] as Theme[]) {
      test(`looks right with the tooltip shown: ${theme}`, async ({ page, example }) => {
        const stage = await example("timestamp/keyboard", { theme });
        await stage.getByText("in 3 hours").hover();
        await expect(page.getByRole("tooltip")).toBeVisible();
        await expect(page).toHaveScreenshot(["timestamp", `shown-${theme}.png`], { clip: await around(page, stage) });
      });
    }
  });

  test("dates and times are written in the viewer's time zone", async ({ browser, baseURL }) => {
    const read = async (timezoneId: string) => {
      const context = await browser.newContext({ timezoneId, baseURL });
      const page = await context.newPage();
      await page.goto("/_examples/timestamp/display");
      await page.locator("html[data-hydrated]").waitFor({ state: "attached" });
      const text = await page.locator("[data-example]").innerText();
      await context.close();
      return text;
    };

    const [tokyo, losAngeles] = [await read("Asia/Tokyo"), await read("America/Los_Angeles")];
    // 14:05 UTC on Monday 12 January.
    expect(tokyo).toContain("12 Jan 2026, 23:05");
    expect(losAngeles).toContain("12 Jan 2026, 06:05");
    // Relative text does not depend on the zone.
    expect(tokyo).toContain("2 days ago");
    expect(losAngeles).toContain("2 days ago");
  });

  test("the exact time is given in the viewer's zone and in UTC", async ({ browser, baseURL }) => {
    const context = await browser.newContext({ timezoneId: "Asia/Tokyo", baseURL });
    const page = await context.newPage();

    await page.goto("/_examples/timestamp/detail");
    await page.locator("html[data-hydrated]").waitFor({ state: "attached" });
    const detail = page.locator("[data-example]");
    await expect(detail).toContainText("Thursday 15 January 2026, 20:25:42");
    await expect(detail).toContainText(/11:25:42\s*UTC/);
    await context.close();
  });

  test("a focusable timestamp shows its tooltip on keyboard focus, and Escape dismisses it", async ({ page, example }) => {
    await example("timestamp/keyboard");
    const tooltip = page.getByRole("tooltip");

    await page.keyboard.press("Tab");
    await expect(tooltip).toBeHidden();
    await page.keyboard.press("Tab");
    await expect(tooltip).toContainText("Thursday 15 January 2026, 15:30:00");

    await page.keyboard.press("Escape");
    await expect(tooltip).toBeHidden();
  });

  test("relative text keeps itself current", async ({ page, example }) => {
    await page.clock.install({ time: new Date("2026-01-15T11:59:00Z") });
    const stage = await example("timestamp/live");
    await page.clock.pauseAt(new Date("2026-01-15T12:00:00Z"));
    const status = stage.getByRole("status");
    await expect(status).toHaveText("Nothing recorded yet");

    await stage.getByRole("button", { name: "Record an event" }).click();
    await expect(status).toHaveText("Recorded just now");

    await page.clock.runFor(59_000);
    await expect(status).toHaveText("Recorded just now");
    await page.clock.runFor(1_000);
    await expect(status).toHaveText("Recorded 1 minute ago");
    await page.clock.runFor(120_000);
    await expect(status).toHaveText("Recorded 3 minutes ago");
  });
});
