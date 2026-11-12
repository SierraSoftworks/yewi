import { expect, test } from "../fixtures";

test.describe("Map", () => {
  test("reports the marker under a click, and a click on bare map", async ({ example }) => {
    const stage = await example("map/markers");
    const map = stage.getByRole("region", { name: "Map" });
    const status = stage.getByRole("status");
    const box = (await map.boundingBox())!;

    // The example is centred on Frankfurt.
    await map.click({ position: { x: box.width / 2, y: box.height / 2 } });
    await expect(status).toHaveText("Selected fra");

    await map.click({ position: { x: 24, y: box.height / 2 } });
    await expect(status).toHaveText("Nothing selected");
  });

  test("the sidebar list selects a feature and opens its panel", async ({ example }) => {
    const stage = await example("map/layout");
    const row = stage.getByRole("button", { name: /Stockholm/ });

    await row.click();
    await expect(row).toHaveAttribute("aria-current", "true");
    const panel = stage.getByRole("region", { name: "Stockholm" });
    await expect(panel).toContainText("3 nodes");

    await panel.getByRole("button", { name: "Close" }).click();
    await expect(panel).toHaveCount(0);
    await expect(row).not.toHaveAttribute("aria-current", "true");
  });

  test("the sidebar folds away and back with the keyboard", async ({ page, example }) => {
    const stage = await example("map/layout");
    const row = stage.getByRole("button", { name: /Frankfurt/ });

    // A <summary> has no role of its own to locate it by.
    await stage.locator("summary", { hasText: "On the map" }).focus();
    await page.keyboard.press("Enter");
    await expect(row).toBeHidden();
    await page.keyboard.press("Enter");
    await expect(row).toBeVisible();
  });

  test("toolbar tools are toggles, and arrow keys move between them", async ({ page, example }) => {
    const stage = await example("map/layout");
    const toolbar = stage.getByRole("toolbar", { name: "Map tools" });
    const select = toolbar.getByRole("button", { name: "Select" });
    const pin = toolbar.getByRole("button", { name: "Place a marker" });

    await expect(select).toHaveAttribute("aria-pressed", "true");
    await pin.click();
    await expect(pin).toHaveAttribute("aria-pressed", "true");
    await expect(select).toHaveAttribute("aria-pressed", "false");
    await expect(stage.getByRole("status")).toHaveText("Click the map to place a marker.");

    await page.keyboard.press("ArrowRight");
    await expect(toolbar.getByRole("button", { name: "Fit everything" })).toBeFocused();
    await page.keyboard.press("Home");
    await expect(select).toBeFocused();
    await expect(select).not.toHaveCSS("outline-style", "none");
  });

  test("zooms with the buttons and with the keyboard", async ({ page, example }) => {
    const stage = await example("map/live");
    const status = stage.getByRole("status");

    await stage.getByRole("button", { name: "Zoom in" }).click();
    await expect(status).toContainText("zoom 4.6");
    await stage.getByRole("button", { name: "Zoom out" }).click();
    await expect(status).toContainText("zoom 3.6");

    await stage.getByRole("region", { name: "Map" }).focus();
    await page.keyboard.press("Equal");
    await expect(status).toContainText("zoom 4.6");
  });

  test("the handle moves the camera and replaces a layer's data", async ({ example }) => {
    const stage = await example("map/live");
    const status = stage.getByRole("status");

    await stage.getByRole("button", { name: "Next fix" }).click();
    await expect(status).toContainText("Fix 2 of 4");

    await stage.getByRole("button", { name: "Centre on Aurora" }).click();
    await expect(status).toContainText("centred on 51.5, -0.1 at zoom 5.0");
  });

  test("redraws in the new colours when the theme changes", async ({ page, example }) => {
    const stage = await example("map/markers", { theme: "light" });
    const map = stage.getByRole("region", { name: "Map" });
    const light = await map.screenshot();

    await page.evaluate(() => document.documentElement.setAttribute("data-theme", "dark"));
    await expect(async () => expect(Buffer.compare(light, await map.screenshot())).not.toBe(0)).toPass();
  });

  test("a fixed locator has no controls and ignores the pointer", async ({ page, example }) => {
    const stage = await example("map/locator");
    const map = stage.getByRole("region", { name: "Office location: Cape Town" });

    await expect(stage.getByRole("button")).toHaveCount(0);
    const before = await map.screenshot();
    await map.hover();
    await page.mouse.wheel(0, -600);
    await page.waitForTimeout(400);
    expect(Buffer.compare(before, await map.screenshot())).toBe(0);
  });
});
