import { expect, test } from "../fixtures";

test.describe("Skeleton", () => {
  test("is replaced by the content without moving the layout", async ({ example }) => {
    const stage = await example("skeleton/composed");
    const section = stage.getByRole("region", { name: "Latest deploy" });

    await expect(section).toHaveAttribute("aria-busy", "true");
    await expect(section.getByRole("status")).toHaveText("Loading the latest deploy");
    await expect(section.locator("[aria-hidden=true]").first()).toBeVisible();
    const before = await section.boundingBox();

    await stage.getByRole("button", { name: "Finish loading" }).click();
    await expect(section.getByText("Asha Kader")).toBeVisible();
    await expect(section).not.toHaveAttribute("aria-busy", "true");
    const after = await section.boundingBox();
    expect(Math.abs(after!.height - before!.height)).toBeLessThanOrEqual(1);
  });

  test("a text placeholder takes exactly the lines it stands in for", async ({ example }) => {
    const stage = await example("skeleton/paragraph");
    const placeholder = stage.locator("[aria-hidden=true]").first();
    const lineHeight = await placeholder.evaluate((el) => parseFloat(getComputedStyle(el).lineHeight));
    const box = await placeholder.boundingBox();
    expect(box!.height).toBeCloseTo(4 * lineHeight, 0);
  });

  test("holds still for people who prefer reduced motion", async ({ page, example }) => {
    const animations = (stage: import("@playwright/test").Locator) =>
      stage.evaluate((el) => el.getAnimations({ subtree: true }).length);

    const moving = await example("skeleton/animation");
    expect(await animations(moving)).toBeGreaterThan(0);

    await page.emulateMedia({ reducedMotion: "reduce" });
    const still = await example("skeleton/animation");
    expect(await animations(still)).toBe(0);
  });
});
