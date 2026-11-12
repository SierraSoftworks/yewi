import { expect, test } from "../fixtures";

test.describe("Checkbox", () => {
  test("toggles from its label and from the keyboard", async ({ page, example }) => {
    const stage = await example("checkbox/basic");
    const box = stage.getByRole("checkbox", { name: "Notify me when the deploy finishes" });
    const status = stage.getByRole("status");

    await stage.getByText("Notify me when the deploy finishes").click();
    await expect(box).toBeChecked();
    await expect(status).toHaveText("Notifications on");

    await page.keyboard.press("Space");
    await expect(box).not.toBeChecked();
    await expect(status).toHaveText("Notifications off");
  });

  test("shows a focus indicator when reached with Tab", async ({ page, example }) => {
    const stage = await example("checkbox/basic");
    await page.keyboard.press("Tab");
    const box = stage.getByRole("checkbox");
    await expect(box).toBeFocused();
    await expect(box).toHaveCSS("outline-style", "solid");
  });

  test("a description is announced with its option", async ({ example }) => {
    const stage = await example("checkbox/descriptions");
    const group = stage.getByRole("group", { name: "Email me about" });
    const noisy = group.getByRole("checkbox", { name: "Successful deploys" });
    await expect(noisy).toHaveAccessibleDescription(/can be noisy/);

    await noisy.check();
    await expect(stage.getByRole("status")).toHaveText("3 of 3 selected");
  });

  test("a parent checkbox is mixed until all or none of its items are selected", async ({ example }) => {
    const stage = await example("checkbox/select-all");
    const all = stage.getByRole("checkbox", { name: "All services" });
    const status = stage.getByRole("status");

    await expect(all).toHaveJSProperty("indeterminate", true);
    await expect(all).toBeChecked({ indeterminate: true });

    await all.click();
    await expect(status).toHaveText("3 selected");
    await expect(all).toBeChecked();
    for (const name of ["checkout-api", "billing", "search"]) {
      await expect(stage.getByRole("checkbox", { name })).toBeChecked();
    }

    await stage.getByRole("checkbox", { name: "billing" }).uncheck();
    await expect(all).toBeChecked({ indeterminate: true });

    await all.click();
    await all.click();
    await expect(status).toHaveText("0 selected");
    await expect(all).not.toBeChecked();
  });

  test("cannot be toggled while disabled", async ({ example }) => {
    const stage = await example("checkbox/states");
    const disabled = stage.getByRole("checkbox", { name: "Disabled", exact: true });
    await expect(disabled).toBeDisabled();
    await stage.getByText("Disabled", { exact: true }).click({ force: true });
    await expect(disabled).not.toBeChecked();
  });

  test("takes its error and required state from the field", async ({ example }) => {
    const stage = await example("checkbox/field");
    const box = stage.getByRole("checkbox", { name: /approved change window/ });
    await expect(box).toHaveJSProperty("required", true);

    await stage.getByRole("button", { name: "Deploy" }).click();
    await expect(stage.getByRole("alert")).toContainText("Confirm the change window");
    await expect(box).toHaveAttribute("aria-invalid", "true");
    await expect(box).toHaveAccessibleDescription(/Confirm the change window/);

    await box.check();
    await expect(stage.getByRole("alert")).toHaveCount(0);
    await stage.getByRole("button", { name: "Deploy" }).click();
    await expect(stage.getByRole("status")).toHaveText("Deploy started");
  });

  test("a bare checkbox is named by its label prop", async ({ example }) => {
    const stage = await example("checkbox/bare");
    await stage.getByRole("checkbox", { name: "Select web-03" }).check();
    await expect(stage.getByRole("status")).toHaveText("2 selected");
  });
});
