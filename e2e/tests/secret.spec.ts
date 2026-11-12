import type { Page } from "@playwright/test";
import { expect, test } from "../fixtures";

const TOKEN = "rtk_live_9Zp4mQ2vX7cL1sT8bN5wK3hJ6dF0gR4yU2aE7iO1pM8nB5vC3xZ6lK9jH";

/** What is on the clipboard, read through a handle that survives the stubs below. */
const clipboardText = (page: Page) => page.evaluate(() => (window as any).__clipboard.readText());

test.describe("SecretInput", () => {
  test("masks typing until the show button is pressed, keeping focus in the field", async ({ example }) => {
    const stage = await example("secret/basic");
    const input = stage.getByLabel("API token");
    const show = stage.getByRole("button", { name: "Show secret" });

    await input.fill("tok_short");
    await expect(input).toHaveAttribute("type", "password");
    await expect(stage.getByRole("status")).toHaveText("9 characters entered");
    await expect(stage.getByRole("alert")).toContainText("at least 20 characters");
    await expect(input).toHaveAttribute("aria-invalid", "true");

    await show.click();
    await expect(input).toHaveAttribute("type", "text");
    await expect(show).toHaveAttribute("aria-pressed", "true");
    await expect(input).toBeFocused();
    await expect(input).toHaveValue("tok_short");

    await show.click();
    await expect(input).toHaveAttribute("type", "password");
    await expect(show).toHaveAttribute("aria-pressed", "false");
  });

  test("is operable from the keyboard with a visible focus indicator", async ({ page, example }) => {
    const stage = await example("secret/basic");
    const input = stage.getByLabel("API token");
    const show = stage.getByRole("button", { name: "Show secret" });

    await page.keyboard.press("Tab");
    await expect(input).toBeFocused();
    await expect(stage.locator(".yi-secret-input")).not.toHaveCSS("outline-width", "0px");

    await page.keyboard.type("tok_live_0123456789abcdef");
    await page.keyboard.press("Tab");
    await expect(show).toBeFocused();
    await expect(show).not.toHaveCSS("outline-style", "none");

    await page.keyboard.press("Space");
    await expect(input).toHaveAttribute("type", "text");
    await expect(show).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(input).toHaveAttribute("type", "password");
  });

  test("generates a strong secret, reveals it and reports it", async ({ example }) => {
    const stage = await example("secret/generator");
    const input = stage.getByLabel("Webhook secret");
    const generate = stage.getByRole("button", { name: "Generate a new secret" });

    await generate.click();
    await expect(input).toHaveAttribute("type", "text");
    await expect(input).toHaveValue(/^[A-Za-z0-9_-]{43}$/);
    await expect(stage.getByRole("status")).toHaveText("Saved a secret of 43 characters");

    const first = await input.inputValue();
    await generate.click();
    await expect(input).not.toHaveValue(first);
  });

  test("a read-only secret can be shown but not changed, and a disabled one neither", async ({ example }) => {
    const stage = await example("secret/states");

    const readonly = stage.getByRole("group").or(stage.locator(".yi-field")).filter({ hasText: "Read-only" });
    await expect(readonly.getByLabel("Read-only")).not.toBeEditable();
    await expect(readonly.getByRole("button", { name: "Generate a new secret" })).toBeDisabled();
    await readonly.getByRole("button", { name: "Show secret" }).click();
    await expect(readonly.getByLabel("Read-only")).toHaveAttribute("type", "text");

    const disabled = stage.locator(".yi-field").filter({ hasText: "Disabled" });
    await expect(disabled.getByLabel("Disabled")).toBeDisabled();
    await expect(disabled.getByRole("button", { name: "Show secret" })).toBeDisabled();

    const writeOnly = stage.locator(".yi-field").filter({ hasText: "Write-only" });
    await expect(writeOnly.getByRole("button")).toHaveCount(0);
  });
});

test.describe("SecretValue", () => {
  test.beforeEach(async ({ context, page }) => {
    await context.grantPermissions(["clipboard-read", "clipboard-write"]);
    await page.addInitScript(() => ((window as any).__clipboard = navigator.clipboard));
  });

  test("a concealed value is absent until shown, and copies either way", async ({ page, example }) => {
    const stage = await example("secret/value");
    const secret = stage.getByRole("group", { name: "Client secret" });

    await expect(secret).not.toContainText("cs_9Zp4");
    await secret.getByRole("button", { name: "Copy" }).click();
    await expect(stage.getByRole("status")).toHaveText("Copied the client secret");
    expect(await clipboardText(page)).toMatch(/^cs_9Zp4/);

    const show = secret.getByRole("button", { name: "Show value" });
    await show.click();
    await expect(secret).toContainText("cs_9Zp4mQ2vX7cL1sT8bN5wK3hJ6dF0gR4yU2aE7iO1");
    await expect(show).toHaveAttribute("aria-pressed", "true");
    await show.click();
    await expect(secret).not.toContainText("cs_9Zp4");
  });

  test("shows the value when the clipboard is refused, to copy by hand", async ({ page, example }) => {
    await page.addInitScript(() => {
      Object.defineProperty(navigator, "clipboard", { value: undefined });
      document.execCommand = () => false;
    });
    const stage = await example("secret/value");
    const secret = stage.getByRole("group", { name: "Client secret" });

    await secret.getByRole("button", { name: "Copy" }).click();
    await expect(secret).toContainText("cs_9Zp4mQ2vX7cL1sT8bN5wK3hJ6dF0gR4yU2aE7iO1");
    await expect(stage.getByRole("status")).toHaveText("Nothing copied yet");
  });
});

test.describe("SecretReveal", () => {
  test.beforeEach(async ({ context, page }) => {
    await context.grantPermissions(["clipboard-read", "clipboard-write"]);
    await page.addInitScript(() => ((window as any).__clipboard = navigator.clipboard));
  });

  test("presents a new secret to copy and stays until dismissed", async ({ page, example }) => {
    await page.clock.install();
    const stage = await example("secret/reveal");

    await stage.getByRole("button", { name: "Create token" }).click();
    const panel = stage.getByRole("region", { name: "Token created" });
    await expect(panel).toContainText("only time");
    await expect(panel).toContainText(TOKEN);

    await panel.getByRole("button", { name: "Copy" }).click();
    expect(await clipboardText(page)).toBe(TOKEN);

    // Never closes on its own.
    await page.clock.runFor(120_000);
    await expect(panel).toBeVisible();

    await panel.getByRole("button", { name: "Done" }).click();
    await expect(panel).toHaveCount(0);
    await expect(stage.getByRole("status")).toHaveText("No token on screen");
  });

  test("moves the aside below the values in a narrow container", async ({ page, example }) => {
    const stage = await example("secret/aside");
    const values = stage.getByRole("group", { name: "Password" });
    const code = stage.getByRole("img", { name: "Enrolment link for field-07" });

    await page.setViewportSize({ width: 1100, height: 800 });
    expect((await code.boundingBox())!.x).toBeGreaterThan((await values.boundingBox())!.x + 100);

    await page.setViewportSize({ width: 380, height: 800 });
    expect((await code.boundingBox())!.y).toBeGreaterThan((await values.boundingBox())!.y);
    expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(true);
  });
});
