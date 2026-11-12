import { expect, hydrated, test } from "../fixtures";

const root = "html";

test.describe("Theme", () => {
  test("follows the system until the user chooses", async ({ page, example }) => {
    await page.emulateMedia({ colorScheme: "dark" });
    await example("theme/from-rust");
    await expect(page.locator(root)).not.toHaveAttribute("data-theme");
    await expect(page.getByRole("status")).toContainText("Preference: System. The page is dark.");

    await page.emulateMedia({ colorScheme: "light" });
    await expect(page.getByRole("status")).toContainText("The page is light.");
  });

  test("the toggle cycles system, light, dark and names the next step", async ({ page, example }) => {
    const stage = await example("theme/toggle");
    const toggle = stage.getByRole("button");

    await expect(toggle).toHaveAccessibleName(/Theme: System/);
    await toggle.click();
    await expect(page.locator(root)).toHaveAttribute("data-theme", "light");
    await expect(toggle).toHaveAccessibleName(/Theme: Light/);
    await toggle.click();
    await expect(page.locator(root)).toHaveAttribute("data-theme", "dark");
    await toggle.click();
    await expect(page.locator(root)).not.toHaveAttribute("data-theme");
  });

  test("a choice survives a reload and applies before the app starts", async ({ page, example }) => {
    const stage = await example("theme/toggle");
    await stage.getByRole("button").click();
    await stage.getByRole("button").click();
    await expect(page.locator(root)).toHaveAttribute("data-theme", "dark");

    // Hold the wasm back: the theme must already be right without it.
    await page.route("**/*.wasm", () => {});
    await page.reload({ waitUntil: "domcontentloaded" });
    await expect(page.locator(root)).toHaveAttribute("data-theme", "dark");
    await expect(page.locator("body")).toHaveCSS("color-scheme", "dark");
  });

  test("changes the colours the page is drawn in", async ({ page, example }) => {
    await example("theme/from-rust");
    const background = () => page.evaluate(() => getComputedStyle(document.body).backgroundColor);

    await page.getByRole("button", { name: "Light" }).click();
    const light = await background();
    await page.getByRole("button", { name: "Dark" }).click();
    await expect.poll(background).not.toBe(light);
  });

  test("the docs header toggle works on a full page", async ({ page }) => {
    await page.goto("/components/button");
    await hydrated(page);
    await page.getByRole("banner").getByRole("button", { name: /Theme/ }).click();
    await expect(page.locator(root)).toHaveAttribute("data-theme", "light");
  });
});
