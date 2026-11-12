import { expect, test } from "../fixtures";

test.describe("DescriptionList", () => {
  test("pairs each term with its value for assistive technology", async ({ example }) => {
    const stage = await example("description-list/basic");
    await expect(stage.locator("dl")).toHaveAccessibleName("Workload");
    await expect(stage.getByRole("term")).toHaveCount(4);
    await expect(stage.getByRole("definition")).toHaveCount(4);

    const cluster = stage.locator("dl > div").filter({ hasText: "Cluster" });
    await expect(cluster.getByRole("definition")).toHaveText("eu-west-1");
  });

  test("starts every value on the same line, beside its term", async ({ example }) => {
    const stage = await example("description-list/basic");
    const terms = await stage.getByRole("term").all();
    const values = await stage.getByRole("definition").all();
    const starts = new Set<number>();

    for (const [i, value] of values.entries()) {
      const [term, box] = [await terms[i].boundingBox(), await value.boundingBox()];
      expect(box!.x).toBeGreaterThan(term!.x + term!.width - 1);
      starts.add(Math.round(box!.x));
    }
    expect(starts.size).toBe(1);
  });

  test("moves terms above their values when the list is narrow", async ({ example }) => {
    const stage = await example("description-list/narrow");
    const [wide, narrow] = await stage.locator("dl").all();
    const below = async (list: typeof wide) => {
      const term = await list.getByRole("term").first().boundingBox();
      const value = await list.getByRole("definition").first().boundingBox();
      return value!.y >= term!.y + term!.height - 1;
    };

    expect(await below(wide)).toBe(false);
    expect(await below(narrow)).toBe(true);
  });

  test("stacked items reflow into fewer columns as space runs out", async ({ page, example }) => {
    const stage = await example("description-list/stacked");
    const columns = async () => {
      const boxes = await Promise.all((await stage.getByRole("term").all()).map((term) => term.boundingBox()));
      return new Set(boxes.map((box) => Math.round(box!.x))).size;
    };

    await page.setViewportSize({ width: 1280, height: 720 });
    const roomy = await columns();
    await page.setViewportSize({ width: 360, height: 720 });
    const tight = await columns();

    expect(roomy).toBeGreaterThan(1);
    expect(tight).toBeLessThan(roomy);
  });

  test("keeps long values inside the list and marks a missing value", async ({ page, example }) => {
    await page.setViewportSize({ width: 360, height: 720 });
    const stage = await example("description-list/rich-values");
    const list = await stage.locator("dl").boundingBox();

    for (const value of await stage.getByRole("definition").all()) {
      const box = await value.boundingBox();
      expect(box!.x + box!.width).toBeLessThanOrEqual(list!.x + list!.width + 1);
    }
    await expect(stage.getByRole("link", { name: "Let's Encrypt" })).toBeVisible();
    await expect(stage.getByRole("definition").last()).not.toBeEmpty();
  });
});
