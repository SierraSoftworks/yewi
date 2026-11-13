import { expect, test } from "../fixtures";

test.describe("TextInput", () => {
  test("reports every edit, and commits on blur or Enter", async ({ page, example }) => {
    const stage = await example("input/basic");
    const input = stage.getByRole("textbox", { name: "Service name" });
    const status = stage.getByRole("status");

    await input.fill("billing");
    await expect(status).toHaveText("Typing: billing · Saved: checkout-api");
    await page.keyboard.press("Enter");
    await expect(status).toHaveText("Typing: billing · Saved: billing");

    await input.pressSequentially("-api");
    await expect(status).toHaveText("Typing: billing-api · Saved: billing");
    await page.keyboard.press("Tab");
    await expect(status).toHaveText("Typing: billing-api · Saved: billing-api");
  });

  test("reports the text left behind on blur, changed or not", async ({ page, example }) => {
    const stage = await example("input/on-blur");
    const input = stage.getByRole("textbox", { name: "Display name" });
    const status = stage.getByRole("status");

    // Nothing is typed, so no change is committed, yet the text is tidied.
    await input.focus();
    await page.keyboard.press("Tab");
    await expect(input).toHaveValue("checkout api");

    await input.fill(" billing   api");
    await expect(status).toHaveText("Name: [ billing   api]");
    await page.keyboard.press("Tab");
    await expect(status).toHaveText("Name: [billing api]");
  });

  test("shows a focus indicator and takes focus from the keyboard", async ({ page, example }) => {
    const stage = await example("input/basic");
    await page.keyboard.press("Tab");
    const input = stage.getByRole("textbox", { name: "Service name" });
    await expect(input).toBeFocused();
    await expect(input).toHaveCSS("outline-style", "solid");
    await expect(input).not.toHaveCSS("outline-width", "0px");
  });

  test("cannot be edited while disabled or read-only", async ({ example }) => {
    const stage = await example("input/states");
    await expect(stage.getByRole("textbox", { name: "Disabled" })).toBeDisabled();
    const readonly = stage.getByRole("textbox", { name: "Read-only" });
    await expect(readonly).not.toBeEditable();
    await readonly.focus();
    await expect(readonly).toBeFocused();
  });

  test("Enter submits the surrounding form", async ({ page, example }) => {
    const stage = await example("input/form");
    const name = stage.getByRole("textbox", { name: "Name" });

    await name.focus();
    await page.keyboard.press("Enter");
    await expect(stage.getByRole("alert")).toContainText("needs a name");
    await expect(name).toHaveAttribute("aria-invalid", "true");

    await name.fill("billing");
    await stage.getByRole("textbox", { name: "Description" }).fill("Invoices");
    await stage.getByRole("spinbutton", { name: "Port" }).fill("9000");
    await page.keyboard.press("Enter");
    await expect(stage.getByRole("status")).toHaveText("Created billing on port 9000");
    // Uncontrolled text survives the re-render.
    await expect(stage.getByRole("textbox", { name: "Description" })).toHaveValue("Invoices");
  });
});

test.describe("TextArea", () => {
  test("reports edits as the user types", async ({ example }) => {
    const stage = await example("input/text-area");
    const notes = stage.getByRole("textbox", { name: "Notes" });
    await notes.fill("Fixed");
    await expect(notes).toHaveAccessibleDescription("5 of 200 characters");
  });

  test("grows with its content up to the cap", async ({ example }) => {
    const stage = await example("input/autosize");
    const message = stage.getByRole("textbox", { name: "Message" });
    const height = async () => (await message.boundingBox())!.height;

    const one = await height();
    await message.fill("one\ntwo\nthree");
    const three = await height();
    expect(three).toBeGreaterThan(one);

    await message.fill(Array(30).fill("line").join("\n"));
    expect(await height()).toBeLessThanOrEqual(8 * 16 + 1);
  });
});

test.describe("NumberInput", () => {
  test("parses typing, treats empty as unset and clamps on blur", async ({ page, example }) => {
    const stage = await example("input/number");
    const input = stage.getByRole("spinbutton", { name: "Replicas" });
    const status = stage.getByRole("status");

    await input.fill("7");
    await expect(status).toHaveText("Replicas: 7");

    await input.fill("");
    await expect(status).toHaveText("Replicas: not set");

    await input.pressSequentially("50");
    await expect(status).toHaveText("Replicas: 50");
    await page.keyboard.press("Tab");
    await expect(status).toHaveText("Replicas: 20");
    await expect(input).toHaveValue("20");
  });

  test("ignores text that is not a number and restores the value on blur", async ({ page, example }) => {
    const stage = await example("input/number");
    const input = stage.getByRole("spinbutton", { name: "Replicas" });

    await input.fill("abc");
    await expect(stage.getByRole("status")).toHaveText("Replicas: 3");
    await page.keyboard.press("Tab");
    await expect(input).toHaveValue("3");
  });

  test("steps with the keyboard within its limits", async ({ page, example }) => {
    const stage = await example("input/number");
    const input = stage.getByRole("spinbutton", { name: "Replicas" });
    const status = stage.getByRole("status");

    await input.focus();
    await page.keyboard.press("ArrowUp");
    await expect(status).toHaveText("Replicas: 4");
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("ArrowDown");
    await expect(status).toHaveText("Replicas: 2");
    await page.keyboard.press("PageUp");
    await expect(status).toHaveText("Replicas: 12");
    await page.keyboard.press("End");
    await expect(status).toHaveText("Replicas: 20");
    await page.keyboard.press("ArrowUp");
    await expect(status).toHaveText("Replicas: 20");
    await page.keyboard.press("Home");
    await expect(status).toHaveText("Replicas: 1");
    await expect(input).toHaveAttribute("aria-valuenow", "1");
  });

  test("stepper buttons change the value, keep focus and stop at the limits", async ({ page, example }) => {
    const stage = await example("input/stepper");
    const replicas = stage.getByRole("spinbutton", { name: "Replicas" });
    const field = stage.locator(".yi-field", { has: page.getByRole("spinbutton", { name: "Replicas" }) });
    const increase = field.getByRole("button", { name: "Increase" });
    const decrease = field.getByRole("button", { name: "Decrease" });

    await replicas.focus();
    await increase.click();
    await increase.click();
    await expect(replicas).toHaveValue("5");
    await expect(replicas).toBeFocused();
    await expect(increase).toBeDisabled();
    await expect(stage.getByRole("status")).toContainText("5 replicas");

    for (let i = 0; i < 4; i++) await decrease.click();
    await expect(replicas).toHaveValue("1");
    await expect(decrease).toBeDisabled();
  });

  test("keeps decimals intact while typing and stepping", async ({ page, example }) => {
    const stage = await example("input/stepper");
    const threshold = stage.getByRole("spinbutton", { name: "Error threshold" });

    await threshold.fill("");
    await threshold.pressSequentially("0.05");
    await expect(threshold).toHaveValue("0.05");
    await expect(stage.getByRole("status")).toContainText("threshold 0.05");

    await page.keyboard.press("ArrowUp");
    await page.keyboard.press("ArrowUp");
    await expect(threshold).toHaveValue("0.15");
  });
});
