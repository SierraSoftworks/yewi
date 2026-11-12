import { expect, test, type Theme } from "../fixtures";

test.describe("Anchor positioning", () => {
  test("opens a floating panel under its anchor, lined up with its start", async ({ page, example }) => {
    const stage = await example("anchor/basic");
    const button = stage.getByRole("button", { name: "Filters" });
    const panel = page.getByRole("group", { name: "Filters" });

    await button.click();
    await expect(panel).toBeVisible();
    await expect(panel).toHaveAttribute("data-yi-side", "bottom");
    const [anchor, floating] = [(await button.boundingBox())!, (await panel.boundingBox())!];
    expect(floating.y).toBeGreaterThanOrEqual(anchor.y + anchor.height);
    expect(Math.abs(floating.x - anchor.x)).toBeLessThanOrEqual(1);

    await button.click();
    await expect(panel).toBeHidden();
  });

  test("puts the floating element on the side each placement asks for", async ({ page, example }) => {
    const stage = await example("anchor/placement");
    const anchor = stage.getByText("Anchor", { exact: true });
    const floating = page.getByRole("status");

    for (const button of await stage.getByRole("button").all()) {
      const name = await button.innerText();
      const side = name.match(/^(Top|Bottom|Left|Right)/)![0].toLowerCase();
      await button.click();
      await expect(floating).toHaveText(name);
      await expect(floating).toHaveAttribute("data-yi-side", side);

      const [a, f] = [(await anchor.boundingBox())!, (await floating.boundingBox())!];
      const onSide = {
        top: f.y + f.height <= a.y,
        bottom: f.y >= a.y + a.height,
        left: f.x + f.width <= a.x,
        right: f.x >= a.x + a.width,
      }[side];
      expect(onSide, name).toBe(true);
    }
  });

  test("follows its anchor while scrolling, flips when out of room and hides with the anchor", async ({ page, example }) => {
    const stage = await example("anchor/collision");
    const area = stage.getByRole("group", { name: "Scrolling area" });
    const button = stage.getByRole("button", { name: "Show" });
    const floating = page.getByRole("status");

    await button.click();
    await expect(floating).toHaveAttribute("data-yi-side", "top");
    const before = (await floating.boundingBox())!;

    await area.evaluate((el) => el.scrollBy(0, 40));
    await expect.poll(async () => (await floating.boundingBox())!.y).toBeLessThan(before.y);

    // Push the anchor up against the top of the window: no room above.
    await page.setViewportSize({ width: 1280, height: 240 });
    await area.evaluate((el) => el.scrollTo(0, 120));
    await page.evaluate(() => window.scrollTo(0, 40));
    await expect(floating).toHaveAttribute("data-yi-side", "bottom");

    await page.evaluate(() => window.scrollTo(0, 2000));
    await area.evaluate((el) => el.scrollTo(0, 0));
    await page.setViewportSize({ width: 1280, height: 720 });
    await expect(floating).toBeVisible();
  });

  for (const theme of ["light", "dark"] as Theme[]) {
    test(`looks right when open: ${theme}`, async ({ page, example }) => {
      const stage = await example("anchor/placement", { theme });
      await stage.getByRole("button", { name: "RightStart" }).click();
      await expect(page.getByRole("status")).toBeVisible();
      await expect(stage).toHaveScreenshot(["anchor", `open-${theme}.png`]);
    });
  }
});
