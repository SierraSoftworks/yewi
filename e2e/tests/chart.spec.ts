import { expect, test } from "../fixtures";

/** Moves the mouse to a fraction of the way across the plot. */
async function pointAt(page: import("@playwright/test").Page, plot: import("@playwright/test").Locator, fraction: number) {
  const box = (await plot.boundingBox())!;
  await page.mouse.move(box.x + box.width * fraction, box.y + box.height / 2, { steps: 4 });
}

test.describe("Chart", () => {
  test("shows the values under the pointer and clears them when it leaves", async ({ page, example }) => {
    const stage = await example("chart/compare");
    const plot = stage.getByRole("slider", { name: "Visitors per day" });
    const status = stage.getByRole("status");

    await pointAt(page, plot, 0);
    await expect(status).toHaveText("Reading 1 Jan");
    const tooltip = stage.locator(".yi-chart__tooltip");
    await expect(tooltip).toContainText("Visitors");
    await expect(tooltip).toContainText("412");
    await expect(tooltip).toContainText("Previous period");
    await expect(tooltip).toContainText("380");

    await pointAt(page, plot, 0.995);
    await expect(status).toHaveText("Reading 14 Jan");
    await expect(tooltip).toContainText("318");

    await page.mouse.move(0, 0);
    await expect(status).toHaveText("Not reading a day");
    await expect(tooltip).toHaveCount(0);
  });

  test("is read with the keyboard", async ({ page, example }) => {
    const stage = await example("chart/compare");
    const plot = stage.getByRole("slider", { name: "Visitors per day" });
    const status = stage.getByRole("status");

    await page.keyboard.press("Tab");
    await expect(plot).toBeFocused();
    await expect(plot).not.toHaveCSS("outline-style", "none");
    await expect(status).toHaveText("Reading 14 Jan");

    await page.keyboard.press("ArrowLeft");
    await expect(status).toHaveText("Reading 13 Jan");
    await expect(plot).toHaveAttribute("aria-valuetext", "13 Jan: Visitors 352, Previous period 322");

    await page.keyboard.press("Home");
    await expect(status).toHaveText("Reading 1 Jan");
    await page.keyboard.press("ArrowRight");
    await expect(plot).toHaveAttribute("aria-valuenow", "1");
    await page.keyboard.press("End");
    await expect(status).toHaveText("Reading 14 Jan");

    await page.keyboard.press("Tab");
    await expect(status).toHaveText("Not reading a day");
  });

  test("lists every series in a legend", async ({ example }) => {
    const stage = await example("chart/styles");
    const legend = stage.getByRole("list", { name: "Series" });
    await expect(legend.getByRole("listitem")).toHaveText(["p50", "p95", "p99", "Target"]);
  });

  test("selects a range by dragging", async ({ page, example }) => {
    const stage = await example("chart/select");
    const plot = stage.getByRole("slider", { name: "Visitors per day" });
    const status = stage.getByRole("status");

    await pointAt(page, plot, 3 / 13);
    await page.mouse.down();
    await pointAt(page, plot, 8 / 13);
    await expect(stage.locator(".yi-chart__selection")).toBeVisible();
    await page.mouse.up();
    await expect(status).toHaveText("Selected 4 Jan to 9 Jan");
    await expect(stage.locator(".yi-chart__selection")).toHaveCount(0);

    // A click without a drag selects nothing.
    await pointAt(page, plot, 0.5);
    await page.mouse.down();
    await page.mouse.up();
    await expect(status).toHaveText("Selected 4 Jan to 9 Jan");
  });

  test("selects a range with the keyboard", async ({ page, example }) => {
    const stage = await example("chart/select");
    const status = stage.getByRole("status");

    await page.keyboard.press("Tab");
    await page.keyboard.press("Home");
    await page.keyboard.press("Shift+ArrowRight");
    await page.keyboard.press("Shift+ArrowRight");
    await page.keyboard.press("Enter");
    await expect(status).toHaveText("Selected 1 Jan to 3 Jan");

    await page.keyboard.press("Shift+ArrowRight");
    await page.keyboard.press("Escape");
    await page.keyboard.press("Enter");
    await expect(status).toHaveText("Selected 1 Jan to 3 Jan");
  });

  test("stays responsive over a large dataset", async ({ page, example }) => {
    const stage = await example("chart/large");
    const plot = stage.getByRole("slider", { name: "Latency per request" });

    const points = await stage.locator(".yi-chart__line").evaluate((path) => (path.getAttribute("d") ?? "").split(/[ML]/).length);
    expect(points).toBeLessThanOrEqual(2001);

    await pointAt(page, plot, 0.5);
    await expect(stage.locator(".yi-chart__tooltip")).toContainText("ms");
    await pointAt(page, plot, 0.25);
    await expect(plot).toHaveAttribute("aria-valuenow", /^(49\d{3}|50\d{3})$/);
  });

  test("keeps its marks aligned with the axis when the container resizes", async ({ page, example }) => {
    const stage = await example("chart/basic");
    const plot = stage.getByRole("slider", { name: "Visitors per day" });
    for (const width of [1100, 420]) {
      await page.setViewportSize({ width, height: 800 });
      const marks = (await stage.locator(".yi-chart__marks").boundingBox())!;
      const box = (await plot.boundingBox())!;
      expect(Math.abs(marks.width - box.width)).toBeLessThan(1);
      await pointAt(page, plot, 0.995);
      await expect(plot).toHaveAttribute("aria-valuenow", "13");
    }
  });
});

test.describe("Sparkline", () => {
  test("is exposed as a labelled image", async ({ example }) => {
    const stage = await example("chart/sparkline");
    await expect(stage.getByRole("img", { name: "Errors, last 12 hours" })).toBeVisible();
  });
});
