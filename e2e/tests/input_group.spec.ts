import { expect, test } from "../fixtures";

test.describe("InputGroup", () => {
  test("pressing an affix focuses the control and the group shows the focus", async ({ page, example }) => {
    const stage = await example("input-group/affixes");
    const input = stage.getByRole("textbox", { name: "Address" });
    const group = stage.locator(".yi-input-group", { has: page.getByRole("textbox", { name: "Address" }) });

    await group.getByText(".example.com").click();
    await expect(input).toBeFocused();
    await expect(group).toHaveCSS("outline-style", "solid");
    await expect(group).not.toHaveCSS("outline-width", "0px");

    await input.fill("billing");
    await expect(stage.getByRole("status")).toContainText("https://billing.example.com");
  });

  test("keeps number input behaviour inside a group", async ({ page, example }) => {
    const stage = await example("input-group/affixes");
    const timeout = stage.getByRole("spinbutton", { name: "Timeout" });

    await timeout.focus();
    await page.keyboard.press("ArrowUp");
    await expect(stage.getByRole("status")).toContainText("timeout 300 ms");
  });

  test("an attached submit button works by click and by Enter", async ({ page, example }) => {
    const stage = await example("input-group/button");
    const email = stage.getByRole("textbox", { name: "Invite a teammate" });
    const status = stage.getByRole("status");

    await email.fill("ada@example.com");
    await stage.getByRole("button", { name: "Invite" }).click();
    await expect(status).toHaveText("Invited ada@example.com");
    await expect(email).toHaveValue("");

    await email.fill("grace@example.com");
    await page.keyboard.press("Enter");
    await expect(status).toHaveText("Invited grace@example.com");
  });

  test("an attached button is reachable by keyboard and acts on the field", async ({ page, example }) => {
    const stage = await example("input-group/button");
    const token = stage.getByRole("textbox", { name: "Deploy token" });
    const before = await token.inputValue();

    await token.focus();
    await page.keyboard.press("Tab");
    const regenerate = stage.getByRole("button", { name: "Regenerate" });
    await expect(regenerate).toBeFocused();
    await expect(regenerate).toHaveCSS("outline-style", "solid");
    await page.keyboard.press("Enter");
    await expect(token).not.toHaveValue(before);
  });

  test("an attached dropdown reports the chosen option", async ({ page, example }) => {
    const stage = await example("input-group/dropdown");
    const status = stage.getByRole("status");
    await expect(status).toContainText("https://hooks.example.com");

    const protocol = stage.getByRole("combobox", { name: "Protocol" });
    await protocol.click();
    await stage.getByRole("option", { name: /http:\/\// }).click();
    await expect(status).toContainText("http://hooks.example.com");
    await expect(protocol).toHaveText("http://");
    await expect(protocol).toBeFocused();
    await expect(stage.getByRole("textbox", { name: "Webhook URL" })).not.toBeFocused();

    const unit = stage.getByRole("combobox", { name: "Unit" });
    await unit.focus();
    await expect(unit).toHaveCSS("outline-style", "solid");
    await page.keyboard.press("ArrowDown");
    await expect(status).toContainText("retry in hr");
  });

  test("an attached menu opens from the keyboard and acts on the value", async ({ page, example }) => {
    const stage = await example("input-group/menu");
    const template = stage.getByRole("textbox", { name: "Message template" });
    const insert = stage.getByRole("button", { name: "Insert" });

    await template.focus();
    await page.keyboard.press("Tab");
    await expect(insert).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(stage.getByRole("menuitem", { name: "Service name" })).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Enter");
    await expect(template).toHaveValue("Deployed {version}");

    await insert.click();
    await stage.getByRole("menuitem", { name: "Author" }).click();
    await expect(stage.getByRole("status")).toHaveText("Template: Deployed {version}{author}");
  });

  test("open chip menus look right in place", async ({ page, example }) => {
    for (const theme of ["light", "dark"] as const) {
      const stage = await example("input-group/dropdown", { theme });
      await stage.getByRole("combobox", { name: "Protocol" }).click();
      await expect(stage.getByRole("option", { name: /https/ })).toBeVisible();
      await expect(page).toHaveScreenshot(["input-group", `open-select-${theme}-desktop.png`]);
      const menu = await example("input-group/menu", { theme });
      await menu.getByRole("button", { name: "Insert" }).click();
      await expect(menu.getByRole("menuitem").first()).toBeVisible();
      await expect(page).toHaveScreenshot(["input-group", `open-menu-${theme}-desktop.png`]);
    }
  });

  test("takes its state from the control inside", async ({ example }) => {
    const stage = await example("input-group/states");
    await expect(stage.getByRole("textbox", { name: "Disabled", exact: true })).toBeDisabled();
    await expect(stage.getByRole("textbox", { name: "Read-only" })).not.toBeEditable();
    await expect(stage.getByRole("textbox", { name: "Invalid" })).toHaveAccessibleDescription(/already taken/);
    await expect(stage.getByRole("searchbox", { name: "Disabled search" })).toBeDisabled();
    await expect(stage.getByRole("button", { name: "Clear search" })).toBeHidden();
  });
});

test.describe("SearchInput", () => {
  test("filters as the user types and clears with the button", async ({ example }) => {
    const stage = await example("input-group/search");
    const search = stage.getByRole("searchbox", { name: "Search services" });
    const status = stage.getByRole("status");
    const clear = stage.getByRole("button", { name: "Clear search" });

    await expect(clear).toBeHidden();
    await search.fill("checkout");
    await expect(status).toHaveText("2 of 6 services");
    await expect(stage.getByRole("listitem")).toHaveText(["checkout-api", "checkout-web"]);

    await clear.click();
    await expect(search).toHaveValue("");
    await expect(search).toBeFocused();
    await expect(status).toHaveText("6 of 6 services");
    await expect(clear).toBeHidden();
  });

  test("Escape clears the query", async ({ page, example }) => {
    const stage = await example("input-group/search");
    const search = stage.getByRole("searchbox", { name: "Search services" });

    await page.keyboard.press("Tab");
    await expect(search).toBeFocused();
    await page.keyboard.type("web");
    await expect(stage.getByRole("status")).toHaveText("2 of 6 services");
    await page.keyboard.press("Escape");
    await expect(search).toHaveValue("");
    await expect(stage.getByRole("status")).toHaveText("6 of 6 services");
  });

  test("runs the search on Enter or from the attached button, within a scope", async ({ page, example }) => {
    const stage = await example("input-group/search-bar");
    const search = stage.getByRole("searchbox", { name: "Search events" });
    const status = stage.getByRole("status");

    await search.fill("timeout");
    await expect(status).toHaveText("No search yet");
    await page.keyboard.press("Enter");
    await expect(status).toHaveText("Searched all for “timeout”");

    await stage.getByRole("combobox", { name: "Scope" }).selectOption("errors");
    await search.fill("oom");
    await stage.getByRole("button", { name: "Search", exact: true }).click();
    await expect(status).toHaveText("Searched errors for “oom”");

    await search.focus();
    await page.keyboard.press("Escape");
    await expect(status).toHaveText("No search yet");
  });

  test("marks itself busy while loading", async ({ page, example }) => {
    const stage = await example("input-group/search-bar");
    const group = stage.locator(".yi-input-group", { has: page.getByRole("searchbox", { name: "Search logs" }) });
    await expect(group).toHaveAttribute("aria-busy", "true");
  });
});
