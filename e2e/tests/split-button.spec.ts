import { expect, test, type Theme } from "../fixtures";

const THEMES: Theme[] = ["light", "dark"];

test.describe("SplitButton", () => {
  test("the face runs its action without opening the menu", async ({ page, example }) => {
    const stage = await example("split-button/basic");

    await stage.getByRole("button", { name: "Deploy", exact: true }).click();
    await expect(stage.getByRole("status")).toHaveText("Deployed to production");
    await expect(page.getByRole("menu")).toBeHidden();
  });

  test("the chevron opens the other actions under the control's end", async ({ page, example }) => {
    const stage = await example("split-button/basic");
    const toggle = stage.getByRole("button", { name: "More ways to deploy" });
    const menu = page.getByRole("menu", { name: "More ways to deploy" });

    await toggle.click();
    await expect(menu).toBeVisible();
    await expect(toggle).toHaveAttribute("aria-expanded", "true");

    const [surface, control] = [(await menu.boundingBox())!, (await stage.getByRole("group").boundingBox())!];
    expect(surface.y).toBeGreaterThanOrEqual(control.y + control.height);
    expect(surface.x).toBeGreaterThanOrEqual(0);

    await menu.getByRole("menuitem", { name: "Deploy a canary" }).click();
    await expect(stage.getByRole("status")).toHaveText("Deployed a canary");
    await expect(menu).toBeHidden();
    await expect(toggle).toBeFocused();
  });

  test("works from the keyboard: the two halves are separate stops", async ({ page, example }) => {
    const stage = await example("split-button/basic");

    await page.keyboard.press("Tab");
    await expect(stage.getByRole("button", { name: "Deploy", exact: true })).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(stage.getByRole("status")).toHaveText("Deployed to production");

    await page.keyboard.press("Tab");
    await expect(stage.getByRole("button", { name: "More ways to deploy" })).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await expect(page.getByRole("menuitem", { name: "Deploy to staging" })).toBeFocused();
    await page.keyboard.press("End");
    await page.keyboard.press("Enter");
    await expect(stage.getByRole("status")).toHaveText("Rolled back");
    await expect(stage.getByRole("button", { name: "More ways to deploy" })).toBeFocused();
  });

  test("loading holds the action but not the menu", async ({ page, example }) => {
    const stage = await example("split-button/states");
    const sync = stage.getByRole("button", { name: "Sync", exact: true });

    // With nothing to cancel the item is disabled: a press does not choose it.
    await stage.getByRole("button", { name: "More actions" }).first().click();
    await page.getByRole("menuitem", { name: "Cancel sync" }).click({ force: true });
    await expect(page.getByRole("menu")).toBeVisible();
    await page.keyboard.press("Escape");

    await sync.click();
    await expect(stage.getByRole("status")).toHaveText("Syncing");
    await expect(sync).toHaveAttribute("aria-busy", "true");

    await stage.getByRole("button", { name: "More actions" }).first().click();
    await page.getByRole("menuitem", { name: "Cancel sync" }).click();
    await expect(stage.getByRole("status")).toHaveText("Idle");
  });

  test("disabled turns off both halves, and a lone action has no chevron", async ({ example }) => {
    const stage = await example("split-button/states");

    await expect(stage.getByRole("button", { name: "Disabled" })).toBeDisabled();
    await expect(stage.getByRole("button", { name: "More actions", exact: true })).toHaveCount(2);
    await expect(stage.getByRole("button", { name: "More actions", exact: true }).nth(1)).toBeDisabled();
    await expect(stage.getByRole("button", { name: "Only one action" })).toBeEnabled();
  });

  test("a disabled face leaves the menu usable", async ({ page, example }) => {
    const stage = await example("split-button/states");

    await expect(stage.getByRole("button", { name: "Demote" })).toBeDisabled();
    await stage.getByRole("button", { name: "More actions for this user" }).click();
    await expect(page.getByRole("menuitem", { name: "Remove" })).toBeVisible();
  });
});

// The example sweeps capture menus closed. These are the open states.
test.describe("SplitButton, open", () => {
  for (const theme of THEMES) {
    test(`looks right: ${theme}`, async ({ page, example }) => {
      await page.setViewportSize({ width: 640, height: 420 });

      let stage = await example("split-button/basic", { theme });
      await stage.getByRole("button", { name: "More ways to deploy" }).click();
      await page.mouse.move(5, 5);
      await expect(page).toHaveScreenshot(["split-button", `open-basic-${theme}-desktop.png`]);

      stage = await example("split-button/grouped", { theme });
      await stage.getByRole("button", { name: "More actions" }).click();
      await page.getByRole("menuitem", { name: "Reboot" }).hover();
      await expect(page).toHaveScreenshot(["split-button", `open-grouped-${theme}-desktop.png`]);
    });
  }
});
