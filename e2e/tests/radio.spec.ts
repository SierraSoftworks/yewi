import { expect, test } from "../fixtures";

test.describe("RadioGroup", () => {
  test("selects an option from its label", async ({ example }) => {
    const stage = await example("radio/basic");
    const group = stage.getByRole("radiogroup", { name: "Deploy strategy" });
    await expect(group).toHaveAccessibleDescription("Applies to the next rollout.");
    await expect(group.getByRole("radio", { name: "Rolling" })).toBeChecked();

    await stage.getByText("Canary").click();
    await expect(group.getByRole("radio", { name: "Canary" })).toBeChecked();
    await expect(group.getByRole("radio", { name: "Rolling" })).not.toBeChecked();
    await expect(stage.getByRole("status")).toHaveText("Strategy: canary");
  });

  test("is one tab stop, and arrow keys move the selection", async ({ page, example }) => {
    const stage = await example("radio/basic");
    const status = stage.getByRole("status");

    await page.keyboard.press("Tab");
    const rolling = stage.getByRole("radio", { name: "Rolling" });
    await expect(rolling).toBeFocused();
    await expect(rolling).toHaveCSS("outline-style", "solid");

    await page.keyboard.press("ArrowDown");
    await expect(status).toHaveText("Strategy: blue-green");
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("ArrowDown");
    await expect(status).toHaveText("Strategy: rolling");
    await page.keyboard.press("ArrowUp");
    await expect(status).toHaveText("Strategy: canary");
    await expect(stage.getByRole("radio", { name: "Canary" })).toBeFocused();

    await page.keyboard.press("Tab");
    await expect(stage.getByRole("radio", { checked: true })).not.toBeFocused();
  });

  test("skips a disabled option and announces descriptions", async ({ page, example }) => {
    const stage = await example("radio/descriptions");
    await expect(stage.getByRole("radio", { name: /Owner.*Only an existing owner/ })).toBeDisabled();

    await stage.getByRole("radio", { name: "Operator" }).focus();
    await page.keyboard.press("ArrowDown");
    await expect(stage.getByRole("status")).toHaveText("Role: viewer");
  });

  test("works in a row", async ({ page, example }) => {
    const stage = await example("radio/horizontal");
    await stage.getByRole("radio", { name: "JSON" }).focus();
    await page.keyboard.press("ArrowRight");
    await expect(stage.getByRole("status")).toHaveText("Format: csv");
  });

  test("starts empty and shows the field's error until a choice is made", async ({ example }) => {
    const stage = await example("radio/form");
    const group = stage.getByRole("radiogroup", { name: "Region" });
    await expect(group.getByRole("radio", { checked: true })).toHaveCount(0);

    await stage.getByRole("button", { name: "Create cluster" }).click();
    await expect(stage.getByRole("alert")).toContainText("Choose a region.");
    await expect(group).toHaveAttribute("aria-invalid", "true");
    await expect(group).toHaveAccessibleDescription(/Choose a region/);

    await group.getByRole("radio", { name: "Sydney" }).check();
    await expect(stage.getByRole("alert")).toHaveCount(0);
    await expect(stage.getByRole("status")).toHaveText("Created in ap-southeast-2");
  });
});
