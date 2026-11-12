import { expect, test } from "../fixtures";

test.describe("InlineCode", () => {
  test("uses the code font and scales with its text", async ({ example }) => {
    const stage = await example("inline-code/basic");
    const sizes = await stage.locator("code").evaluateAll((els) =>
      els.map((el) => ({
        family: getComputedStyle(el).fontFamily,
        ratio: parseFloat(getComputedStyle(el).fontSize) / parseFloat(getComputedStyle(el.parentElement!).fontSize),
      })),
    );
    for (const { family, ratio } of sizes) {
      expect(family).toContain("Fira Code");
      expect(ratio).toBeCloseTo(sizes[0].ratio, 2);
    }
  });

  test("a long value wraps instead of overflowing", async ({ example }) => {
    const stage = await example("inline-code/wrapping");
    const paragraph = stage.locator("p");
    const overflow = await paragraph.evaluate((el) => el.scrollWidth - el.clientWidth);
    expect(overflow).toBeLessThanOrEqual(0);
    const lines = await stage.locator("code").evaluate((el) => el.getClientRects().length);
    expect(lines).toBeGreaterThan(1);
  });

  test("takes the link colour inside a link", async ({ example }) => {
    const stage = await example("inline-code/in-links");
    const link = stage.getByRole("link");
    const colour = await link.evaluate((el) => getComputedStyle(el).color);
    await expect(link.locator("code")).toHaveCSS("color", colour);
  });
});
