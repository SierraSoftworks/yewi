import { expect, test, type Theme } from "../fixtures";

const THEMES: Theme[] = ["light", "dark"];

test.describe("UserMenu", () => {
  test("a press opens the menu with who is signed in, and choosing an item closes it", async ({ page, example }) => {
    const stage = await example("user-menu/basic");
    const button = stage.getByRole("button", { name: "Ada Lovelace" });
    const menu = page.getByRole("menu", { name: "Account" });

    await expect(button).toHaveAttribute("aria-haspopup", "menu");
    await expect(button).toHaveAttribute("aria-expanded", "false");
    await expect(menu).toBeHidden();

    await button.click();
    await expect(menu).toBeVisible();
    await expect(button).toHaveAttribute("aria-expanded", "true");
    await expect(menu.getByRole("menuitem")).toHaveText(["Profile", "Access tokens", "Settings", "Sign out"]);

    await menu.getByRole("menuitem", { name: "Access tokens" }).click();
    await expect(menu).toBeHidden();
    await expect(stage.getByRole("status")).toHaveText("Opened access tokens");
    await expect(button).toBeFocused();
  });

  test("works from the keyboard and signs out with Enter", async ({ page, example }) => {
    const stage = await example("user-menu/basic");
    const button = stage.getByRole("button", { name: "Ada Lovelace" });
    const item = (name: string) => page.getByRole("menuitem", { name });

    await page.keyboard.press("Tab");
    await expect(button).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(item("Profile")).toBeFocused();
    await page.keyboard.press("Escape");
    await expect(page.getByRole("menu")).toBeHidden();
    await expect(button).toBeFocused();
    await expect(stage.getByRole("status")).toHaveText("Signed in as Ada Lovelace");

    await page.keyboard.press("ArrowUp");
    await expect(item("Sign out")).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(stage.getByRole("status")).toHaveText("Signed out");
    await expect(page.getByRole("menu")).toBeHidden();
  });

  test("a press outside closes it", async ({ page, example }) => {
    const stage = await example("user-menu/basic");
    await stage.getByRole("button", { name: "Ada Lovelace" }).click();
    await expect(page.getByRole("menu")).toBeVisible();
    await page.mouse.click(5, 300);
    await expect(page.getByRole("menu")).toBeHidden();
  });

  test("sits at the end of an app bar, opens inside the screen and signs out", async ({ page, example }) => {
    const stage = await example("user-menu/app-bar");
    const viewport = page.viewportSize()!;
    const button = stage.getByRole("button", { name: "Grace Hopper" });

    await button.click();
    const menu = page.getByRole("menu", { name: "Account" });
    const [surface, trigger] = [(await menu.boundingBox())!, (await button.boundingBox())!];
    expect(surface.x).toBeGreaterThanOrEqual(0);
    expect(surface.x + surface.width).toBeLessThanOrEqual(viewport.width);
    expect(surface.y).toBeGreaterThanOrEqual(trigger.y + trigger.height);

    await menu.getByRole("menuitem", { name: "Sign out" }).click();
    await expect(stage.getByRole("status")).toHaveText("Signed out");
    await expect(stage.getByRole("button", { name: "Sign in" })).toBeVisible();
  });

  test("the button is the avatar alone, and still named after the person", async ({ page, example }) => {
    const stage = await example("user-menu/app-bar");
    const button = stage.getByRole("button", { name: "Grace Hopper" });

    expect((await button.boundingBox())!.width).toBeLessThan(48);
    await button.click();
    await expect(page.getByRole("menu")).toBeVisible();
    await expect(page.getByText("Grace Hopper", { exact: true })).toHaveCount(1);
    await expect(page.getByText("read only", { exact: true })).toBeVisible();
  });

  test("show_name puts the name beside the avatar, except on a narrow screen", async ({ page, example }) => {
    const stage = await example("user-menu/named");
    const button = stage.getByRole("button", { name: "Alan Turing" });
    const width = async () => (await button.boundingBox())!.width;

    await expect(button.getByText("Alan Turing")).toBeInViewport();
    expect(await width()).toBeGreaterThan(100);

    // The button already says who is signed in, so the menu does not repeat it.
    await button.click();
    await expect(page.getByRole("menu")).toBeVisible();
    await expect(page.getByText("Alan Turing")).toHaveCount(1);
    await page.keyboard.press("Escape");

    await page.setViewportSize({ width: 390, height: 700 });
    await expect.poll(width).toBeLessThan(48);
    await button.click();
    await expect(page.getByRole("menu")).toBeVisible();
    await expect(page.getByText("admin").last()).toBeVisible();
  });

  test("sign out can be a link, and options stay in step", async ({ page, example }) => {
    const stage = await example("user-menu/links");
    const button = stage.getByRole("button", { name: "Edsger Dijkstra" });

    await button.click();
    await expect(page.getByRole("menuitem", { name: "Sign out" })).toHaveAttribute("href", "#signed-out");
    await page.getByRole("menuitemcheckbox", { name: "Weekly digest" }).click();
    await expect(stage.getByRole("status")).toHaveText("Weekly digest off");

    await button.click();
    await expect(page.getByRole("menuitemcheckbox", { name: "Weekly digest" })).toHaveAttribute("aria-checked", "false");
  });
});

// The example sweeps capture menus closed. These are the open states.
test.describe("UserMenu, open", () => {
  for (const theme of THEMES) {
    test(`looks right: ${theme}`, async ({ page, example }) => {
      await page.setViewportSize({ width: 640, height: 420 });

      let stage = await example("user-menu/basic", { theme });
      await stage.getByRole("button", { name: "Ada Lovelace" }).click();
      await page.getByRole("menuitem", { name: "Access tokens" }).hover();
      await expect(page).toHaveScreenshot(["user-menu", `open-basic-${theme}-desktop.png`]);

      stage = await example("user-menu/app-bar", { theme });
      await stage.getByRole("button", { name: "Grace Hopper" }).click();
      await page.mouse.move(5, 300);
      await expect(page).toHaveScreenshot(["user-menu", `open-app-bar-${theme}-desktop.png`]);

      stage = await example("user-menu/links", { theme });
      await stage.getByRole("button", { name: "Edsger Dijkstra" }).click();
      await page.mouse.move(5, 300);
      await expect(page).toHaveScreenshot(["user-menu", `open-links-${theme}-desktop.png`]);
    });
  }
});
