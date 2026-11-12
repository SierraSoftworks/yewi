import { expect, test } from "../fixtures";

test.describe("Steps", () => {
  test("follows the flow and marks the current step", async ({ example }) => {
    const stage = await example("steps/wizard");
    const steps = stage.getByRole("list", { name: "Checkout progress" }).getByRole("listitem");
    const status = stage.getByRole("status");

    await expect(steps).toHaveCount(4);
    await expect(steps.nth(1)).toHaveAttribute("aria-current", "step");
    await expect(steps.nth(0)).toContainText("completed");

    await stage.getByRole("button", { name: "Next" }).click();
    await expect(status).toHaveText("Step 3 of 4: Payment");
    await expect(steps.nth(2)).toHaveAttribute("aria-current", "step");
    await expect(steps.nth(1)).not.toHaveAttribute("aria-current", "step");
  });

  test("goes back to a finished step with the mouse", async ({ example }) => {
    const stage = await example("steps/wizard");
    await stage.getByRole("button", { name: "Next" }).click();
    await stage.getByRole("button", { name: "Shipping" }).click();
    await expect(stage.getByRole("status")).toHaveText("Step 2 of 4: Shipping");
  });

  test("goes back to a finished step with the keyboard", async ({ page, example }) => {
    const stage = await example("steps/wizard");
    await page.keyboard.press("Tab");
    const cart = stage.getByRole("button", { name: "Cart" });
    await expect(cart).toBeFocused();
    await expect(cart).not.toHaveCSS("outline-style", "none");
    await page.keyboard.press("Enter");
    await expect(stage.getByRole("status")).toHaveText("Step 1 of 4: Cart");
  });

  test("offers no control for steps that are not finished", async ({ example }) => {
    const stage = await example("steps/wizard");
    await expect(stage.getByRole("button", { name: "Cart" })).toBeVisible();
    for (const name of ["Shipping", "Payment", "Review"]) {
      await expect(stage.getByRole("button", { name })).toHaveCount(0);
    }
  });

  test("stacks horizontal steps when there is no room", async ({ page, example }) => {
    await page.setViewportSize({ width: 1280, height: 800 });
    const stage = await example("steps/wizard");
    const steps = stage.getByRole("list", { name: "Checkout progress" }).getByRole("listitem");
    const [first, second] = [await steps.nth(0).boundingBox(), await steps.nth(1).boundingBox()];
    expect(second!.x).toBeGreaterThan(first!.x);

    await page.setViewportSize({ width: 390, height: 844 });
    await expect.poll(async () => (await steps.nth(1).boundingBox())!.y).toBeGreaterThan((await steps.nth(0).boundingBox())!.y);
    expect((await steps.nth(1).boundingBox())!.x).toBe((await steps.nth(0).boundingBox())!.x);
  });
});
