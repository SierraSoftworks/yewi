import { expect, test } from "../fixtures";

test.describe("ToggleChip", () => {
  test("turns on and off with the mouse", async ({ example }) => {
    const stage = await example("toggle-chip/basic");
    const group = stage.getByRole("group", { name: "Tailscale tags" }).last();
    const edge = group.getByRole("button", { name: "tag:edge" });
    await expect(group.getByRole("button", { name: "tag:server" })).toHaveAttribute("aria-pressed", "true");
    await expect(edge).toHaveAttribute("aria-pressed", "false");

    await edge.click();
    await expect(edge).toHaveAttribute("aria-pressed", "true");
    await expect(stage.getByRole("status")).toHaveText("2 of 4 tags");

    await edge.click();
    await expect(edge).toHaveAttribute("aria-pressed", "false");
    await expect(stage.getByRole("status")).toHaveText("1 of 4 tags");
  });

  test("each chip is a tab stop that Space and Enter toggle", async ({ page, example }) => {
    const stage = await example("toggle-chip/filters");
    const group = stage.getByRole("group", { name: "Filter runs" });
    const status = stage.getByRole("status");

    await page.keyboard.press("Tab");
    const failed = group.getByRole("button", { name: "Failed" });
    await expect(failed).toBeFocused();
    await expect(failed).toHaveCSS("outline-style", "solid");
    await page.keyboard.press("Space");
    await expect(status).toHaveText("Showing every run");

    await page.keyboard.press("Tab");
    await page.keyboard.press("Enter");
    await expect(group.getByRole("button", { name: "Started by me" })).toBeFocused();
    await expect(status).toHaveText("Showing: mine");
  });

  test("shows a mark while on, unless it has its own icon", async ({ example }) => {
    const stage = await example("toggle-chip/filters");
    const failed = stage.getByRole("button", { name: "Failed" });
    const pinned = stage.getByRole("button", { name: "Pinned" });
    await expect(failed.locator("svg")).toHaveCount(1);
    await expect(pinned.locator("svg")).toHaveCount(1);

    await failed.click();
    await pinned.click();
    await expect(failed.locator("svg")).toHaveCount(0);
    await expect(pinned.locator("svg")).toHaveCount(1);
  });

  test("a disabled chip keeps its state and cannot be pressed", async ({ example }) => {
    const stage = await example("toggle-chip/states");
    const chip = stage.getByRole("button", { name: "On, disabled" });
    await expect(chip).toBeDisabled();
    await expect(chip).toHaveAttribute("aria-pressed", "true");
  });
});
