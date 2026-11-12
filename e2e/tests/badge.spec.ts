import { expect, test } from "../fixtures";

test.describe("Badge", () => {
  test("appears, counts, caps and disappears", async ({ example }) => {
    const stage = await example("badge/live");
    const status = stage.getByRole("status");
    const receive = stage.getByRole("button", { name: "Receive 4" });
    const badge = (text: string) => stage.getByText(text, { exact: true });

    await expect(status).toHaveText("0 unread notifications");
    await expect(badge("0")).toHaveCount(0);

    await receive.click();
    await expect(badge("4")).toBeVisible();

    await receive.click();
    await receive.click();
    await expect(status).toHaveText("12 unread notifications");
    await expect(badge("9+")).toBeVisible();

    await stage.getByRole("button", { name: "Mark all read" }).click();
    await expect(status).toHaveText("0 unread notifications");
    await expect(badge("9+")).toHaveCount(0);
  });

  test("sits on the corner of what it is attached to, without blocking it", async ({ example }) => {
    const stage = await example("badge/live");
    const bell = stage.getByRole("button", { name: "Notifications" });
    await stage.getByRole("button", { name: "Receive 4" }).click();

    const button = (await bell.boundingBox())!;
    const badge = (await stage.getByText("4", { exact: true }).boundingBox())!;
    const centre = { x: badge.x + badge.width / 2, y: badge.y + badge.height / 2 };
    expect(Math.abs(centre.x - (button.x + button.width))).toBeLessThan(3);
    expect(Math.abs(centre.y - button.y)).toBeLessThan(3);

    // A click that lands on the badge still reaches the button.
    await stage.page().mouse.click(centre.x - 2, centre.y + 2);
    await expect(bell).toBeFocused();
  });

  test("reads its label, not the bare number, to assistive technology", async ({ example }) => {
    const stage = await example("badge/attached");
    await expect(stage.getByText("3 unread notifications")).toBeAttached();
    await expect(stage.getByText("3", { exact: true })).toHaveAttribute("aria-hidden", "true");

    const dots = await example("badge/dot");
    await expect(dots.getByText("Update available")).toBeAttached();
  });
});
