import type { Page } from "@playwright/test";
import { expect, test } from "../fixtures";

const clipboardText = (page: Page) => page.evaluate(() => (window as any).__clipboard.readText());
const background = (el: Element) => getComputedStyle(el).backgroundColor;

test.describe("CodeBlock", () => {
  test.beforeEach(async ({ context, page }) => {
    await context.grantPermissions(["clipboard-read", "clipboard-write"]);
    await page.addInitScript(() => ((window as any).__clipboard = navigator.clipboard));
  });

  test("names the file and language, and copies the code exactly", async ({ page, example }) => {
    const stage = await example("code-block/basic");
    await expect(stage.getByText("src/app.rs")).toBeVisible();
    await expect(stage.getByText("Rust", { exact: true })).toBeVisible();

    await stage.getByRole("button", { name: "Copy code" }).click();

    await expect(stage.getByRole("status")).toHaveText("Copied 19 lines");
    const copied = await clipboardText(page);
    expect(copied).toContain("pub fn App() -> Html {");
    expect(copied).toContain('<Button variant={ButtonVariant::Primary} {on_click}>{ "Deploy" }</Button>');
  });

  test("the code can be focused and scrolled with the keyboard", async ({ page, example }) => {
    await page.setViewportSize({ width: 390, height: 844 });
    const stage = await example("code-block/basic");
    const code = stage.locator("pre");

    await page.keyboard.press("Tab");
    await page.keyboard.press("Tab");
    await expect(code).toBeFocused();
    await expect(code).not.toHaveCSS("outline-style", "none");
    await page.keyboard.press("ArrowRight");
    await page.keyboard.press("ArrowRight");
    await expect.poll(() => code.evaluate((el) => el.scrollLeft)).toBeGreaterThan(0);
  });

  test("selecting numbered code leaves the numbers out and keeps the lines", async ({ page, example }) => {
    const stage = await example("code-block/lines");
    const code = stage.locator("pre code").first();
    await expect(code.locator(".yi-code-block__line--marked")).toHaveCount(3);

    const selected = await code.evaluate((el) => {
      getSelection()!.selectAllChildren(el);
      return getSelection()!.toString();
    });
    expect(selected.split("\n").slice(0, 5)).toEqual([
      'job "docs" {',
      '  datacenters = ["fra", "ams"]',
      "",
      '  group "web" {',
      "    count = 4",
    ]);
    expect(await page.evaluate(() => getSelection()!.toString())).not.toMatch(/^\s*\d+\s/m);
  });

  test("a forced dark block stays dark while its neighbour follows the page", async ({ example }) => {
    for (const theme of ["light", "dark"] as const) {
      const stage = await example("code-block/dark", { theme });
      const [following, forced] = [0, 1].map((i) => stage.locator(".yi-code-block").nth(i));
      await expect(forced).toHaveCSS("color-scheme", "dark");
      const same = (await following.evaluate(background)) === (await forced.evaluate(background));
      expect(same).toBe(theme === "dark");
    }
  });

  test("wrap is toggled from the header and removes the sideways scroll", async ({ example }) => {
    const stage = await example("code-block/long");
    const code = stage.locator("pre").first();
    const overflows = () => code.evaluate((el) => el.scrollWidth > el.clientWidth);
    const toggle = stage.getByRole("button", { name: "Wrap" });

    expect(await overflows()).toBe(true);
    await toggle.click();
    await expect(toggle).toHaveAttribute("aria-pressed", "true");
    expect(await overflows()).toBe(false);

    // The second block is capped in height and scrolls inside itself.
    const capped = stage.locator("pre").nth(1);
    expect(await capped.evaluate((el) => el.scrollHeight > el.clientHeight)).toBe(true);
  });

  test("a block with nothing to name has no header", async ({ example }) => {
    const stage = await example("code-block/bare");
    await expect(stage.getByRole("button", { name: "Copy code" })).toHaveCount(1);
    await expect(stage.getByText("Shell", { exact: true })).toBeVisible();
  });
});
