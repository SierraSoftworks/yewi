import type { Locator, Page } from "@playwright/test";
import { expect, test } from "../fixtures";

/** The area around an example that its popovers can reach. */
async function around(page: Page, stage: Locator) {
  const box = (await stage.boundingBox())!;
  const view = page.viewportSize()!;
  const y = Math.max(0, box.y - 140);
  return { x: 0, y, width: view.width, height: Math.min(view.height - y, box.height + 280) };
}

test.describe("Timeline", () => {
  test("lists its events in order", async ({ example }) => {
    const stage = await example("timeline/commit-log");
    const events = stage.getByRole("list", { name: "Revisions" }).getByRole("listitem");
    await expect(events).toHaveCount(5);
    await expect(events.first()).toContainText("Raise the proxy's upstream timeout");
    await expect(events.nth(1)).toContainText("operator action");
  });

  test("opens an event from anywhere on it with the mouse", async ({ example }) => {
    const stage = await example("timeline/commit-log");
    const event = stage.getByRole("listitem").filter({ hasText: "Pin the kernel" });
    // The middle of the event's text block, away from the title itself.
    await event.locator(".yi-timeline__body").click();
    await expect(stage.getByRole("status")).toHaveText("Opened 4f2a9c1");
  });

  test("opens an event with the keyboard", async ({ page, example }) => {
    const stage = await example("timeline/commit-log");
    await page.keyboard.press("Tab");
    const title = stage.getByRole("button", { name: "Raise the proxy's upstream timeout" });
    await expect(title).toBeFocused();
    await expect(title).not.toHaveCSS("outline-style", "none");
    await page.keyboard.press("Enter");
    await expect(stage.getByRole("status")).toHaveText("Opened 9c41e07");
  });

  test("an event without a target offers no control", async ({ example }) => {
    const stage = await example("timeline/commit-log");
    await expect(stage.getByRole("button", { name: "Restart nginx on the edge hosts" })).toHaveCount(0);
  });

  test("content inside a clickable event keeps its own behaviour", async ({ page, example }) => {
    const stage = await example("timeline/commit-log");
    const event = stage.getByRole("listitem").filter({ hasText: "Pin the kernel" });
    await event.locator("time").hover();
    await expect(page.getByRole("tooltip")).toContainText("UTC");
  });

  test.describe("horizontal", () => {
    test("spreads its markers across the width", async ({ example }) => {
      const stage = await example("timeline/summary");
      const list = stage.getByRole("list", { name: "API incident, oldest first" });
      const events = list.getByRole("listitem");
      await expect(events).toHaveCount(4);
      const [box, first, last] = [
        (await list.boundingBox())!,
        (await events.first().locator(".yi-popover").boundingBox())!,
        (await events.last().locator(".yi-popover").boundingBox())!,
      ];
      expect(last.x - first.x).toBeGreaterThan(box.width / 2);
      expect(Math.abs(last.y - first.y)).toBeLessThan(1);
    });

    test("shows an event's detail on hover", async ({ page, example }) => {
      const stage = await example("timeline/summary");
      const tooltip = page.getByRole("tooltip");
      await expect(tooltip).toHaveCount(0);
      await stage.getByRole("listitem").nth(1).locator(".yi-popover").hover();
      await expect(tooltip).toContainText("Identified");
      await expect(tooltip).toContainText("A certificate on the regional load balancer expired.");
      await page.mouse.move(0, 0);
      await expect(tooltip).toHaveCount(0);
    });

    test("shows each event's detail on keyboard focus, and Escape dismisses it", async ({ page, example }) => {
      await example("timeline/summary");
      const tooltip = page.getByRole("tooltip");
      await page.keyboard.press("Tab");
      await expect(tooltip).toContainText("Investigating");
      await page.keyboard.press("Tab");
      await expect(tooltip).toContainText("Identified");
      await page.keyboard.press("Escape");
      await expect(tooltip).toHaveCount(0);
    });

    for (const theme of ["light", "dark"] as const) {
      test(`looks right with an event's detail shown: ${theme}`, async ({ page, example }) => {
        const stage = await example("timeline/summary", { theme });
        await stage.getByRole("listitem").nth(2).locator(".yi-popover").hover();
        await expect(page.getByRole("tooltip")).toBeVisible();
        await expect(page).toHaveScreenshot(["timeline", `summary-open-${theme}.png`], { clip: await around(page, stage) });
      });
    }
  });
});
