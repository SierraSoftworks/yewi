import { expect, test } from "../fixtures";

test.describe("PageHeader", () => {
  test("exposes the title as the page heading, with its actions after it", async ({ page, example }) => {
    const stage = await example("page-header/basic");
    await expect(stage.getByRole("heading", { level: 1 })).toHaveText("Rollouts");

    await page.keyboard.press("Tab");
    await expect(stage.getByRole("button", { name: "Refresh" })).toBeFocused();
    await page.keyboard.press("Tab");
    await expect(stage.getByRole("button", { name: "New rollout" })).toBeFocused();
  });

  test("drops the actions under the title on a narrow page instead of overflowing", async ({ page, example }) => {
    await page.setViewportSize({ width: 340, height: 700 });
    const stage = await example("page-header/context");
    const title = (await stage.getByRole("heading", { level: 1 }).boundingBox())!;
    const approve = (await stage.getByRole("button", { name: "Approve" }).boundingBox())!;

    expect(approve.y).toBeGreaterThan(title.y + title.height - 1);
    expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(true);
  });

  test("content under the header stays interactive", async ({ example }) => {
    const stage = await example("page-header/content");
    await stage.getByRole("button", { name: "Needs attention" }).click();
    await expect(stage.getByRole("status")).toHaveText("Showing: Needs attention");
    await expect(stage.getByRole("button", { name: "Needs attention" })).toHaveAttribute("aria-pressed", "true");
  });
});
