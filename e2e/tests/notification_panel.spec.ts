import type { Locator, Page } from "@playwright/test";
import { expect, test, type Theme } from "../fixtures";

/** The example and the panel hanging below it. */
async function around(page: Page, stage: Locator, panel: Locator) {
  const [box, surface] = [(await stage.boundingBox())!, (await panel.boundingBox())!];
  const x = Math.max(0, Math.min(box.x, surface.x - 16));
  const right = Math.min(page.viewportSize()!.width, Math.max(box.x + box.width, surface.x + surface.width + 16));
  return { x, y: box.y, width: right - x, height: surface.y + surface.height + 16 - box.y };
}

test.describe("NotificationPanel", () => {
  test("the bell opens the list, and a press outside closes it", async ({ page, example }) => {
    const stage = await example("notification_panel/basic");
    const bell = stage.getByRole("button", { name: "3 hosts need attention" });
    const panel = page.getByRole("group", { name: "Needs attention" });

    await expect(stage.getByRole("listitem")).toHaveCount(0);
    await bell.click();
    await expect(panel).toBeVisible();
    await expect(bell).toHaveAttribute("aria-expanded", "true");
    await expect(panel.getByRole("listitem")).toHaveCount(3);
    await expect(panel.getByRole("listitem").nth(1)).toContainText("Health check failing");
    await expect(panel.getByRole("listitem").nth(1)).toContainText("Plan is stale");

    // Reading the list does not close it.
    await panel.getByText("Agent offline for 12 minutes").click();
    await expect(panel).toBeVisible();

    await page.mouse.click(5, 5);
    await expect(panel).toBeHidden();
    await expect(bell).toHaveAttribute("aria-expanded", "false");
  });

  test("the panel stays on screen under the bell", async ({ page, example }) => {
    const stage = await example("notification_panel/basic");
    const bell = stage.getByRole("button", { name: "3 hosts need attention" });
    await bell.click();
    const panel = page.getByRole("group", { name: "Needs attention" });
    await expect(panel).toBeVisible();

    const [surface, control] = [(await panel.boundingBox())!, (await bell.boundingBox())!];
    expect(surface.x).toBeGreaterThanOrEqual(0);
    expect(surface.x + surface.width).toBeLessThanOrEqual(page.viewportSize()!.width);
    expect(surface.y).toBeGreaterThanOrEqual(control.y + control.height);
  });

  test("works from the keyboard, and Escape returns focus to the bell", async ({ page, example }) => {
    const stage = await example("notification_panel/basic");
    const bell = stage.getByRole("button", { name: "3 hosts need attention" });
    const panel = page.getByRole("group", { name: "Needs attention" });

    await page.keyboard.press("Tab");
    await expect(bell).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(panel).toBeVisible();

    await page.keyboard.press("Tab");
    await expect(panel.getByRole("link", { name: "web-03.eu-west-1" })).toBeFocused();
    await page.keyboard.press("Tab");
    await expect(panel.getByRole("link", { name: "db-01.eu-west-1" })).toBeFocused();

    await page.keyboard.press("Escape");
    await expect(panel).toBeHidden();
    await expect(bell).toBeFocused();
  });

  test("following a link in the list closes the panel", async ({ page, example }) => {
    const stage = await example("notification_panel/basic");
    await stage.getByRole("button", { name: "3 hosts need attention" }).click();
    const panel = page.getByRole("group", { name: "Needs attention" });

    await panel.getByRole("link", { name: "db-01.eu-west-1" }).click();
    await expect(panel).toBeHidden();
    await expect(page).toHaveURL(/#db-01$/);
  });

  test("an empty panel has no count and says why", async ({ page, example }) => {
    const stage = await example("notification_panel/empty");
    const bell = stage.getByRole("button", { name: "Nothing needs attention" });
    await expect(stage).toHaveText("");

    await bell.click();
    const panel = page.getByRole("group", { name: "Needs attention" });
    await expect(panel).toContainText("Every host is healthy");
    await expect(panel.getByRole("list")).toHaveCount(0);
  });

  test("an action in the foot clears the list and closes the panel", async ({ page, example }) => {
    const stage = await example("notification_panel/actions");
    const status = stage.getByRole("status");
    await expect(status).toHaveText("3 unread notifications");

    await stage.getByRole("button", { name: "3 unread notifications" }).click();
    const panel = page.getByRole("group", { name: "Notifications" });
    await expect(panel.getByRole("listitem")).toHaveCount(3);
    await panel.getByRole("button", { name: "Mark all as read" }).click();
    await expect(panel).toBeHidden();
    await expect(status).toHaveText("No unread notifications");

    await stage.getByRole("button", { name: "No unread notifications" }).click();
    await expect(panel).toContainText("You are all caught up.");
    await expect(panel.getByRole("button", { name: "Mark all as read" })).toHaveCount(0);
  });

  for (const theme of ["light", "dark"] as Theme[]) {
    test(`looks right when open: ${theme}`, async ({ page, example }) => {
      for (const [id, name] of [
        ["basic", "Needs attention"],
        ["empty", "Needs attention"],
        ["actions", "Notifications"],
      ]) {
        const stage = await example(`notification_panel/${id}`, { theme });
        await stage.getByRole("button").first().click();
        const panel = page.getByRole("group", { name });
        await expect(panel).toBeVisible();
        await expect(page).toHaveScreenshot(["notification_panel", `open-${id}-${theme}.png`], {
          clip: await around(page, stage, panel),
        });
      }
    });
  }
});
