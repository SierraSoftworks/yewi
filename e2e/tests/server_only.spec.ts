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

  test("on a narrow screen the menu button opens the navigation in a drawer", async ({ page, serverOnly }) => {
    await page.setViewportSize({ width: 390, height: 800 });
    const stage = await serverOnly("app-shell/grouped");
    const menu = stage.getByRole("button", { name: "Navigation" }).first();

    await expect(stage.getByText("Devices").filter({ visible: true })).toHaveCount(0);
    await menu.click();
    const drawer = page.getByRole("dialog", { name: "Navigation" });
    await expect(drawer).toBeVisible();
    await expect(drawer.getByText("Devices")).toBeVisible();

    await page.keyboard.press("Escape");
    await expect(drawer).toBeHidden();
    await expect(menu).toBeFocused();
  });

  test("on a wide screen the menu button hides the sidebar, and that is remembered", async ({ page, serverOnly }) => {
    await page.setViewportSize({ width: 1280, height: 800 });
    const stage = await serverOnly("app-shell/grouped");
    const menu = stage.getByRole("button", { name: "Navigation" }).first();
    const sidebar = stage.locator(".yi-app-shell__sidebar");

    await expect(sidebar).toBeVisible();
    await menu.click();
    await expect(sidebar).toBeHidden();
    await expect(menu).toHaveAttribute("aria-expanded", "false");
    await expect(page.getByRole("dialog", { name: "Navigation" })).toBeHidden();

    await page.reload();
    await expect(sidebar).toBeHidden();
    await menu.click();
    await expect(sidebar).toBeVisible();
    await expect(menu).toHaveAttribute("aria-expanded", "true");
  });

  for (const [id, button, first, second] of [
    ["menu-button/basic", "Actions", "Rename", "Duplicate"],
    ["split-button/basic", "More ways to deploy", "Deploy to staging", "Deploy a canary"],
    ["top-menu/collapsed", "Menu", "Services", "Incidents"],
  ]) {
    test(`a menu opens under its button and is worked from the keyboard: ${id}`, async ({ page, serverOnly }) => {
      const stage = await serverOnly(id);
      const trigger = stage.getByRole("button", { name: button, exact: true });
      const item = (name: string) => stage.getByRole("menuitem", { name, exact: true });

      await expect(item(first)).toBeHidden();
      await trigger.click();
      await expect(item(first)).toBeFocused();
      await expect(trigger).toHaveAttribute("aria-expanded", "true");
      const [from, menu] = [await trigger.boundingBox(), await stage.getByRole("menu").boundingBox()];
      expect(menu!.y).toBeGreaterThanOrEqual(from!.y + from!.height);

      await page.keyboard.press("ArrowDown");
      await expect(item(second)).toBeFocused();
      await page.keyboard.press("Home");
      await expect(item(first)).toBeFocused();
      await page.keyboard.press(second[0]);
      await expect(item(second)).toBeFocused();

      await page.keyboard.press("Escape");
      await expect(item(first)).toBeHidden();
      await expect(trigger).toBeFocused();
      await expect(trigger).toHaveAttribute("aria-expanded", "false");

      await page.keyboard.press("ArrowUp");
      await expect(stage.getByRole("menuitem").last()).toBeFocused();
      await trigger.click();
      await expect(item(first)).toBeHidden();
    });
  }

  test("a popover opens from its trigger and closes on a second press or a press outside", async ({ page, serverOnly }) => {
    const stage = await serverOnly("popover/basic");
    const trigger = stage.locator("[data-yi-popover=click]").first();
    const surface = trigger.locator("[popover]");

    await trigger.locator("button, [tabindex]").first().click();
    await expect(surface).toBeVisible();
    await trigger.locator("button, [tabindex]").first().click();
    await expect(surface).toBeHidden();

    await trigger.locator("button, [tabindex]").first().click();
    await expect(surface).toBeVisible();
    await page.mouse.click(5, 500);
    await expect(surface).toBeHidden();
  });

  test("a tooltip shows on hover and on keyboard focus, and Escape hides it", async ({ page, serverOnly }) => {
    const stage = await serverOnly("tooltip/basic");
    const tip = stage.getByRole("tooltip").first();
    const described = stage.locator("[data-yi-popover=hover]").first();

    await expect(tip).toBeHidden();
    await described.hover();
    await expect(tip).toBeVisible();
    await page.mouse.move(5, 500);
    await expect(tip).toBeHidden();

    await page.keyboard.press("Tab");
    await expect(tip).toBeVisible();
    await expect(page.locator(":focus")).toHaveAttribute("aria-describedby", /.+/);
    await page.keyboard.press("Escape");
    await expect(tip).toBeHidden();
  });

  test("menu items follow links, submit forms and open dialogs", async ({ page, serverOnly }) => {
    const stage = await serverOnly("server-only/menu");
    const choose = async (name: string) => {
      await stage.getByRole("button", { name: "Host" }).click();
      await stage.getByRole("menuitem", { name }).click();
    };

    await choose("Remove");
    await expect(page.getByRole("alertdialog", { name: "Remove pi-1?" })).toBeVisible();
    await expect(stage.getByRole("menu")).toBeHidden();
    await page.keyboard.press("Escape");

    await choose("Restart");
    await expect(page).toHaveURL(/\?action=restart$/);
    await choose("Logs");
    await expect(page).toHaveURL(/\?opened=logs$/);
  });

  test("a file drop names the files chosen or dropped, and its form sends them", async ({ page, serverOnly }) => {
    const stage = await serverOnly("server-only/upload");
    const [many, single] = [stage.locator("label").nth(0), stage.locator("label").nth(1)];
    const file = (name: string) => ({ name, mimeType: "application/zip", buffer: Buffer.from(name) });

    await many.locator("input").setInputFiles([file("a.zip"), file("b.zip")]);
    await expect(many).toContainText("a.zip, b.zip");

    const dropped = await page.evaluateHandle(() => {
      const data = new DataTransfer();
      for (const name of ["c.zip", "d.zip"]) data.items.add(new File([name], name, { type: "application/zip" }));
      return data;
    });
    await single.dispatchEvent("dragover", { dataTransfer: dropped });
    await single.dispatchEvent("drop", { dataTransfer: dropped });
    await expect(single).toContainText("c.zip");
    await expect(single).not.toContainText("d.zip");

    const sends = await stage.locator("form").evaluate((form: HTMLFormElement) =>
      [...new FormData(form).entries()].map(([name, value]) => `${name}=${(value as File).name}`),
    );
    expect(sends).toEqual(["bundle=a.zip", "bundle=b.zip", "single=c.zip"]);
  });

  test("tabs switch panels by press and by arrow key", async ({ page, serverOnly }) => {
    const stage = await serverOnly("tabs/basic");
    const tab = (name: string) => stage.getByRole("tab", { name });
    const panel = stage.getByRole("tabpanel");

    await expect(panel).toContainText("Name, callsign");
    await tab("Devices").click();
    await expect(panel).toHaveCount(1);
    await expect(panel).toContainText("Phones and radios");
    await expect(tab("Devices")).toHaveAttribute("aria-selected", "true");
    await expect(tab("Profile")).toHaveAttribute("aria-selected", "false");

    await page.keyboard.press("ArrowRight");
    await expect(tab("Credentials")).toBeFocused();
    await expect(panel).toContainText("Client certificates");
    await page.keyboard.press("Home");
    await expect(panel).toContainText("Name, callsign");
  });
});
