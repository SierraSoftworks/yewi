import { expect, test } from "../fixtures";

test.describe("Card", () => {
  test("lays panels out flush, with alternating fills", async ({ example }) => {
    const stage = await example("card/panels");
    const card = stage.locator("section").first();
    const panels = card.locator("section");
    await expect(panels).toHaveCount(3);

    const boxes = await Promise.all([card, panels.nth(0), panels.nth(1)].map((l) => l.boundingBox()));
    const [outer, first, second] = boxes.map((box) => box!);
    // Edge to edge inside the card's border, and touching each other.
    expect(first.width).toBeGreaterThan(outer.width - 4);
    expect(Math.abs(second.y - (first.y + first.height))).toBeLessThan(1);

    const fill = (i: number) => panels.nth(i).evaluate((el) => getComputedStyle(el).backgroundColor);
    expect(await fill(0)).not.toBe(await fill(1));
    expect(await fill(0)).toBe(await fill(2));
  });

  test("keeps its actions reachable by keyboard", async ({ page, example }) => {
    const stage = await example("card/footer");
    await page.keyboard.press("Tab");
    await expect(stage.getByRole("button", { name: "Cancel" })).toBeFocused();
    await page.keyboard.press("Tab");
    await expect(stage.getByRole("button", { name: "Save changes" })).toBeFocused();
  });

  test("titles are headings", async ({ example }) => {
    const stage = await example("card/grid");
    await expect(stage.getByRole("heading", { level: 3 })).toHaveText(["Home", "Staging", "Production"]);
  });
});
