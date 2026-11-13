import { expect, test } from "../fixtures";

test.describe("Tabs", () => {
  test("selects a tab with the mouse and shows its panel", async ({ example }) => {
    const stage = await example("tabs/basic");
    const devices = stage.getByRole("tab", { name: "Devices" });

    await expect(stage.getByRole("tab", { name: "Profile" })).toHaveAttribute("aria-selected", "true");
    await devices.click();

    await expect(devices).toHaveAttribute("aria-selected", "true");
    await expect(stage.getByRole("tabpanel", { name: "Devices" })).toContainText("Phones and radios");
    await expect(stage.getByRole("tabpanel")).toHaveCount(1);
    await expect(stage.getByRole("status")).toHaveText("Showing devices");
  });

  test("is one tab stop, and arrow keys, Home and End move the selection", async ({ page, example }) => {
    const stage = await example("tabs/basic");
    const tab = (name: string) => stage.getByRole("tab", { name });

    await page.keyboard.press("Tab");
    await expect(tab("Profile")).toBeFocused();
    await expect(tab("Profile")).not.toHaveCSS("outline-style", "none");

    await page.keyboard.press("ArrowRight");
    await expect(tab("Devices")).toBeFocused();
    await expect(tab("Devices")).toHaveAttribute("aria-selected", "true");

    await page.keyboard.press("End");
    await expect(tab("Credentials")).toBeFocused();
    await page.keyboard.press("ArrowRight");
    await expect(tab("Profile")).toBeFocused();
    await page.keyboard.press("ArrowLeft");
    await expect(tab("Credentials")).toHaveAttribute("aria-selected", "true");
    await page.keyboard.press("Home");
    await expect(stage.getByRole("status")).toHaveText("Showing profile");

    await page.keyboard.press("Tab");
    await expect(stage.getByRole("tabpanel", { name: "Profile" })).toBeFocused();
  });

  test("a shared panel follows the selected tab", async ({ page, example }) => {
    const stage = await example("tabs/shared-panel");
    const panel = stage.getByRole("tabpanel");

    await expect(panel).toHaveAccessibleName("Pages");
    await expect(panel).toContainText("/pricing");

    await stage.getByRole("tab", { name: "Pages" }).focus();
    await page.keyboard.press("ArrowRight");
    await expect(stage.getByRole("tabpanel")).toHaveAccessibleName("Browsers");
    await expect(stage.getByRole("tabpanel")).toContainText("Firefox");
    await expect(stage.getByRole("tabpanel")).toHaveCount(1);
  });

  test("skips disabled tabs", async ({ page, example }) => {
    const stage = await example("tabs/icons-and-counts");
    const disabled = stage.getByRole("tab", { name: "Config package" });

    await expect(disabled).toBeDisabled();
    await stage.getByRole("tab", { name: "Credentials" }).focus();
    await page.keyboard.press("ArrowRight");
    await expect(stage.getByRole("tab", { name: "Profile" })).toBeFocused();
  });

  test("manual tabs wait for Enter, and kept panels keep their state", async ({ page, example }) => {
    const stage = await example("tabs/controlled");
    const status = stage.getByRole("status");

    await stage.getByRole("button", { name: "Open notes" }).click();
    await expect(stage.getByRole("tab", { name: "Notes" })).toHaveAttribute("aria-selected", "true");
    await stage.getByRole("textbox").fill("Paged the on-call");

    await stage.getByRole("tab", { name: "Notes" }).focus();
    await page.keyboard.press("ArrowLeft");
    await expect(stage.getByRole("tab", { name: "Timeline" })).toBeFocused();
    await expect(status).toHaveText("Selected: notes");
    await page.keyboard.press("Enter");
    await expect(status).toHaveText("Selected: timeline");
    await expect(stage.getByRole("tabpanel", { name: "Timeline" })).toBeVisible();

    await stage.getByRole("tab", { name: "Notes" }).click();
    await expect(stage.getByRole("textbox")).toHaveValue("Paged the on-call");
  });

  test("as navigation, is a landmark of links with the current page marked", async ({ example }) => {
    const stage = await example("tabs/navigation");
    const nav = stage.getByRole("navigation", { name: "Settings sections" });

    await expect(nav.getByRole("link")).toHaveCount(4);
    await expect(nav.getByRole("link", { name: "General" })).toHaveAttribute("aria-current", "page");
    await expect(nav.getByRole("link", { name: /Members/ })).toHaveAttribute("href", "#members");
  });

  test("with a router, follows routes without leaving the page", async ({ page, example }) => {
    const stage = await example("tabs/router");
    const url = page.url();
    const members = stage.getByRole("link", { name: /Members/ });

    await expect(stage.getByRole("link", { name: "Overview" })).toHaveAttribute("aria-current", "page");
    await expect(members).toHaveAttribute("href", "/missions/alpha/members");
    await members.click();

    await expect(stage.getByRole("status")).toHaveText("Route: /missions/alpha/members");
    await expect(members).toHaveAttribute("aria-current", "page");
    await expect(stage.getByRole("link", { name: "Overview" })).not.toHaveAttribute("aria-current");
    expect(page.url()).toBe(url);
  });
});
