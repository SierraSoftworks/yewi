import { expect, test, type Theme } from "../fixtures";

const THEMES: Theme[] = ["light", "dark"];
const VIEWPORT = { width: 1280, height: 720 };

test.describe("Drawer", () => {
  test("slides in from the end edge at full height and returns focus on Escape", async ({ page, example }) => {
    const stage = await example("drawer/basic");
    const trigger = stage.getByRole("button", { name: "Show host" });
    const drawer = page.getByRole("dialog", { name: "Host pi-1" });

    await trigger.focus();
    await page.keyboard.press("Enter");
    await expect(drawer).toBeVisible();
    expect(await drawer.evaluate((el) => el.matches(":modal"))).toBe(true);
    await expect.poll(async () => (await drawer.boundingBox())!.x + (await drawer.boundingBox())!.width).toBe(VIEWPORT.width);
    const box = (await drawer.boundingBox())!;
    expect(box.y).toBe(0);
    expect(box.height).toBe(VIEWPORT.height);

    await page.keyboard.press("Escape");
    await expect(drawer).toBeHidden();
    await expect(trigger).toBeFocused();
  });

  test("closes from the backdrop and the close button, and runs a footer action", async ({ page, example }) => {
    const stage = await example("drawer/basic");
    const trigger = stage.getByRole("button", { name: "Show host" });
    const drawer = page.getByRole("dialog", { name: "Host pi-1" });

    await trigger.click();
    await drawer.getByText("It serves 3 workloads").click();
    await expect(drawer).toBeVisible();
    await page.mouse.click(8, 8);
    await expect(drawer).toBeHidden();

    await trigger.click();
    await drawer.getByRole("button", { name: "Close" }).first().click();
    await expect(drawer).toBeHidden();
    await expect(stage.getByRole("status")).toHaveText("pi-1 is in service");

    await trigger.click();
    await drawer.getByRole("button", { name: "Drain host" }).click();
    await expect(drawer).toBeHidden();
    await expect(stage.getByRole("status")).toHaveText("pi-1 is draining");
  });

  test("opens from the start edge as labelled navigation", async ({ page, example }) => {
    const stage = await example("drawer/navigation");
    const trigger = stage.getByRole("button", { name: "Menu" });
    const drawer = page.getByRole("dialog", { name: "Navigation" });

    await trigger.click();
    await expect(drawer).toBeVisible();
    await expect.poll(async () => (await drawer.boundingBox())!.x).toBe(0);
    await expect(drawer.getByRole("button", { name: "Overview" })).toBeFocused();

    await page.keyboard.press("Tab");
    await page.keyboard.press("Enter");
    await expect(drawer).toBeHidden();
    await expect(stage.getByRole("status")).toHaveText("Showing Hosts");
    await expect(trigger).toBeFocused();
  });

  test("leaves part of the page visible on a phone", async ({ page, example }) => {
    await page.setViewportSize({ width: 390, height: 844 });
    const stage = await example("drawer/basic");
    await stage.getByRole("button", { name: "Show host" }).click();
    const drawer = page.getByRole("dialog", { name: "Host pi-1" });
    await expect.poll(async () => (await drawer.boundingBox())!.x + (await drawer.boundingBox())!.width).toBe(390);
    const box = (await drawer.boundingBox())!;
    expect(box.x).toBeGreaterThan(0);
    await expect(drawer.getByRole("button", { name: "Drain host" })).toBeInViewport();
  });

  test("a confirmation stacks above it and Escape closes one layer at a time", async ({ page, example }) => {
    const stage = await example("drawer/nested");
    const drawer = page.getByRole("dialog", { name: "Project Website" });
    const confirm = page.getByRole("alertdialog", { name: "Delete project Website?" });
    const remove = drawer.getByRole("button", { name: "Delete project" }).first();

    await stage.getByRole("button", { name: "Manage project" }).click();
    await remove.click();
    await expect(confirm).toBeVisible();

    await page.keyboard.press("Escape");
    await expect(confirm).toBeHidden();
    await expect(drawer).toBeVisible();
    await expect(remove).toBeFocused();

    await remove.click();
    await confirm.getByRole("button", { name: "Delete project" }).click();
    await expect(confirm).toBeHidden();
    await expect(drawer).toBeHidden();
    await expect(stage.getByRole("status")).toHaveText("Project Website was deleted");
  });
});

test.describe("Drawer, open", () => {
  for (const theme of THEMES) {
    test(`looks right: ${theme}`, async ({ page, example }) => {
      await page.setViewportSize({ width: 1280, height: 800 });

      let stage = await example("drawer/basic", { theme });
      await stage.getByRole("button", { name: "Show host" }).click();
      await expect(page).toHaveScreenshot(["drawer", `open-basic-${theme}-desktop.png`]);

      stage = await example("drawer/navigation", { theme });
      await stage.getByRole("button", { name: "Menu" }).click();
      await expect(page).toHaveScreenshot(["drawer", `open-navigation-${theme}-desktop.png`]);

      stage = await example("drawer/nested", { theme });
      await stage.getByRole("button", { name: "Manage project" }).click();
      await page.getByRole("dialog").getByRole("button", { name: "Delete project" }).click();
      await expect(page).toHaveScreenshot(["drawer", `open-nested-${theme}-desktop.png`]);

      await page.setViewportSize({ width: 390, height: 844 });
      stage = await example("drawer/basic", { theme });
      await stage.getByRole("button", { name: "Show host" }).click();
      await expect(page).toHaveScreenshot(["drawer", `open-basic-${theme}-phone.png`]);
    });
  }
});
