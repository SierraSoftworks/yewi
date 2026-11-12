import { expect, test } from "../fixtures";

test.describe("Switch", () => {
  test("flips from its label and from the keyboard", async ({ page, example }) => {
    const stage = await example("switch/basic");
    const toggle = stage.getByRole("switch", { name: "Pause deploys" });
    const status = stage.getByRole("status");

    await stage.getByText("Pause deploys").click();
    await expect(toggle).toBeChecked();
    await expect(status).toHaveText("Deploys are paused");

    await page.keyboard.press("Space");
    await expect(toggle).not.toBeChecked();
    await expect(status).toHaveText("Deploys are running");
  });

  test("shows a focus indicator when reached with Tab", async ({ page, example }) => {
    const stage = await example("switch/basic");
    await page.keyboard.press("Tab");
    const toggle = stage.getByRole("switch");
    await expect(toggle).toBeFocused();
    await expect(toggle).toHaveCSS("outline-style", "solid");
  });

  test("is named and described by the field around it", async ({ example }) => {
    const stage = await example("switch/settings");
    const freeze = stage.getByRole("switch", { name: "Weekend freeze" });
    await expect(freeze).toHaveAccessibleDescription(/Friday 18:00/);

    await stage.getByText("Weekend freeze").click();
    await expect(freeze).toBeChecked();
    await expect(stage.getByRole("status")).toHaveText("3 of 3 enabled");
  });

  test("cannot be flipped while disabled, and keeps showing its state", async ({ example }) => {
    const stage = await example("switch/states");
    const on = stage.getByRole("switch", { name: "Disabled, on" });
    await expect(on).toBeDisabled();
    await expect(on).toBeChecked();
    await stage.getByText("Disabled, on").click({ force: true });
    await expect(on).toBeChecked();
  });
});
