import { expect, test } from "../fixtures";

test.describe("SegmentedControl", () => {
  test("selects a segment on click", async ({ example }) => {
    const stage = await example("segmented-control/basic");
    const control = stage.getByRole("radiogroup", { name: "Files shown" });
    await expect(control.getByRole("radio", { name: "Changed" })).toBeChecked();

    await control.getByRole("radio", { name: "All" }).click();
    await expect(control.getByRole("radio", { name: "All" })).toBeChecked();
    await expect(control.getByRole("radio", { name: "Changed" })).not.toBeChecked();
    await expect(stage.getByRole("status")).toHaveText("Showing all 37 files");
  });

  test("is one tab stop, and arrow keys move the selection", async ({ page, example }) => {
    const stage = await example("segmented-control/icons");
    const status = stage.getByRole("status");

    await page.keyboard.press("Tab");
    await expect(stage.getByRole("radio", { name: "List" })).toBeFocused();
    await page.keyboard.press("ArrowRight");
    await expect(status).toHaveText("Layout: grid, aligned left");
    await page.keyboard.press("ArrowLeft");
    await page.keyboard.press("ArrowLeft");
    await expect(status).toHaveText("Layout: columns, aligned left");

    await page.keyboard.press("Tab");
    await expect(stage.getByRole("radio", { name: "Left" })).toBeFocused();
    await page.keyboard.press("ArrowRight");
    await expect(status).toHaveText("Layout: columns, aligned center");
  });

  test("shows a focus indicator on the focused segment", async ({ page, example }) => {
    const stage = await example("segmented-control/basic");
    await page.keyboard.press("Tab");
    const segment = stage.locator("label", { has: page.getByRole("radio", { name: "Changed" }) });
    await expect(segment).toHaveCSS("outline-style", "solid");
  });

  test("skips disabled segments", async ({ page, example }) => {
    const stage = await example("segmented-control/states");
    const range = stage.getByRole("radiogroup", { name: "Range", exact: true });
    await expect(range.getByRole("radio", { name: "Year" })).toBeDisabled();

    await range.getByRole("radio", { name: "Day" }).focus();
    await page.keyboard.press("ArrowRight");
    await page.keyboard.press("ArrowRight");
    await expect(stage.getByRole("status")).toHaveText("Range: day");

    for (const radio of await stage.getByRole("radiogroup", { name: "Archived range" }).getByRole("radio").all()) {
      await expect(radio).toBeDisabled();
    }
  });

  test("fills its container with equal segments when block", async ({ example }) => {
    const stage = await example("segmented-control/block");
    const [a, b] = await Promise.all(
      ["Combined", "By target"].map((name) =>
        stage.locator("label", { hasText: name }).boundingBox().then((box) => box!.width),
      ),
    );
    expect(Math.abs(a - b)).toBeLessThanOrEqual(1);
    const control = (await stage.getByRole("radiogroup").boundingBox())!.width;
    expect(a + b).toBeGreaterThan(control * 0.9);
  });

  test("is named and described by the field around it", async ({ example }) => {
    const stage = await example("segmented-control/field");
    const control = stage.getByRole("radiogroup", { name: "Table density" });
    await expect(control).toHaveAccessibleDescription("Applies to every table in this workspace.");
    await control.getByRole("radio", { name: "Compact" }).check();
    await expect(control.getByRole("radio", { name: "Compact" })).toBeChecked();
  });
});
