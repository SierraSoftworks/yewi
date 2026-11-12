import { expect, test } from "../fixtures";

test.describe("Breadcrumbs", () => {
  test("is a landmark whose last crumb is the current page, not a link", async ({ example }) => {
    const stage = await example("breadcrumbs/basic");
    const nav = stage.getByRole("navigation", { name: "Breadcrumb" });

    await expect(nav.getByRole("listitem")).toHaveCount(3);
    await expect(nav.getByRole("link")).toHaveCount(2);
    await expect(nav.getByRole("link", { name: "Exceptions" })).toHaveAttribute("href", "#exceptions");
    await expect(nav.locator('[aria-current="page"]')).toHaveText("TimeoutError");
  });

  test("keeps separators out of the accessible names and the tab order", async ({ page, example }) => {
    const stage = await example("breadcrumbs/separators");
    const nav = stage.getByRole("navigation", { name: "Slash breadcrumb" });

    await expect(nav.getByRole("link", { name: "eu-west", exact: true })).toBeVisible();
    await page.keyboard.press("Tab");
    await expect(nav.getByRole("link", { name: "Fleet" })).toBeFocused();
    await expect(nav.getByRole("link", { name: "Fleet" })).not.toHaveCSS("outline-style", "none");
    await page.keyboard.press("Tab");
    await expect(nav.getByRole("link", { name: "eu-west" })).toBeFocused();
  });

  test("truncates a long label and stays inside its container", async ({ page, example }) => {
    await page.setViewportSize({ width: 390, height: 844 });
    const stage = await example("breadcrumbs/icons-and-long-labels");
    const long = stage.getByRole("link", { name: /Search and rescue/ });

    const fits = await long.evaluate((a) => a.getBoundingClientRect().right <= a.closest("nav")!.getBoundingClientRect().right);
    expect(fits).toBe(true);
    const clipped = await long.locator("span").evaluate((label) => label.scrollWidth > label.clientWidth);
    expect(clipped).toBe(true);
  });

  test("with a router, follows routes and carries the query", async ({ page, example }) => {
    const stage = await example("breadcrumbs/router");
    const url = page.url();
    const status = stage.getByRole("status");

    await expect(status).toHaveText("Route: /exceptions/TimeoutError");
    await expect(stage.getByRole("link", { name: "Exceptions" })).toHaveAttribute(
      "href",
      "/exceptions?period=7d&env=production",
    );
    await stage.getByRole("link", { name: "Exceptions" }).click();
    await expect(status).toHaveText("Route: /exceptions?period=7d&env=production");

    await stage.getByRole("link", { name: "Dashboard" }).click();
    await expect(status).toHaveText("Route: /?period=7d&env=production");
    await expect(stage.getByRole("link")).toHaveCount(1);
    expect(page.url()).toBe(url);
  });
});
