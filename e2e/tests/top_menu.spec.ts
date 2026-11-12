import { expect, test } from "../fixtures";

test.describe("TopMenu", () => {
  test("is a navigation landmark whose current link follows the page", async ({ example }) => {
    const stage = await example("top-menu/basic");
    const nav = stage.getByRole("navigation", { name: "Main" });
    const link = (name: string) => nav.getByRole("link", { name });

    await expect(link("Workflows")).toHaveAttribute("aria-current", "page");
    await link("Activity").click();
    await expect(stage.getByRole("status")).toHaveText("Showing Activity");
    await expect(link("Activity")).toHaveAttribute("aria-current", "page");
    await expect(link("Workflows")).not.toHaveAttribute("aria-current");
    await expect(link("Docs")).toHaveAttribute("target", "_blank");
  });

  test("links reach down to the lower edge of the bar they sit in", async ({ example }) => {
    const stage = await example("top-menu/basic");
    const bar = (await stage.locator("header").boundingBox())!;
    const link = (await stage.getByRole("link", { name: "Workflows" }).boundingBox())!;

    expect(link.y + link.height).toBeCloseTo(bar.y + bar.height, 0);
    expect(link.y).toBeCloseTo(bar.y, 0);
  });

  test("a group opens its destinations as a menu and closes when one is chosen", async ({ page, example }) => {
    const stage = await example("top-menu/groups");
    const fleet = stage.getByRole("button", { name: "Fleet" });
    const menu = page.getByRole("menu", { name: "Fleet" });

    await expect(fleet).toHaveAttribute("aria-expanded", "false");
    await fleet.click();
    await expect(menu).toBeVisible();
    await expect(fleet).toHaveAttribute("aria-expanded", "true");
    await expect(menu.getByRole("menuitem")).toHaveText(["Nodes", "Packages", "Rollouts"]);

    await menu.getByRole("menuitem", { name: "Packages" }).click();
    await expect(menu).toBeHidden();
    await expect(stage.getByRole("status")).toHaveText("Opened Packages");
    await expect(fleet).toBeFocused();
  });

  test("a group works from the keyboard", async ({ page, example }) => {
    const stage = await example("top-menu/groups");
    const admin = stage.getByRole("button", { name: "Admin" });
    const item = (name: string) => page.getByRole("menuitem", { name });

    await admin.focus();
    await page.keyboard.press("ArrowDown");
    await expect(item("Accounts")).toBeFocused();
    await page.keyboard.press("Escape");
    await expect(page.getByRole("menu")).toBeHidden();
    await expect(admin).toBeFocused();

    await page.keyboard.press("ArrowUp");
    await expect(item("Audit log")).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(stage.getByRole("status")).toHaveText("Opened Audit log");
    await expect(admin).toBeFocused();
  });

  test("collapsed, one button opens the actions and every destination", async ({ page, example }) => {
    const stage = await example("top-menu/collapsed");
    const button = stage.getByRole("navigation").getByRole("button", { name: "Menu" });
    const dropdown = page.getByRole("group", { name: "Menu" });
    const menu = dropdown.getByRole("menu", { name: "Main" });

    await expect(stage.getByRole("link")).toHaveCount(0);
    await expect(stage.getByText("live")).toBeHidden();
    await button.click();
    await expect(button).toHaveAttribute("aria-expanded", "true");
    await expect(menu.getByRole("menuitem", { name: "Services" })).toBeFocused();

    // The actions are a row above the destinations, and using one leaves the dropdown open.
    const live = (await dropdown.getByText("live").boundingBox())!;
    const first = (await menu.getByRole("menuitem", { name: "Services" }).boundingBox())!;
    expect(live.y + live.height).toBeLessThanOrEqual(first.y);
    await dropdown.getByRole("button", { name: /theme/i }).click();
    await expect(menu).toBeVisible();

    await expect(menu.getByRole("menuitem")).toHaveText(["Services", "Incidents", "Accounts", "Audit log"]);
    await expect(menu.getByRole("menuitem", { name: "Services" })).toHaveAttribute("aria-current", "page");
    // Only the current destination carries the accent edge.
    const edge = (name: string) => menu.getByRole("menuitem", { name }).evaluate((el) => getComputedStyle(el).boxShadow);
    expect(await edge("Services")).not.toBe("none");
    expect(await edge("Incidents")).toBe("none");
    await expect(menu.getByRole("group", { name: "Admin" })).toBeVisible();

    await menu.getByRole("menuitem", { name: "Incidents" }).click();
    await expect(menu).toBeHidden();
    await expect(stage.getByRole("status")).toHaveText("Opened Incidents");
    await expect(button).toBeFocused();
  });

  test("the row gives way to the menu button when there is no room for it", async ({ page, example }) => {
    await page.setViewportSize({ width: 1280, height: 720 });
    const stage = await example("top-menu/basic");
    const nav = stage.getByRole("navigation", { name: "Main" });
    const button = nav.getByRole("button", { name: "Menu" });

    await expect(nav.getByRole("link", { name: "Activity" })).toBeVisible();
    await expect(nav.getByRole("img", { name: "Ada Lovelace" })).toBeVisible();
    await expect(button).toBeHidden();

    await page.setViewportSize({ width: 390, height: 720 });
    await expect(nav.getByRole("link", { name: "Activity" })).toBeHidden();
    await expect(nav.getByRole("img", { name: "Ada Lovelace" })).toBeHidden();
    await button.click();
    await expect(page.getByRole("group", { name: "Menu" }).getByRole("img", { name: "Ada Lovelace" })).toBeVisible();
    await page.getByRole("menuitem", { name: "Activity" }).click();
    await expect(stage.getByRole("status")).toHaveText("Showing Activity");
  });

  test("route targets navigate client-side and mark themselves current", async ({ page, example }) => {
    const stage = await example("top-menu/routed");
    const url = page.url();
    const activity = stage.getByRole("link", { name: "Activity" });

    await expect(stage.getByRole("link", { name: "Workflows" })).toHaveAttribute("aria-current", "page");
    await activity.click();
    await expect(stage.getByRole("status")).toHaveText("Route: /activity");
    await expect(activity).toHaveAttribute("aria-current", "page");
    await expect(stage.getByRole("link", { name: "Workflows" })).not.toHaveAttribute("aria-current");
    expect(page.url()).toBe(url);
  });
});
