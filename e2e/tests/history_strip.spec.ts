import { expect, test } from "../fixtures";

test.describe("HistoryStrip", () => {
  test("hovering a cell shows its detail, which follows the pointer and hides on leaving", async ({ page, example }) => {
    const stage = await example("history-strip/basic");
    const popover = page.getByRole("status");

    await stage.getByRole("img", { name: "03:30, Failing" }).hover();
    await expect(popover).toContainText("Failing");
    await expect(popover).toContainText("41.7%");

    await stage.getByRole("img", { name: "04:30, Degraded" }).hover();
    await expect(popover).toHaveCount(1);
    await expect(popover).toContainText("92.5%");

    // The pointer can cross onto the popover to read or select its text.
    await popover.getByText("Availability").hover();
    await expect(popover).toContainText("92.5%");

    await page.mouse.move(5, 5);
    await expect(popover).toBeHidden();
  });

  test("works from the keyboard: one tab stop, arrows move between cells, Escape closes", async ({ page, example }) => {
    const stage = await example("history-strip/basic");
    const strip = stage.getByRole("group", { name: "api.example.com, last 24 hours" });
    const popover = page.getByRole("status");

    await page.keyboard.press("Tab");
    await expect(strip).toBeFocused();
    await expect(popover).toContainText("11:30 UTC");

    await page.keyboard.press("ArrowLeft");
    await expect(popover).toContainText("11:00 UTC");
    await page.keyboard.press("Home");
    await expect(popover).toContainText("12:00 UTC");
    await page.keyboard.press("End");
    await expect(popover).toContainText("11:30 UTC");

    await page.keyboard.press("Escape");
    await expect(popover).toBeHidden();
    await expect(strip).toBeFocused();
    await page.keyboard.press("ArrowLeft");
    await expect(popover).toBeVisible();

    await page.keyboard.press("Tab");
    await expect(popover).toBeHidden();
  });

  test("pressing a cell, or Enter on it, selects it", async ({ page, example }) => {
    const stage = await example("history-strip/discrete");
    const status = stage.getByText(/run selected|Selected the run/);
    await expect(status).toHaveText("No run selected");

    await stage.getByRole("img", { name: "6 January, Failed" }).click();
    await expect(status).toHaveText("Selected the run of 6 January");
    await expect(page.getByRole("status").filter({ hasText: "Duration 0m 48s" })).toBeVisible();

    await page.mouse.click(5, 5);
    await page.keyboard.press("Tab");
    await page.keyboard.press("ArrowLeft");
    await page.keyboard.press("Enter");
    await expect(status).toHaveText("Selected the run of 13 January");
  });

  test("a strip without detail takes no focus and shows nothing on hover", async ({ page, example }) => {
    const stage = await example("history-strip/plain");
    await stage.getByRole("img", { name: "17:00, failing" }).hover();
    await page.keyboard.press("Tab");
    await expect(stage.getByRole("group").first()).not.toBeFocused();
    await expect(page.locator(".yi-popover__surface")).toHaveCount(0);
  });

  test("a narrow strip keeps the most recent cells in view", async ({ example }) => {
    const stage = await example("history-strip/narrow");
    const narrow = stage.getByRole("group").last();
    await expect(narrow.getByRole("img", { name: "23:45" })).toBeInViewport();
    await expect(narrow.getByRole("img", { name: "00:00" })).not.toBeInViewport();
  });
});
