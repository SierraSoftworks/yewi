import type { Page } from "@playwright/test";
import { expect, test } from "../fixtures";

/** What is on the clipboard. */
const clipboardText = (page: Page) => page.evaluate(() => navigator.clipboard.readText());

test.describe("Server-only pages", () => {
  test.beforeEach(async ({ context }) => {
    await context.grantPermissions(["clipboard-read", "clipboard-write"]);
  });

  test("a copy button copies, confirms and returns to idle", async ({ page, serverOnly, problems }) => {
    const stage = await serverOnly("copyable/button");
    const copy = stage.getByRole("button", { name: "Copy command" }).first();

    await copy.click();
    expect(await clipboardText(page)).toBe("nomad job run caddy.hcl");
    await expect(copy).toHaveAttribute("data-yi-state", "copied");
    await expect(copy).toHaveAttribute("title", "Copied");
    await expect(page.getByRole("status").filter({ hasText: /^Copied$/ })).toBeAttached();
    await expect(copy.locator("svg:visible")).toHaveCount(1);

    await expect(copy).not.toHaveAttribute("data-yi-state", /.+/, { timeout: 4000 });
    await expect(copy).toHaveAttribute("title", "Copy command");
    await expect(copy.locator("svg:visible")).toHaveCount(1);
    expect(problems).toEqual([]);
  });

  test("a copy button works from the keyboard, and a disabled one does nothing", async ({ page, serverOnly }) => {
    const stage = await serverOnly("copyable/button");
    await page.evaluate(() => navigator.clipboard.writeText("untouched"));

    await stage.getByRole("button", { name: "Copy", exact: true }).click({ force: true });
    expect(await clipboardText(page)).toBe("untouched");

    const labelled = stage.getByRole("button", { name: "Copy command" }).nth(1);
    await labelled.focus();
    await page.keyboard.press("Enter");
    await expect(labelled).toHaveAttribute("data-yi-state", "copied");
    expect(await clipboardText(page)).toBe("nomad job run caddy.hcl");
  });

  for (const example of ["code-block/basic", "code-block/lines"]) {
    test(`a code block copies exactly the code it shows: ${example}`, async ({ page, serverOnly }) => {
      const stage = await serverOnly(example);
      const block = stage.locator(".yi-code-block").first();
      const shown = await block.locator("pre > code").evaluate((code) => code.textContent);

      await block.getByRole("button", { name: "Copy" }).click();
      await expect(block.getByRole("button", { name: "Copy" })).toHaveAttribute("data-yi-state", "copied");
      expect(shown).toContain("\n");
      expect(await clipboardText(page)).toBe(shown);
    });
  }

  test("stepper buttons change the value, keep focus and stop at the limits", async ({ page, serverOnly }) => {
    const stage = await serverOnly("input/stepper");
    const replicas = stage.getByRole("spinbutton", { name: "Replicas" });
    const field = stage.locator(".yi-field", { has: page.getByRole("spinbutton", { name: "Replicas" }) });
    const increase = field.getByRole("button", { name: "Increase" });
    const decrease = field.getByRole("button", { name: "Decrease" });

    await replicas.focus();
    await increase.click();
    await increase.click();
    await expect(replicas).toHaveValue("5");
    await expect(replicas).toBeFocused();
    await expect(replicas).toHaveAttribute("aria-valuenow", "5");
    await expect(increase).toBeDisabled();

    for (let i = 0; i < 4; i++) await decrease.click({ force: true });
    await expect(replicas).toHaveValue("1");
    await expect(decrease).toBeDisabled();
    await expect(increase).toBeEnabled();
  });

  test("a number steps from the keyboard, keeps its decimals and clamps on leaving", async ({ page, serverOnly }) => {
    const stage = await serverOnly("input/stepper");
    const threshold = stage.getByRole("spinbutton", { name: "Error threshold" });

    await threshold.focus();
    await page.keyboard.press("ArrowUp");
    await expect(threshold).toHaveValue("0.55");
    await page.keyboard.press("PageDown");
    await expect(threshold).toHaveValue("0.05");
    await page.keyboard.press("End");
    await expect(threshold).toHaveValue("1");
    await page.keyboard.press("Home");
    await expect(threshold).toHaveValue("0");

    await threshold.fill("7");
    await page.keyboard.press("Tab");
    await expect(threshold).toHaveValue("1");

    // A field without a stepper still steps from the keyboard.
    const port = stage.getByRole("spinbutton", { name: "Port" });
    await port.focus();
    await page.keyboard.press("ArrowUp");
    await expect(port).toHaveValue("8081");
  });

  test("the theme toggle cycles the theme, names the next step and is remembered", async ({ page, serverOnly }) => {
    const stage = await serverOnly("theme/toggle");
    const toggle = stage.getByRole("button", { name: /^Theme:/ }).first();
    const root = page.locator("html");

    await expect(toggle).toHaveAccessibleName("Theme: System. Switch to Light.");
    await toggle.click();
    await expect(root).toHaveAttribute("data-theme", "light");
    await expect(toggle).toHaveAccessibleName("Theme: Light. Switch to Dark.");
    await toggle.click();
    await expect(root).toHaveAttribute("data-theme", "dark");

    await page.reload();
    await expect(root).toHaveAttribute("data-theme", "dark");
    await expect(toggle).toHaveAccessibleName("Theme: Dark. Switch to System.");
    await toggle.click();
    await expect(root).not.toHaveAttribute("data-theme", /.+/);
    expect(await page.evaluate(() => localStorage.getItem("yewi-theme"))).toBeNull();
  });

  test("a button opens a dialog and a drawer, which close natively", async ({ page, serverOnly, problems }) => {
    const stage = await serverOnly("server-only/dialogs");

    // The dialog the server rendered open is showing, as a modal.
    const notice = page.getByRole("dialog", { name: "Session expired" });
    await expect(notice).toBeVisible();
    expect(await notice.evaluate((dialog) => dialog.matches(":modal"))).toBe(true);
    await notice.getByRole("button", { name: "Close" }).click();
    await expect(notice).toBeHidden();

    const trigger = stage.getByRole("button", { name: "Details" });
    await trigger.click();
    const details = page.getByRole("dialog", { name: "Host details" });
    await expect(details).toContainText("pi-1 has been up for 12 days.");
    expect(await details.evaluate((dialog) => dialog.matches(":modal"))).toBe(true);
    await page.keyboard.press("Escape");
    await expect(details).toBeHidden();
    await expect(trigger).toBeFocused();

    await stage.getByRole("button", { name: "Filters" }).click();
    const filters = page.getByRole("dialog", { name: "Filters" });
    await expect(filters).toBeVisible();
    await filters.getByRole("button", { name: "Close" }).click();
    await expect(filters).toBeHidden();
    expect(problems).toEqual([]);
  });

  test("a confirmation submits its form only once its steps are done", async ({ page, serverOnly }) => {
    const stage = await serverOnly("server-only/confirm");
    await stage.getByRole("button", { name: "Forget pi-1" }).click();
    const dialog = page.getByRole("alertdialog", { name: "Forget pi-1?" });
    await expect(dialog).toBeVisible();
    const confirm = dialog.getByRole("button", { name: "Forget pi-1" });

    // Neither step is done: the browser refuses the submit.
    await confirm.click();
    await expect(dialog).toBeVisible();
    expect(new URL(page.url()).search).toBe("");

    await dialog.getByRole("checkbox", { name: "I have drained this host" }).check();
    await dialog.getByRole("textbox").fill("pi-2");
    await confirm.click();
    expect(new URL(page.url()).search).toBe("");

    await dialog.getByRole("textbox").fill("pi-1");
    await confirm.click();
    await expect(page).toHaveURL(/\?forget=pi-1$/);
  });

  test("cancelling a confirmation closes it and submits nothing", async ({ page, serverOnly }) => {
    const stage = await serverOnly("server-only/confirm");
    await stage.getByRole("button", { name: "Forget pi-1" }).click();
    const dialog = page.getByRole("alertdialog", { name: "Forget pi-1?" });
    await dialog.getByRole("button", { name: "Cancel" }).click();
    await expect(dialog).toBeHidden();
    expect(new URL(page.url()).search).toBe("");
  });
});
