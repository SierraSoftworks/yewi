import type { Locator, Page } from "@playwright/test";
import { expect, test, type Theme } from "../fixtures";

/** The area around an example that its tooltips can reach. */
async function around(page: Page, stage: Locator) {
  const box = (await stage.boundingBox())!;
  const viewport = page.viewportSize()!;
  return { x: 0, y: 0, width: Math.min(box.x + box.width, viewport.width), height: box.y + box.height + 48 };
}

test.describe("Tooltip", () => {
  test("shows after the pointer rests and hides when it leaves", async ({ page, example }) => {
    const stage = await example("tooltip/basic");
    const tooltip = page.getByRole("tooltip");

    await stage.getByRole("button", { name: "Retry" }).hover();
    await expect(tooltip).toBeHidden();
    await expect(tooltip).toHaveText("Runs every check again from the start");

    await page.mouse.move(600, 500);
    await expect(tooltip).toBeHidden();
  });

  test("shows at once on keyboard focus, describes the control, and Escape dismisses it", async ({ page, example }) => {
    const stage = await example("tooltip/basic");
    const retry = stage.getByRole("button", { name: "Retry" });
    const tooltip = page.getByRole("tooltip");

    await page.keyboard.press("Tab");
    await expect(retry).toBeFocused();
    await expect(tooltip).toBeVisible();
    await expect(retry).toHaveAccessibleDescription("Runs every check again from the start");

    await page.keyboard.press("Escape");
    await expect(tooltip).toBeHidden();
    await expect(retry).toBeFocused();
    await expect(retry).toHaveAccessibleDescription("");

    await page.keyboard.press("Tab");
    await expect(tooltip).toHaveText("Copy the deploy key");
  });

  test("goes away when the control is pressed, and a click does not bring it back", async ({ page, example }) => {
    const stage = await example("tooltip/basic");
    const retry = stage.getByRole("button", { name: "Retry" });

    await retry.hover();
    await expect(page.getByRole("tooltip")).toBeVisible();
    await retry.click();
    await expect(page.getByRole("tooltip")).toBeHidden();
  });

  test("stays on screen and off its control at every placement", async ({ page, example }) => {
    const stage = await example("tooltip/placement");
    const viewport = page.viewportSize()!;

    for (const button of await stage.getByRole("button").all()) {
      // A tooltip beside one button may cover the next: leave it first.
      await page.mouse.move(0, 0);
      await button.hover();
      const tooltip = page.getByRole("tooltip");
      await expect(tooltip).toBeVisible();
      const [tip, control] = [(await tooltip.boundingBox())!, (await button.boundingBox())!];

      expect(tip.x).toBeGreaterThanOrEqual(0);
      expect(tip.y).toBeGreaterThanOrEqual(0);
      expect(tip.x + tip.width).toBeLessThanOrEqual(viewport.width);
      const apart =
        tip.x >= control.x + control.width ||
        tip.x + tip.width <= control.x ||
        tip.y >= control.y + control.height ||
        tip.y + tip.height <= control.y;
      expect(apart, await button.innerText()).toBe(true);
    }
  });

  test("can be hovered itself, so links inside it can be followed", async ({ page, example }) => {
    const stage = await example("tooltip/rich");
    const link = page.getByRole("link", { name: "Open the runbook" });

    await stage.getByRole("button", { name: "Rollout blocked" }).hover();
    await expect(link).toBeVisible();
    await link.hover();
    await page.waitForTimeout(300);
    await expect(link).toBeVisible();
    await link.click();
    await expect(page).toHaveURL(/#runbook$/);
  });

  test("shows only one tooltip at a time", async ({ page, example }) => {
    const stage = await example("tooltip/rich");

    await page.keyboard.press("Tab");
    await expect(page.getByRole("tooltip")).toContainText("12:00:00 UTC");
    await stage.getByRole("button", { name: "Rollout blocked" }).hover();
    await expect(page.getByRole("tooltip")).toContainText("Blocked");
    await expect(page.getByRole("tooltip")).toHaveCount(1);
  });

  test("reaches content that cannot take focus, and disabled controls", async ({ page, example }) => {
    const stage = await example("tooltip/unfocusable");
    const tooltip = page.getByRole("tooltip");

    await page.keyboard.press("Tab");
    await expect(tooltip).toHaveText("Only members of the release team can see this host");
    await page.keyboard.press("Tab");
    await expect(tooltip).toHaveText("deploy-eu-west-1-production-canary-7f3a9c");
    await page.keyboard.press("Escape");

    await stage.getByRole("button", { name: "Approve" }).hover({ force: true });
    await expect(tooltip).toHaveText("Two approvals are needed and one has been given");
  });

  test("open holds it shown whatever the pointer does", async ({ page, example }) => {
    const stage = await example("tooltip/controlled");
    const tooltip = page.getByRole("tooltip");

    await stage.getByRole("button", { name: "Copy key" }).click();
    await expect(stage.getByRole("status")).toHaveText("Key copied");
    await page.mouse.move(600, 500);
    await expect(tooltip).toHaveText("Copied");

    await stage.getByRole("button", { name: "Reset" }).click();
    await expect(tooltip).toBeHidden();
  });

  for (const theme of ["light", "dark"] as Theme[]) {
    test(`looks right when shown: ${theme}`, async ({ page, example }) => {
      const stage = await example("tooltip/variants", { theme });
      for (const name of ["Surface", "Surface, arrow", "Dark", "Dark, arrow"]) {
        await stage.getByRole("button", { name, exact: true }).hover();
        await expect(page.getByRole("tooltip")).toBeVisible();
        const file = `shown-${name.toLowerCase().replace(", ", "-")}-${theme}.png`;
        await expect(page).toHaveScreenshot(["tooltip", file], { clip: await around(page, stage) });
      }

      const rich = await example("tooltip/rich", { theme });
      await rich.getByRole("button", { name: "Rollout blocked" }).hover();
      await expect(page.getByRole("tooltip")).toBeVisible();
      await expect(page).toHaveScreenshot(["tooltip", `shown-rich-${theme}.png`], { clip: await around(page, rich) });
    });
  }
});
