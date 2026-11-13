import { expect, test } from "../fixtures";

test.describe("KeyValueEditor", () => {
  test("edits a row in place", async ({ example }) => {
    const stage = await example("key-value-editor/basic");
    const value = stage.getByRole("textbox", { name: "Value, label 1" });
    await expect(value).toHaveValue("production");

    await value.fill("staging");
    await expect(stage.getByRole("status")).toHaveText("env=staging, tier=web");
  });

  test("adds a row and moves focus to its key", async ({ page, example }) => {
    const stage = await example("key-value-editor/basic");
    await stage.getByRole("button", { name: "Add label" }).click();

    const key = stage.getByRole("textbox", { name: "Key, label 3" });
    await expect(key).toBeFocused();
    await page.keyboard.type("zone");
    await page.keyboard.press("Tab");
    await page.keyboard.type("b");
    await expect(stage.getByRole("status")).toHaveText("env=production, tier=web, zone=b");
  });

  test("removes a row and keeps focus in the editor", async ({ example }) => {
    const stage = await example("key-value-editor/basic");
    await stage.getByRole("button", { name: "Remove label 1" }).click();

    await expect(stage.getByRole("textbox")).toHaveCount(2);
    await expect(stage.getByRole("textbox", { name: "Key, label 1" })).toHaveValue("tier");
    await expect(stage.getByRole("status")).toHaveText("tier=web");
    await expect(stage.getByRole("button", { name: "Add label" })).toBeFocused();
  });

  test("describes a row by its problem until it is fixed", async ({ example }) => {
    const stage = await example("key-value-editor/validation");
    const key = stage.getByRole("textbox", { name: "Key, variable 2" });
    await expect(key).toHaveAttribute("aria-invalid", "true");
    await expect(key).toHaveAccessibleDescription(/capital letters/);
    await expect(stage.getByRole("textbox", { name: "Key, variable 1" })).not.toHaveAttribute("aria-invalid");
    await expect(stage.getByRole("status")).toHaveText("2 problems");

    await key.fill("REGION");
    await expect(key).not.toHaveAttribute("aria-invalid");
    await stage.getByRole("button", { name: "Remove variable 3" }).click();
    await expect(stage.getByRole("status")).toHaveText("Ready to save");
  });

  test("stops adding at the limit and disables everything on request", async ({ example }) => {
    const stage = await example("key-value-editor/options");
    const add = stage.getByRole("button", { name: "Add header" });
    await add.click();
    await expect(add).toBeEnabled();
    await add.click();
    await expect(add).toBeDisabled();
    await expect(stage.getByRole("group", { name: "Request headers" }).getByRole("textbox")).toHaveCount(6);

    const inherited = stage.getByRole("group", { name: "Inherited labels" }).last();
    await expect(inherited.getByRole("textbox", { name: "Key, label 1" })).toBeDisabled();
    await expect(inherited.getByRole("button", { name: "Remove label 1" })).toBeDisabled();
  });
});
