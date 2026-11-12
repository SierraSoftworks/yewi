import { expect, test } from "../fixtures";

test.describe("StatTile", () => {
  test("chooses a metric with the mouse and the keyboard", async ({ page, example }) => {
    const stage = await example("stat-tile/selectable");
    const status = stage.getByRole("status");
    const tile = (name: string) => stage.getByRole("button", { name: new RegExp(name) });

    await expect(tile("Visitors")).toHaveAttribute("aria-pressed", "true");
    await expect(status).toHaveText("Charting visitors");

    await tile("Pageviews").click();
    await expect(status).toHaveText("Charting pageviews");
    await expect(tile("Pageviews")).toHaveAttribute("aria-pressed", "true");
    await expect(tile("Visitors")).toHaveAttribute("aria-pressed", "false");

    await page.keyboard.press("Tab");
    await expect(tile("Events")).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(status).toHaveText("Charting events");

    await page.keyboard.press("Shift+Tab");
    await page.keyboard.press("Shift+Tab");
    await page.keyboard.press("Space");
    await expect(status).toHaveText("Charting visitors");
  });

  test("shows a visible focus indicator for keyboard users", async ({ page, example }) => {
    const stage = await example("stat-tile/selectable");
    await page.keyboard.press("Tab");
    const focused = stage.getByRole("button", { name: /Visitors/ });
    await expect(focused).toBeFocused();
    await expect(focused).not.toHaveCSS("outline-style", "none");
  });

  test("announces the direction of a change as text", async ({ example }) => {
    const stage = await example("stat-tile/basic");
    await expect(stage.getByText("Up 8%")).toBeVisible();
    await expect(stage.getByText("Down 3%")).toBeVisible();
  });

  test("with href is a real link", async ({ example }) => {
    const stage = await example("stat-tile/sizes");
    await expect(stage.getByRole("link", { name: /Probes/ })).toHaveAttribute("href", /#/);
  });
});
