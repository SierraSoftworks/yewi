import { expect, test } from "../fixtures";

test.describe("SegmentBar", () => {
  test("is announced as one sentence and shows a segment's number on hover", async ({ page, example }) => {
    const stage = await example("segment_bar/basic");
    const bar = stage.getByRole("img", { name: "Hosts: 14 done, 2 failed, 8 to go" });
    await expect(bar).toBeVisible();

    await bar.locator("> *").first().hover();
    await expect(page.getByRole("tooltip")).toHaveText("14 done");
  });

  test("segments are sized in proportion to their values", async ({ example }) => {
    const stage = await example("segment_bar/basic");
    const parts = stage.getByRole("img", { name: /^Hosts/ }).locator("> *");
    await expect(parts).toHaveCount(3);
    const [done, failed, rest] = await Promise.all([0, 1, 2].map(async (i) => (await parts.nth(i).boundingBox())!.width));
    expect(done / failed).toBeGreaterThan(5);
    expect(done / failed).toBeLessThan(9);
    expect(rest / failed).toBeGreaterThan(3);
  });

  test("a tiny share stays visible", async ({ example }) => {
    const stage = await example("segment_bar/states");
    const parts = stage.getByRole("img", { name: /^eu-west/ }).locator("> *");
    for (const box of await Promise.all((await parts.all()).map((part) => part.boundingBox()))) {
      expect(box!.width).toBeGreaterThanOrEqual(3);
    }
  });
});
