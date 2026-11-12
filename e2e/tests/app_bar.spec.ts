import { expect, test } from "../fixtures";

test.describe("AppBar", () => {
  test("lays out brand, content and actions from start to end", async ({ page, example }) => {
    await page.setViewportSize({ width: 1280, height: 720 });
    const stage = await example("app-bar/slots");
    const x = async (name: string | RegExp, role: "button" | "link" = "link") =>
      (await stage.getByRole(role, { name }).first().boundingBox())!.x;

    const brand = (await stage.getByText("Grey", { exact: true }).boundingBox())!.x;
    expect(brand).toBeLessThan(await x("Dashboards"));
    expect(await x("Settings")).toBeLessThan(await x("Notifications", "button"));
    await expect(stage.getByRole("button", { name: "Navigation" })).toHaveCount(0);
  });

  test("its content is reachable and operable by keyboard", async ({ page, example }) => {
    await page.setViewportSize({ width: 1280, height: 720 });
    const stage = await example("app-bar/slots");
    await stage.getByRole("link", { name: "Dashboards" }).focus();
    await page.keyboard.press("Tab");
    await page.keyboard.press("Enter");
    await expect(stage.getByRole("status")).toHaveText("Showing Alerts");
    await expect(stage.getByRole("link", { name: "Alerts" })).toHaveAttribute("aria-current", "page");
  });

  test("on a phone nothing in the bar is clipped or overlaps", async ({ page, example }) => {
    await page.setViewportSize({ width: 390, height: 720 });
    const stage = await example("app-bar/slots");
    const bar = stage.locator("header");
    const box = (await bar.boundingBox())!;

    // The bar does not scroll in either direction, and what it shows sits inside it.
    expect(await bar.evaluate((el) => el.scrollWidth <= el.clientWidth && el.scrollHeight <= el.clientHeight + 1)).toBe(true);
    for (const visible of [stage.getByText("Grey", { exact: true }), stage.getByRole("button", { name: "Menu" })]) {
      const inner = (await visible.boundingBox())!;
      expect(inner.y).toBeGreaterThanOrEqual(box.y);
      expect(inner.y + inner.height).toBeLessThanOrEqual(box.y + box.height);
      expect(inner.x + inner.width).toBeLessThanOrEqual(box.x + box.width);
    }
    await expect(stage.getByRole("link", { name: "Alerts" })).toBeHidden();

    await stage.getByRole("button", { name: "Menu" }).click();
    await expect(page.getByRole("group", { name: "Menu" }).getByRole("button", { name: "Notifications" })).toBeVisible();
    await page.getByRole("menuitem", { name: "Alerts" }).click();
    await expect(stage.getByRole("status")).toHaveText("Showing Alerts");
  });

  test("the brand links home when given an href", async ({ example }) => {
    const stage = await example("app-bar/basic");
    await expect(stage.getByRole("link", { name: "Buckle overview" })).toHaveAttribute("href", "#");
  });
});
