import { expect, test } from "../fixtures";

test.describe("Alert", () => {
  test("is dismissed with the mouse and can be shown again", async ({ example }) => {
    const stage = await example("alert/actions");
    const alert = stage.getByRole("alert");
    const status = stage.getByRole("status");

    await expect(alert).toContainText("Could not save the mission");
    await alert.getByRole("button", { name: "Dismiss" }).click();
    await expect(alert).toHaveCount(0);
    await expect(status).toHaveText("Alert dismissed");

    await stage.getByRole("button", { name: "Show the alert again" }).click();
    await expect(alert).toBeVisible();
    await expect(status).toHaveText("Alert shown");
  });

  test("actions and the dismiss button work from the keyboard, in reading order", async ({ page, example }) => {
    const stage = await example("alert/actions");
    const alert = stage.getByRole("alert");

    await page.keyboard.press("Tab");
    await expect(alert.getByRole("button", { name: "Retry" })).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(stage.getByRole("status")).toHaveText("Retried 1 times");

    await page.keyboard.press("Tab");
    await page.keyboard.press("Tab");
    const dismiss = alert.getByRole("button", { name: "Dismiss" });
    await expect(dismiss).toBeFocused();
    await expect(dismiss).not.toHaveCSS("outline-style", "none");
    await page.keyboard.press("Space");
    await expect(alert).toHaveCount(0);
  });

  test("announces problems assertively and everything else politely", async ({ example }) => {
    const stage = await example("alert/tones");
    await expect(stage.getByRole("alert")).toHaveText([/expires in 6 days/, /Rollout failed/]);
    await expect(stage.getByRole("status")).toHaveCount(4);
  });
});

test.describe("Banner", () => {
  test("is dismissed without disturbing the banners after it", async ({ example }) => {
    const stage = await example("alert/banner");
    const announcement = stage.getByText("Version 2.4 is out");

    await expect(stage.getByRole("link", { name: "What changed" })).toBeVisible();
    await stage.getByRole("button", { name: "Dismiss announcement" }).click();
    await expect(announcement).toHaveCount(0);
    await expect(stage.getByText("Announcement dismissed")).toBeVisible();
    await expect(stage.getByRole("button", { name: "Reconnect" })).toBeVisible();
  });

  test("spans the full width of its container", async ({ example }) => {
    const stage = await example("alert/banner-solid");
    const banner = stage.getByText("All services are operating normally").locator("xpath=ancestor::*[@role='status']");
    const [box, parent] = await Promise.all([banner.boundingBox(), banner.locator("xpath=..").boundingBox()]);
    expect(box!.width).toBeCloseTo(parent!.width, 0);
  });
});
