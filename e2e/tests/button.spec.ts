import { expect, test } from "../fixtures";

test.describe("Button", () => {
  test("activates with the mouse and the keyboard", async ({ page, example }) => {
    const stage = await example("button/states");
    const status = stage.getByRole("status");
    const finish = stage.getByRole("button", { name: "Finish saving" });

    await stage.getByRole("button", { name: "Save" }).click();
    await expect(status).toHaveText("Saved 1 times");

    await finish.click();
    await stage.getByRole("button", { name: "Save" }).focus();
    await page.keyboard.press("Enter");
    await expect(status).toHaveText("Saved 2 times");

    await finish.click();
    await page.keyboard.press("Shift+Tab");
    await page.keyboard.press("Space");
    await expect(status).toHaveText("Saved 3 times");
  });

  test("ignores clicks while loading, but stays focusable and announces it", async ({ example }) => {
    const stage = await example("button/states");
    const save = stage.getByRole("button", { name: "Save" });

    await save.click();
    await expect(save).toHaveAttribute("aria-busy", "true");
    await save.click({ force: true });
    await expect(stage.getByRole("status")).toHaveText("Saved 1 times");

    await save.focus();
    await expect(save).toBeFocused();

    await stage.getByRole("button", { name: "Finish saving" }).click();
    await expect(save).not.toHaveAttribute("aria-busy", "true");
  });

  test("cannot be focused or clicked while disabled", async ({ page, example }) => {
    const stage = await example("button/states");
    const unavailable = stage.getByRole("button", { name: "Unavailable" });

    await expect(unavailable).toBeDisabled();
    await page.keyboard.press("Tab");
    await page.keyboard.press("Tab");
    await page.keyboard.press("Tab");
    await expect(unavailable).not.toBeFocused();
  });

  test("with href is a real link", async ({ example }) => {
    const stage = await example("button/link");
    await expect(stage.getByRole("link", { name: "Source" })).toHaveAttribute("href", /github\.com/);
  });

  test("shows a visible focus indicator for keyboard users", async ({ page, example }) => {
    const stage = await example("button/variants");
    await page.keyboard.press("Tab");
    const focused = stage.getByRole("button", { name: "Approve" });
    await expect(focused).toBeFocused();
    await expect(focused).not.toHaveCSS("outline-style", "none");
  });

  test("an icon button is operable by its label", async ({ page, example }) => {
    const stage = await example("button/group-toolbar");
    const status = stage.getByRole("status");
    const zoomIn = stage.getByRole("button", { name: "Zoom in" });

    await zoomIn.click();
    await expect(status).toContainText("Zoom 9");
    await zoomIn.focus();
    await page.keyboard.press("Enter");
    await expect(status).toContainText("Zoom 10");
    await expect(zoomIn).toBeDisabled();

    await stage.getByRole("button", { name: "Zoom out" }).click();
    await expect(status).toContainText("Zoom 9");
    await expect(zoomIn).toBeEnabled();
  });

  test("a toggle button reports its pressed state", async ({ page, example }) => {
    const stage = await example("button/toggle");
    const pin = stage.getByRole("button", { name: "Pin to dashboard" });
    const mute = stage.getByRole("button", { name: "Mute alerts" });

    await expect(pin).toHaveAttribute("aria-pressed", "false");
    await expect(mute).toHaveAttribute("aria-pressed", "true");

    await pin.click();
    await expect(pin).toHaveAttribute("aria-pressed", "true");
    await expect(stage.getByRole("status")).toHaveText("Pinned, alerts muted");

    await mute.focus();
    await page.keyboard.press("Space");
    await expect(mute).toHaveAttribute("aria-pressed", "false");
    await expect(stage.getByRole("status")).toHaveText("Pinned, alerts on");
  });

  test("a group is announced by its label and each button is a tab stop", async ({ page, example }) => {
    const stage = await example("button/group");
    const group = stage.getByRole("group", { name: "Connection actions" });
    const [reconnect, edit, unlink] = ["Reconnect", "Edit", "Unlink"].map((name) =>
      group.getByRole("button", { name, exact: true }),
    );

    await page.keyboard.press("Tab");
    await expect(reconnect).toBeFocused();
    await page.keyboard.press("Tab");
    await expect(edit).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(stage.getByRole("status")).toHaveText("Last action: Edit");
    await page.keyboard.press("Tab");
    await expect(unlink).toBeFocused();
    await expect(unlink).not.toHaveCSS("outline-style", "none");
  });

  test("grouped buttons join into one control", async ({ example }) => {
    const stage = await example("button/group");
    const group = stage.getByRole("group", { name: "Connection actions" });
    const boxes = await Promise.all(
      (await group.getByRole("button").all()).map(async (button) => (await button.boundingBox())!),
    );

    for (let i = 1; i < boxes.length; i++) {
      expect(boxes[i].x).toBeLessThanOrEqual(boxes[i - 1].x + boxes[i - 1].width);
      expect(boxes[i].y).toBe(boxes[0].y);
    }
  });

  test("an icon button shows its label as a tooltip on hover, in place of the native title", async ({ page, example }) => {
    const stage = await example("button/icon-only");
    const refresh = stage.getByRole("button", { name: "Refresh" });
    const hint = page.locator(".yi-tooltip__bubble");

    await expect(refresh).toHaveAttribute("title", "Refresh");
    await refresh.hover();
    await expect(hint).toHaveText("Refresh");
    await expect(refresh).not.toHaveAttribute("title");
    // The hint repeats the name, so it must not be read as well.
    await expect(refresh).toHaveAccessibleName("Refresh");
    await expect(refresh).not.toHaveAttribute("aria-describedby");

    await page.mouse.move(600, 500);
    await expect(hint).toHaveCount(0);
  });

  test("an icon button shows its hint on keyboard focus and Escape dismisses it", async ({ page, example }) => {
    const stage = await example("button/icon-only");
    const hint = page.locator(".yi-tooltip__bubble");

    await page.keyboard.press("Tab");
    await expect(stage.getByRole("button", { name: "Add host" })).toBeFocused();
    await expect(hint).toHaveText("Add host");

    await page.keyboard.press("Tab");
    await expect(hint).toHaveText("Refresh");
    await page.keyboard.press("Escape");
    await expect(hint).toHaveCount(0);
  });

  test("the hint does not take the button's clicks or its place in a group", async ({ page, example }) => {
    const stage = await example("button/group-toolbar");
    const buttons = stage.getByRole("button");
    const first = buttons.first();
    const before = (await first.boundingBox())!;

    await first.hover();
    await expect(page.locator(".yi-tooltip__bubble")).toBeVisible();
    expect(await first.boundingBox()).toEqual(before);
    await expect(first.locator("xpath=..")).toHaveClass(/yi-button-group/);
  });
});
