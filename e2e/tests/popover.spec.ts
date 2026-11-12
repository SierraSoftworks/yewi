import type { Locator, Page } from "@playwright/test";
import { expect, test, type Theme } from "../fixtures";

/** The area around an example that its popovers can reach. */
async function around(page: Page, stage: Locator) {
  const box = (await stage.boundingBox())!;
  const viewport = page.viewportSize()!;
  return { x: 0, y: 0, width: Math.min(box.x + box.width, viewport.width), height: box.y + box.height + 140 };
}

test.describe("Popover", () => {
  test("a press on the trigger toggles it, and a press outside closes it", async ({ page, example }) => {
    const stage = await example("popover/basic");
    const trigger = stage.getByRole("button", { name: "Deployment" });
    const popover = page.getByRole("group");

    await trigger.click();
    await expect(popover).toBeVisible();
    await expect(popover).toContainText("Deployed");
    await expect(popover).toContainText("Version 2.14.0");
    await expect(trigger).toHaveAttribute("aria-expanded", "true");

    // A press on the popover's own content keeps it open.
    await popover.getByText("eu-west-1").click();
    await expect(popover).toBeVisible();

    await trigger.click();
    await expect(popover).toBeHidden();
    await expect(trigger).toHaveAttribute("aria-expanded", "false");

    await trigger.click();
    await expect(popover).toBeVisible();
    await page.mouse.click(5, 5);
    await expect(popover).toBeHidden();
  });

  test("works from the keyboard: opens, tabs into the content, and Escape returns focus", async ({ page, example }) => {
    const stage = await example("popover/basic");
    const trigger = stage.getByRole("button", { name: "Deployment" });
    const popover = page.getByRole("group");

    await page.keyboard.press("Tab");
    await expect(trigger).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(popover).toBeVisible();

    await page.keyboard.press("Tab");
    await expect(page.getByRole("link", { name: "Release notes" })).toBeFocused();
    await page.keyboard.press("Escape");
    await expect(popover).toBeHidden();
    await expect(trigger).toBeFocused();

    // Tabbing on past the content closes it.
    await page.keyboard.press("Enter");
    await page.keyboard.press("Tab");
    await page.keyboard.press("Tab");
    await expect(stage.getByRole("button", { name: "About permissions" })).toBeFocused();
    await expect(popover).toBeHidden();
  });

  test("hover shows detail after a rest, stays while the popover is hovered and hides on leaving", async ({ page, example }) => {
    const stage = await example("popover/hover");
    const popover = page.getByRole("tooltip");

    await stage.getByRole("img", { name: "11:20: Failing" }).hover();
    await expect(popover).toContainText("Failing");
    await expect(popover).toContainText("62.5%");

    await popover.getByText("Availability").hover();
    await page.waitForTimeout(300);
    await expect(popover).toBeVisible();

    await stage.getByRole("img", { name: "11:50: Passing" }).hover();
    await expect(popover).toContainText("11:50 UTC");
    await expect(popover).toHaveCount(1);

    await page.mouse.move(5, 5);
    await expect(popover).toBeHidden();
  });

  test("hover detail reaches keyboard users as the trigger's description", async ({ page, example }) => {
    const stage = await example("popover/hover");
    const first = stage.getByRole("img", { name: "11:00: Passing" });

    await page.keyboard.press("Tab");
    await expect(first).toBeFocused();
    await expect(page.getByRole("tooltip")).toContainText("41 ms");
    await expect(first).toHaveAccessibleDescription(/Passing.*41 ms/);

    await page.keyboard.press("Tab");
    await expect(page.getByRole("tooltip")).toContainText("44 ms");
    await page.keyboard.press("Escape");
    await expect(page.getByRole("tooltip")).toBeHidden();
  });

  test("stays on screen and off its trigger at every placement", async ({ page, example }) => {
    const stage = await example("popover/placement");
    const viewport = page.viewportSize()!;

    for (const button of await stage.getByRole("button").all()) {
      await button.click();
      const popover = page.getByRole("group");
      await expect(popover).toBeVisible();
      const [surface, control] = [(await popover.boundingBox())!, (await button.boundingBox())!];

      expect(surface.x).toBeGreaterThanOrEqual(0);
      expect(surface.y).toBeGreaterThanOrEqual(0);
      expect(surface.x + surface.width).toBeLessThanOrEqual(viewport.width);
      const apart =
        surface.x >= control.x + control.width ||
        surface.x + surface.width <= control.x ||
        surface.y >= control.y + control.height ||
        surface.y + surface.height <= control.y;
      expect(apart, await button.innerText()).toBe(true);

      await page.keyboard.press("Escape");
      await expect(popover).toBeHidden();
    }
  });

  test("a controlled popover closes from the actions inside it", async ({ page, example }) => {
    const stage = await example("popover/actions");
    const popover = page.getByRole("group", { name: "Confirm" });
    const status = stage.getByRole("status");

    await stage.getByRole("button", { name: "Pause monitoring" }).click();
    await popover.getByRole("button", { name: "Cancel" }).click();
    await expect(popover).toBeHidden();
    await expect(status).toHaveText("Monitoring is running");

    await stage.getByRole("button", { name: "Pause monitoring" }).click();
    await popover.getByRole("button", { name: "Pause" }).click();
    await expect(popover).toBeHidden();
    await expect(status).toHaveText("Monitoring is paused");
    await expect(stage.getByRole("button", { name: "Resume monitoring" })).toBeVisible();
  });

  test("a focus popover is open while focus is in the input or the list", async ({ page, example }) => {
    const stage = await example("popover/focus");
    const input = stage.getByRole("searchbox", { name: "Filter probes" });
    const popover = page.getByRole("group", { name: "Filters" });

    await input.click();
    await expect(popover).toBeVisible();
    const [field, surface] = [(await input.boundingBox())!, (await popover.boundingBox())!];
    expect(surface.y).toBeGreaterThanOrEqual(field.y + field.height);
    expect(Math.abs(surface.x - field.x)).toBeLessThanOrEqual(1);

    await page.keyboard.press("Tab");
    await expect(popover.getByRole("button", { name: /status:/ })).toBeFocused();
    await page.keyboard.press("Tab");
    await page.keyboard.press("Enter");
    await expect(stage.getByRole("status")).toHaveText("Filtering by region:");
    await expect(popover).toBeVisible();

    await page.mouse.click(5, 5);
    await expect(popover).toBeHidden();
  });

  test("nested popovers close innermost first", async ({ page, example }) => {
    const stage = await example("popover/nested");
    const outer = page.getByRole("group", { name: "Probe" });
    const inner = page.getByText("Each region must fail twice");

    await stage.getByRole("button", { name: "Probe" }).click();
    await outer.getByRole("button", { name: "How failures are counted" }).click();
    await expect(inner).toBeVisible();

    // A press inside the inner popover is inside the outer one too.
    await inner.click();
    await expect(inner).toBeVisible();
    await expect(outer).toBeVisible();

    await page.keyboard.press("Escape");
    await expect(inner).toBeHidden();
    await expect(outer).toBeVisible();
    await page.keyboard.press("Escape");
    await expect(outer).toBeHidden();
  });

  for (const theme of ["light", "dark"] as Theme[]) {
    test(`looks right when open: ${theme}`, async ({ page, example }) => {
      const basic = await example("popover/basic", { theme });
      await basic.getByRole("button", { name: "Deployment" }).click();
      await expect(page.getByRole("group")).toBeVisible();
      await expect(page).toHaveScreenshot(["popover", `open-basic-${theme}.png`], { clip: await around(page, basic) });

      const hover = await example("popover/hover", { theme });
      await hover.getByRole("img", { name: "11:20: Failing" }).hover();
      await expect(page.getByRole("tooltip")).toBeVisible();
      await expect(page).toHaveScreenshot(["popover", `open-hover-${theme}.png`], { clip: await around(page, hover) });

      const focus = await example("popover/focus", { theme });
      await focus.getByRole("searchbox").click();
      await expect(page.getByRole("group", { name: "Filters" })).toBeVisible();
      await expect(page).toHaveScreenshot(["popover", `open-flush-${theme}.png`], { clip: await around(page, focus) });

      const actions = await example("popover/actions", { theme });
      await actions.getByRole("button", { name: "Pause monitoring" }).click();
      await expect(page.getByRole("group", { name: "Confirm" })).toBeVisible();
      await expect(page).toHaveScreenshot(["popover", `open-actions-${theme}.png`], { clip: await around(page, actions) });
    });
  }
});
