import { expect, test } from "../fixtures";

test.describe("Heading", () => {
  test("exposes each level to assistive technology", async ({ example }) => {
    const stage = await example("heading/levels");
    for (const level of [1, 2, 3, 4, 5, 6]) {
      await expect(stage.getByRole("heading", { level })).toHaveCount(1);
    }
  });

  test("gets lighter as it gets larger", async ({ example }) => {
    const stage = await example("heading/sizes");
    const headings = await stage.getByRole("heading").evaluateAll((els) =>
      els.map((el) => ({ size: parseFloat(getComputedStyle(el).fontSize), weight: Number(getComputedStyle(el).fontWeight) })),
    );

    const bySize = [...headings].sort((a, b) => a.size - b.size);
    for (let i = 1; i < bySize.length; i++) {
      expect(bySize[i].weight).toBeLessThanOrEqual(bySize[i - 1].weight);
    }
    expect(bySize.at(-1)!.weight).toBeLessThan(bySize[0].weight);
  });

  test("size changes the look without changing the level", async ({ example }) => {
    const stage = await example("heading/sizes");
    await expect(stage.getByRole("heading", { level: 3 })).toHaveCount(8);
  });

  test("a truncated title stays on one line inside its container", async ({ example }) => {
    const stage = await example("heading/in-context");
    const title = stage.getByRole("heading", { level: 1 });
    const { clipped, lines } = await title.evaluate((el) => ({
      clipped: el.scrollWidth > el.clientWidth,
      lines: el.getBoundingClientRect().height / parseFloat(getComputedStyle(el).lineHeight),
    }));
    expect(clipped).toBe(true);
    expect(Math.round(lines)).toBe(1);
  });
});
