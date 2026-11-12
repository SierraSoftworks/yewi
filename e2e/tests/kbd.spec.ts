import { expect, test } from "../fixtures";

test.describe("Kbd", () => {
  test("a shortcut shows one cap per key", async ({ example }) => {
    const stage = await example("kbd/shortcuts");
    const palette = stage.locator("p", { hasText: "command palette" }).locator("kbd kbd");
    await expect(palette).toHaveText(["Ctrl", "Shift", "P"]);

    const zoom = stage.locator("p", { hasText: "Zoom in" }).locator("kbd kbd");
    await expect(zoom).toHaveText(["Ctrl", "+"]);
  });

  test("a shortcut never breaks across lines", async ({ page, example }) => {
    await page.setViewportSize({ width: 320, height: 600 });
    const stage = await example("kbd/shortcuts");
    const heights = await stage.locator("p > kbd").evaluateAll((els) =>
      els.map((el) => el.getClientRects().length),
    );
    expect(heights.every((rects) => rects === 1)).toBe(true);
  });

  test("a key cap stays within its line of text", async ({ example }) => {
    const stage = await example("kbd/basic");
    const paragraph = stage.locator("p");
    const { height, lineHeight } = await paragraph.evaluate((el) => ({
      height: el.getBoundingClientRect().height,
      lineHeight: parseFloat(getComputedStyle(el).lineHeight),
    }));
    expect(height).toBeLessThanOrEqual(lineHeight + 1);
  });
});
