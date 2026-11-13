import { expect, test, type Theme } from "../fixtures";

const THEMES: Theme[] = ["light", "dark"];
const PHONE = { width: 390, height: 844 };

test.describe("AppShell", () => {
  test("the menu button hides and shows the sidebar beside the page", async ({ example }) => {
    const stage = await example("app-shell/basic");
    const menu = stage.getByRole("button", { name: "Navigation" });
    const nav = stage.getByRole("navigation", { name: "Sections" });

    await expect(nav).toBeVisible();
    await expect(menu).toHaveAttribute("aria-expanded", "true");

    await menu.focus();
    await stage.page().keyboard.press("Enter");
    await expect(nav).toBeHidden();
    await expect(menu).toHaveAttribute("aria-expanded", "false");

    await stage.page().keyboard.press("Enter");
    await expect(nav).toBeVisible();
    await expect(menu).toHaveAttribute("aria-expanded", "true");
  });

  test("without a sidebar the page takes the full width", async ({ example }) => {
    const stage = await example("app-shell/bar-only");
    const bar = (await stage.locator(".yi-app-bar").boundingBox())!;
    const heading = (await stage.getByRole("heading", { name: "Welcome" }).boundingBox())!;

    await expect(stage.getByRole("button", { name: "Navigation" })).toHaveCount(0);
    // The heading starts at the page's edge, not after an empty column, and
    // has the width to stay on one line.
    expect(heading.x - bar.x).toBeLessThan(64);
    expect(heading.width).toBeGreaterThan(bar.width / 2);
  });

  test("picking a destination marks it current and shows its page", async ({ example }) => {
    const stage = await example("app-shell/basic");
    const nav = stage.getByRole("navigation", { name: "Sections" });

    await nav.getByRole("button", { name: "Hosts" }).click();
    await expect(stage.getByRole("status")).toHaveText("Showing Hosts");
    await expect(nav.getByRole("button", { name: "Hosts" })).toHaveAttribute("aria-current", "true");
    await expect(nav.getByRole("button", { name: "Overview" })).not.toHaveAttribute("aria-current");
  });

  test("on a narrow screen the sidebar opens in a drawer that closes on a pick", async ({ page, example }) => {
    await page.setViewportSize(PHONE);
    const stage = await example("app-shell/basic");
    const menu = stage.getByRole("button", { name: "Navigation" });
    const drawer = page.getByRole("dialog", { name: "Navigation" });

    await expect(stage.getByRole("navigation", { name: "Sections" })).toBeHidden();
    await expect(menu).toHaveAttribute("aria-expanded", "false");

    await menu.click();
    await expect(drawer).toBeVisible();
    await expect(drawer.getByRole("navigation", { name: "Sections" })).toBeVisible();

    await drawer.getByRole("button", { name: "Rollouts" }).click();
    await expect(drawer).toBeHidden();
    await expect(stage.getByRole("status")).toHaveText("Showing Rollouts");
    await expect(menu).toBeFocused();

    await menu.click();
    await expect(drawer.getByRole("button", { name: "Rollouts" })).toHaveAttribute("aria-current", "true");
    await page.keyboard.press("Escape");
    await expect(drawer).toBeHidden();
    await expect(menu).toHaveAttribute("aria-expanded", "false");
  });

  test("widening the window closes the drawer and restores the sidebar", async ({ page, example }) => {
    await page.setViewportSize(PHONE);
    const stage = await example("app-shell/basic");
    await stage.getByRole("button", { name: "Navigation" }).click();
    await expect(page.getByRole("dialog", { name: "Navigation" })).toBeVisible();

    await page.setViewportSize({ width: 1280, height: 720 });
    await expect(page.getByRole("dialog", { name: "Navigation" })).toBeHidden();
    await expect(stage.getByRole("navigation", { name: "Sections" })).toBeVisible();
    await expect(stage.getByRole("button", { name: "Navigation" })).toHaveAttribute("aria-expanded", "true");
  });

  test("a hidden sidebar is remembered across visits", async ({ example }) => {
    let stage = await example("app-shell/grouped");
    const nav = () => stage.getByRole("navigation", { name: "Admin sections" });
    await expect(nav().getByRole("group", { name: "Clients" })).toBeVisible();

    await stage.getByRole("button", { name: "Navigation" }).click();
    await expect(nav()).toBeHidden();

    stage = await example("app-shell/grouped");
    await expect(nav()).toBeHidden();
    await expect(stage.getByRole("button", { name: "Navigation" })).toHaveAttribute("aria-expanded", "false");

    await stage.getByRole("button", { name: "Navigation" }).click();
    stage = await example("app-shell/grouped");
    await expect(nav()).toBeVisible();
  });

  test("the page area scrolls while the bar and sidebar stay put", async ({ example }) => {
    const stage = await example("app-shell/scrolling");
    const bar = stage.getByRole("button", { name: "Navigation" });
    const before = await bar.boundingBox();

    await stage.getByText("build-2").scrollIntoViewIfNeeded();
    await expect(stage.getByText("build-2")).toBeInViewport();
    expect(await bar.boundingBox()).toEqual(before);
    await expect(stage.getByRole("navigation", { name: "Sections" })).toBeInViewport();
  });

  test("a control in the page can drive the navigation", async ({ example }) => {
    const stage = await example("app-shell/custom-toggle");
    const nav = stage.getByRole("navigation", { name: "Sections" });

    await stage.getByRole("button", { name: "Hide navigation" }).click();
    await expect(nav).toBeHidden();
    await stage.getByRole("button", { name: "Show navigation" }).click();
    await expect(nav).toBeVisible();
  });
});

test.describe("AppShell, drawer open", () => {
  for (const theme of THEMES) {
    test(`looks right: ${theme}`, async ({ page, example }) => {
      await page.setViewportSize(PHONE);
      const stage = await example("app-shell/basic", { theme });
      await stage.getByRole("button", { name: "Navigation" }).click();
      await expect(page.getByRole("dialog", { name: "Navigation" })).toBeVisible();
      await expect(page).toHaveScreenshot(["app-shell", `open-basic-${theme}-phone.png`]);
    });
  }
});
