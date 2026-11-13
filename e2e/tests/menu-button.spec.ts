import { expect, test, type Theme } from "../fixtures";

const THEMES: Theme[] = ["light", "dark"];

test.describe("MenuButton", () => {
  test("a press opens the menu, and choosing an item closes it and returns focus", async ({ page, example }) => {
    const stage = await example("menu-button/basic");
    const button = stage.getByRole("button", { name: "Actions" });
    const menu = page.getByRole("menu", { name: "Actions" });

    await expect(button).toHaveAttribute("aria-haspopup", "menu");
    await expect(button).toHaveAttribute("aria-expanded", "false");
    await expect(menu).toBeHidden();

    await button.click();
    await expect(menu).toBeVisible();
    await expect(button).toHaveAttribute("aria-expanded", "true");
    await expect(menu.getByRole("menuitem")).toHaveText(["Rename", "Duplicate", "Export", "Delete"]);

    await menu.getByRole("menuitem", { name: "Duplicate" }).click();
    await expect(menu).toBeHidden();
    await expect(stage.getByRole("status")).toHaveText("Duplicated the dashboard");
    await expect(button).toBeFocused();
    await expect(button).toHaveAttribute("aria-expanded", "false");
  });

  // The surface is the Popover's; its limits are MenuButton's. This held only
  // for components sorted after `popover` until tokens moved into `:where()`.
  test("the menu takes its corner and its limits from MenuButton, and scrolls when it is too long", async ({ page, example }) => {
    await page.setViewportSize({ width: 800, height: 150 });
    const stage = await example("menu-button/basic");
    await stage.getByRole("button", { name: "Actions" }).click();

    const menu = page.getByRole("menu", { name: "Actions" });
    await expect(menu).toBeVisible();
    const surface = await menu.evaluate((element) => {
      const surface = element.closest(".yi-menu-button__surface")!;
      const body = [surface, ...surface.querySelectorAll("*")].find((part) => part.scrollHeight > part.clientHeight + 1);
      const style = getComputedStyle(surface);
      const control = getComputedStyle(document.documentElement).getPropertyValue("--yi-radius-control").trim();
      return { radius: style.borderTopLeftRadius, control, scrolls: body !== undefined };
    });
    expect(surface.radius).toBe(surface.control);
    expect(surface.scrolls).toBe(true);
  });

  test("a second press, a press outside and Escape all close it", async ({ page, example }) => {
    const stage = await example("menu-button/basic");
    const button = stage.getByRole("button", { name: "Actions" });
    const menu = page.getByRole("menu");

    await button.click();
    await expect(menu).toBeVisible();
    await button.click();
    await expect(menu).toBeHidden();

    await button.click();
    await expect(menu).toBeVisible();
    await page.mouse.click(5, 5);
    await expect(menu).toBeHidden();

    await button.click();
    await page.keyboard.press("Escape");
    await expect(menu).toBeHidden();
    await expect(button).toBeFocused();
    await expect(stage.getByRole("status")).toHaveText("Nothing chosen yet");
  });

  test("works from the keyboard: opens on the first or last item and chooses with Enter", async ({ page, example }) => {
    const stage = await example("menu-button/basic");
    const button = stage.getByRole("button", { name: "Actions" });
    const item = (name: string) => page.getByRole("menuitem", { name });

    await page.keyboard.press("Tab");
    await expect(button).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(item("Rename")).toBeFocused();
    await page.keyboard.press("Escape");
    await expect(button).toBeFocused();

    await page.keyboard.press("ArrowUp");
    await expect(item("Delete")).toBeFocused();
    await page.keyboard.press("Escape");

    await page.keyboard.press("ArrowDown");
    await expect(item("Rename")).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Enter");
    await expect(stage.getByRole("status")).toHaveText("Exported the dashboard");
    await expect(page.getByRole("menu")).toBeHidden();
    await expect(button).toBeFocused();
  });

  test("tabbing out of the menu closes it", async ({ page, example }) => {
    const stage = await example("menu-button/variants");
    const menu = page.getByRole("menu");

    await stage.getByRole("button", { name: "Default" }).focus();
    await page.keyboard.press("Enter");
    await expect(menu).toBeVisible();
    await page.keyboard.press("Tab");
    await expect(menu).toBeHidden();
    await expect(stage.getByRole("button", { name: "Primary" })).toBeFocused();
  });

  test("the pointer and the keyboard highlight one item between them", async ({ page, example }) => {
    const stage = await example("menu-button/basic");
    await stage.getByRole("button", { name: "Actions" }).click();

    await page.getByRole("menuitem", { name: "Export" }).hover();
    await expect(page.getByRole("menuitem", { name: "Export" })).toBeFocused();
    await page.keyboard.press("ArrowUp");
    await expect(page.getByRole("menuitem", { name: "Duplicate" })).toBeFocused();
  });

  test("each row has its own menu, opened towards the row and kept on screen", async ({ page, example }) => {
    const stage = await example("menu-button/row-actions");
    const viewport = page.viewportSize()!;

    await stage.getByRole("button", { name: "Actions for web-2" }).click();
    const menu = page.getByRole("menu", { name: "Actions for web-2" });
    await expect(menu).toBeVisible();
    await expect(page.getByRole("menu")).toHaveCount(1);

    const [surface, button] = [
      (await menu.boundingBox())!,
      (await stage.getByRole("button", { name: "Actions for web-2" }).boundingBox())!,
    ];
    expect(surface.x).toBeGreaterThanOrEqual(0);
    expect(surface.x + surface.width).toBeLessThanOrEqual(viewport.width);
    expect(surface.y).toBeGreaterThanOrEqual(button.y + button.height);

    await menu.getByRole("menuitem", { name: "Drain" }).click();
    await expect(stage.getByRole("status")).toHaveText("Drained web-2");

    // Opening another row's menu closes the first.
    await stage.getByRole("button", { name: "Actions for worker-1" }).click();
    await stage.getByRole("button", { name: "Actions for web-1" }).click();
    await expect(page.getByRole("menu")).toHaveCount(1);
    await expect(page.getByRole("menu", { name: "Actions for web-1" })).toBeVisible();
  });

  test("an option stays in step with the state it controls", async ({ page, example }) => {
    const stage = await example("menu-button/grouped");
    const button = stage.getByRole("button", { name: "View" });

    await button.click();
    await expect(page.getByRole("group", { name: "Display" })).toBeVisible();
    await page.getByRole("menuitemcheckbox", { name: "Wrap long lines" }).click();
    await expect(stage.getByRole("status")).toHaveText("Relative times, wrapped");

    await button.click();
    await expect(page.getByRole("menuitemcheckbox", { name: "Wrap long lines" })).toHaveAttribute("aria-checked", "true");
  });

  test("a disabled button does not open", async ({ page, example }) => {
    const stage = await example("menu-button/variants");
    const button = stage.getByRole("button", { name: "Disabled" });

    await expect(button).toBeDisabled();
    await button.click({ force: true });
    await expect(page.getByRole("menu")).toBeHidden();
  });
});

// The example sweeps capture menus closed. These are the open states.
test.describe("MenuButton, open", () => {
  for (const theme of THEMES) {
    test(`looks right: ${theme}`, async ({ page, example }) => {
      await page.setViewportSize({ width: 640, height: 420 });

      let stage = await example("menu-button/basic", { theme });
      await stage.getByRole("button", { name: "Actions" }).click();
      await page.getByRole("menuitem", { name: "Duplicate" }).hover();
      await expect(page).toHaveScreenshot(["menu-button", `open-basic-${theme}-desktop.png`]);

      stage = await example("menu-button/grouped", { theme });
      await stage.getByRole("button", { name: "View" }).click();
      await page.mouse.move(5, 5);
      await expect(page).toHaveScreenshot(["menu-button", `open-grouped-${theme}-desktop.png`]);

      stage = await example("menu-button/row-actions", { theme });
      await stage.getByRole("button", { name: "Actions for web-2" }).click();
      await page.mouse.move(5, 5);
      await expect(page).toHaveScreenshot(["menu-button", `open-row-actions-${theme}-desktop.png`]);
    });
  }
});
