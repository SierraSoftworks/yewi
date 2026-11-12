import { expect, test } from "../fixtures";

test.describe("Menu", () => {
  test("arrow keys, Home and End move between items and wrap", async ({ page, example }) => {
    const stage = await example("menu/basic");
    const item = (name: string) => stage.getByRole("menuitem", { name });

    await page.keyboard.press("Tab");
    await expect(stage.getByRole("menu", { name: "Dashboard" })).toBeFocused();

    await page.keyboard.press("ArrowDown");
    await expect(item("Rename")).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await expect(item("Duplicate")).toBeFocused();
    await page.keyboard.press("End");
    await expect(item("Archive")).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await expect(item("Rename")).toBeFocused();
    await page.keyboard.press("ArrowUp");
    await expect(item("Archive")).toBeFocused();
    await page.keyboard.press("Home");
    await expect(item("Rename")).toBeFocused();
  });

  test("a letter jumps to the next item starting with it, and Enter chooses it", async ({ page, example }) => {
    const stage = await example("menu/basic");

    await page.keyboard.press("Tab");
    await page.keyboard.press("e");
    await expect(stage.getByRole("menuitem", { name: "Export" })).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(stage.getByRole("status")).toHaveText("Exported the dashboard");

    await page.keyboard.press("d");
    await page.keyboard.press("Space");
    await expect(stage.getByRole("status")).toHaveText("Duplicated the dashboard");
  });

  test("a click chooses an item", async ({ example }) => {
    const stage = await example("menu/groups");

    await stage.getByRole("menuitem", { name: "Reboot" }).click();
    await expect(stage.getByRole("status")).toHaveText("Rebooted web-2");
    await stage.getByRole("menuitem", { name: "Forget" }).click();
    await expect(stage.getByRole("status")).toHaveText("Forgot web-2");
  });

  test("options toggle, a disabled item is skipped and a link is followed", async ({ page, example }) => {
    const stage = await example("menu/states");
    const wrap = stage.getByRole("menuitemcheckbox", { name: "Wrap long lines" });

    await expect(wrap).toHaveAttribute("aria-checked", "false");
    await wrap.click();
    await expect(wrap).toHaveAttribute("aria-checked", "true");
    await expect(stage.getByRole("status")).toHaveText("Relative times, wrapped");

    // The keyboard passes over the disabled item to the link after it.
    await page.keyboard.press("ArrowDown");
    await expect(stage.getByRole("menuitem", { name: "Query syntax" })).toBeFocused();

    // A link item is followed by Space as well as Enter.
    await stage.getByRole("menuitem", { name: "Query syntax" }).focus();
    await page.keyboard.press("Space");
    await expect(page).toHaveURL(/#query-syntax$/);
  });
});
