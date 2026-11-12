import { expect, test } from "../fixtures";

test.describe("Toast", () => {
  test("a success dismisses itself, a failure stays until dismissed", async ({ page, example }) => {
    await page.clock.install();
    const stage = await example("toast/basic");
    const region = stage.getByRole("region", { name: "Notifications" });

    await stage.getByRole("button", { name: "Save" }).click();
    await stage.getByRole("button", { name: "Publish" }).click();
    const saved = region.getByRole("status").filter({ hasText: "Settings saved." });
    const failed = region.getByRole("alert").filter({ hasText: "Publishing failed" });
    await expect(saved).toBeVisible();
    await expect(failed).toBeVisible();

    await page.clock.runFor(6_000);
    await expect(saved).toHaveCount(0);
    await expect(failed).toBeVisible();

    await failed.getByRole("button", { name: "Dismiss" }).click();
    await expect(failed).toHaveCount(0);
  });

  test("the timer waits while the pointer or focus is on the stack", async ({ page, example }) => {
    await page.clock.install();
    const stage = await example("toast/basic");
    const saved = stage.getByRole("status").filter({ hasText: "Settings saved." });

    await stage.getByRole("button", { name: "Save" }).click();
    await saved.hover();
    await page.clock.runFor(10_000);
    await expect(saved).toBeVisible();

    await saved.getByRole("button", { name: "Dismiss" }).focus();
    await stage.getByRole("button", { name: "Save" }).hover();
    await page.clock.runFor(10_000);
    await expect(saved).toBeVisible();

    await stage.getByRole("button", { name: "Publish" }).focus();
    await page.clock.runFor(6_000);
    await expect(saved).toHaveCount(0);
  });

  test("is dismissed from the keyboard with Escape", async ({ page, example }) => {
    const stage = await example("toast/basic");
    const failed = stage.getByRole("alert");

    await stage.getByRole("button", { name: "Publish" }).click();
    await page.keyboard.press("Tab");
    const dismiss = failed.getByRole("button", { name: "Dismiss" });
    await expect(dismiss).toBeFocused();
    await expect(dismiss).not.toHaveCSS("outline-style", "none");

    await page.keyboard.press("Escape");
    await expect(failed).toHaveCount(0);
  });

  test("an action runs and dismisses its toast", async ({ example }) => {
    const stage = await example("toast/action");
    const status = stage.getByRole("status").filter({ hasText: "active hosts" });

    await stage.getByRole("button", { name: "Archive 3 hosts" }).click();
    await expect(status).toHaveText("No active hosts");
    const toast = stage.getByRole("status").filter({ hasText: "3 hosts archived." });

    await toast.getByRole("button", { name: "Undo" }).click();
    await expect(status).toHaveText("3 active hosts");
    await expect(toast).toHaveCount(0);
  });

  test("a sticky toast is replaced by its result", async ({ example }) => {
    const stage = await example("toast/lifecycle");
    const region = stage.getByRole("region", { name: "Notifications" });

    await stage.getByRole("button", { name: "Roll out" }).click();
    await expect(region.getByRole("status")).toHaveText(/Rolling out/);

    await stage.getByRole("button", { name: "Finish rollout" }).click();
    await expect(region.getByRole("status")).toHaveText(/Rollout finished/);
  });

  test("shows the newest four, nearest the chosen edge", async ({ page, example }) => {
    const stage = await example("toast/placement");
    const region = stage.getByRole("region", { name: "Notifications" });
    const notify = stage.getByRole("button", { name: "Notify" });
    const viewport = page.viewportSize()!;

    for (let i = 0; i < 5; i++) await notify.click();
    await expect(region.getByRole("status")).toHaveText([2, 3, 4, 5].map((n) => new RegExp(`Notification ${n}`)));

    const newest = region.getByRole("status").filter({ hasText: "Notification 5" });
    const oldest = region.getByRole("status").filter({ hasText: "Notification 2" });
    const centre = async (toast: typeof newest) => {
      const box = (await toast.boundingBox())!;
      return { x: box.x + box.width / 2, y: box.y + box.height / 2 };
    };

    // Bottom end by default.
    expect((await centre(newest)).x).toBeGreaterThan(viewport.width / 2);
    expect((await centre(newest)).y).toBeGreaterThan(viewport.height / 2);
    expect((await centre(newest)).y).toBeGreaterThan((await centre(oldest)).y);

    await stage.getByRole("button", { name: "Top start" }).click();
    expect((await centre(newest)).x).toBeLessThan(viewport.width / 2);
    expect((await centre(newest)).y).toBeLessThan(viewport.height / 2);
    expect((await centre(newest)).y).toBeLessThan((await centre(oldest)).y);

    // The stack never covers the page beneath it.
    await stage.getByRole("button", { name: "Bottom center" }).click();
    await expect(stage.getByRole("button", { name: "Bottom center" })).toHaveAttribute("aria-pressed", "true");
  });

  for (const theme of ["light", "dark"] as const) {
    test(`the stack in place: ${theme}`, async ({ page, example }) => {
      const stage = await example("toast/basic", { theme });
      await stage.getByRole("button", { name: "Publish" }).click();
      await stage.getByRole("button", { name: "Publish" }).click();
      await page.mouse.move(0, 0);
      await expect(page).toHaveScreenshot(["toast", `stack-${theme}-desktop.png`]);
    });
  }
});
