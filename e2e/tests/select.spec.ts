import type { Locator, Page } from "@playwright/test";
import { expect, test, type Theme } from "../fixtures";

/** The area around an example that its list can reach. */
async function around(page: Page, stage: Locator) {
  const box = (await stage.boundingBox())!;
  const viewport = page.viewportSize()!;
  return { x: 0, y: 0, width: Math.min(box.x + box.width, viewport.width), height: box.y + box.height + 280 };
}

test.describe("Select", () => {
  test("a press opens the list with the choice check-marked, and a press on an option chooses it", async ({
    page,
    example,
  }) => {
    const stage = await example("select/basic");
    const select = stage.getByRole("combobox", { name: "Date range" });
    const list = page.getByRole("listbox", { name: "Date range" });

    await expect(select).toHaveText("Last 7 days");
    await select.click();
    await expect(list).toBeVisible();
    await expect(select).toHaveAttribute("aria-expanded", "true");
    await expect(list.getByRole("option", { selected: true })).toHaveText("Last 7 days");

    // The list is at least as wide as the control.
    const [control, popup] = [(await select.boundingBox())!, (await list.boundingBox())!];
    expect(popup.width).toBeGreaterThanOrEqual(control.width - 2);

    await list.getByRole("option", { name: "Last 30 days" }).click();
    await expect(list).toBeHidden();
    await expect(select).toHaveText("Last 30 days");
    await expect(select).toBeFocused();
    await expect(stage.getByRole("status")).toHaveText("Showing 30d");
  });

  test("closes on a second press, a press outside and Escape, without changing the choice", async ({
    page,
    example,
  }) => {
    const stage = await example("select/basic");
    const select = stage.getByRole("combobox", { name: "Date range" });
    const list = page.getByRole("listbox");

    await select.click();
    await expect(list).toBeVisible();
    await select.click();
    await expect(list).toBeHidden();

    await select.click();
    await expect(list).toBeVisible();
    await page.mouse.click(5, 5);
    await expect(list).toBeHidden();

    await select.press("ArrowDown");
    await expect(list).toBeVisible();
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Escape");
    await expect(list).toBeHidden();
    await expect(select).toBeFocused();
    await expect(stage.getByRole("status")).toHaveText("Showing 7d");
  });

  test("works from the keyboard: arrows move, Enter and Space choose, Home and End jump", async ({
    page,
    example,
  }) => {
    const stage = await example("select/basic");
    const select = stage.getByRole("combobox", { name: "Date range" });
    const list = page.getByRole("listbox");
    const activeText = async () => {
      const id = await select.getAttribute("aria-activedescendant");
      return page.locator(`[id="${id}"]`).innerText();
    };

    await page.keyboard.press("Tab");
    await expect(select).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(list).toBeVisible();
    expect(await activeText()).toBe("Last 7 days");

    await page.keyboard.press("ArrowDown");
    expect(await activeText()).toBe("Last 30 days");
    await page.keyboard.press("Enter");
    await expect(list).toBeHidden();
    await expect(stage.getByRole("status")).toHaveText("Showing 30d");

    await page.keyboard.press("Space");
    await expect(list).toBeVisible();
    await page.keyboard.press("End");
    expect(await activeText()).toBe("Last 12 months");
    await page.keyboard.press("ArrowDown");
    expect(await activeText()).toBe("Last 12 months");
    await page.keyboard.press("Home");
    expect(await activeText()).toBe("Last 24 hours");
    await page.keyboard.press("Space");
    await expect(list).toBeHidden();
    await expect(select).toBeFocused();
    await expect(stage.getByRole("status")).toHaveText("Showing 24h");
  });

  test("typing jumps to the option starting with those letters", async ({ page, example }) => {
    const stage = await example("select/basic");
    const select = stage.getByRole("combobox", { name: "Date range" });

    await select.focus();
    await page.keyboard.type("last 9");
    await expect(page.getByRole("listbox")).toBeVisible();
    await page.keyboard.press("Enter");
    await expect(stage.getByRole("status")).toHaveText("Showing 90d");
  });

  test("Tab closes the list and moves on", async ({ page, example }) => {
    const stage = await example("select/toolbar");
    const status = stage.getByRole("combobox", { name: "Status" });

    await status.click();
    await expect(page.getByRole("listbox")).toBeVisible();
    await page.keyboard.press("Tab");
    await expect(page.getByRole("listbox")).toBeHidden();
    await expect(stage.getByRole("combobox", { name: "Date range" })).toBeFocused();
  });

  test("groups have headings, and a disabled option is skipped and cannot be chosen", async ({ page, example }) => {
    const stage = await example("select/grouped");
    const select = stage.getByRole("combobox", { name: "Region" });
    const list = page.getByRole("listbox");

    await expect(select).toHaveText("Choose a region");
    await select.press("ArrowDown");
    await expect(list.getByRole("group", { name: "Americas" }).getByRole("option")).toHaveCount(3);

    await page.keyboard.press("End");
    // Up from the last option passes over the disabled one before it.
    await page.keyboard.press("ArrowUp");
    await expect(select).toHaveAttribute("aria-activedescendant", /option-2$/);

    await list.getByRole("option", { name: "GovCloud" }).click({ force: true });
    await expect(list).toBeVisible();
    await expect(stage.getByRole("status")).toHaveText("No region chosen");

    await page.keyboard.press("Enter");
    await expect(select).toHaveText("Virginia");
    await expect(stage.getByRole("status")).toHaveText("Region us-east-1");
  });

  test("a disabled select does not open", async ({ page, example }) => {
    const stage = await example("select/states");
    const select = stage.getByRole("combobox", { name: "Disabled", exact: true });
    await expect(select).toBeDisabled();
    await select.click({ force: true });
    await expect(page.getByRole("listbox")).toBeHidden();
  });

  test("submits its value in a native form", async ({ page, example }) => {
    const stage = await example("select/form");
    await stage.getByRole("combobox", { name: "Plan" }).click();
    await page.getByRole("option", { name: "Enterprise" }).click();
    await expect(stage.locator('input[name="plan"]')).toHaveValue("enterprise");
    await stage.getByRole("button", { name: "Save" }).click();
    await expect(stage.getByRole("status")).toHaveText("Saved plan=enterprise");
  });
});

test.describe("Combobox", () => {
  test("typing narrows the list, and Enter chooses the highlighted match", async ({ page, example }) => {
    const stage = await example("select/combobox");
    const input = stage.getByRole("combobox", { name: "Time zone" });
    const list = page.getByRole("listbox", { name: "Time zone" });

    await expect(input).toHaveValue("Europe/Dublin");
    await input.fill("america");
    await expect(list.getByRole("option")).toHaveCount(3);
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Enter");
    await expect(list).toBeHidden();
    await expect(input).toHaveValue("America/New York");
    await expect(input).toBeFocused();
    await expect(stage.getByRole("status")).toHaveText("Times shown in America/New_York");

    // The description is searched too, and every word must match.
    await input.fill("asia +09");
    await expect(list.getByRole("option")).toHaveText(["Asia/TokyoUTC+09:00"]);
  });

  test("a press opens the whole list, and a press on an option chooses it", async ({ page, example }) => {
    const stage = await example("select/combobox");
    const input = stage.getByRole("combobox", { name: "Time zone" });
    const list = page.getByRole("listbox");

    await input.click();
    await expect(list.getByRole("option")).toHaveCount(10);
    await expect(list.getByRole("option", { selected: true })).toContainText("Europe/Dublin");
    await list.getByRole("option", { name: "Asia/Tokyo" }).click();
    await expect(list).toBeHidden();
    await expect(input).toHaveValue("Asia/Tokyo");
    await expect(input).toBeFocused();
  });

  test("leaving without choosing puts the choice back, as does Escape twice", async ({ page, example }) => {
    const stage = await example("select/combobox");
    const input = stage.getByRole("combobox", { name: "Time zone" });

    await input.fill("zzz");
    await expect(page.getByRole("status").filter({ hasText: "No matches" })).toBeVisible();
    await page.keyboard.press("Tab");
    await expect(input).toHaveValue("Europe/Dublin");

    await input.fill("tok");
    await expect(page.getByRole("listbox")).toBeVisible();
    await page.keyboard.press("Escape");
    await expect(page.getByRole("listbox")).toBeHidden();
    await expect(input).toHaveValue("tok");
    await page.keyboard.press("Escape");
    await expect(input).toHaveValue("Europe/Dublin");
    await expect(stage.getByRole("status")).toHaveText("Times shown in Europe/Dublin");
  });

  test("allow_custom accepts text that matches no option", async ({ page, example }) => {
    const stage = await example("select/custom");
    const input = stage.getByRole("combobox", { name: "Branch" });

    await input.fill("feature/audit-log");
    await page.keyboard.press("Enter");
    await expect(stage.getByRole("status")).toHaveText("Deploying from feature/audit-log");
    await expect(input).toHaveValue("feature/audit-log");

    await input.fill("rel");
    await page.getByRole("option", { name: "release/2.13" }).click();
    await expect(stage.getByRole("status")).toHaveText("Deploying from release/2.13");

    await input.fill("hotfix");
    await page.keyboard.press("Tab");
    await expect(stage.getByRole("status")).toHaveText("Deploying from hotfix");
  });

  test("a long list renders only the first matches and counts the rest", async ({ page, example }) => {
    const stage = await example("select/large");
    const input = stage.getByRole("combobox", { name: "Host" });
    const list = page.getByRole("listbox");

    await input.click();
    await expect(list.getByRole("option")).toHaveCount(50);
    await expect(page.getByText("Showing the first 50 of 5000")).toBeVisible();

    await input.fill("web-4999");
    await expect(list.getByRole("option")).toHaveCount(1);
    await page.keyboard.press("Enter");
    await expect(stage.getByRole("status")).toHaveText("Host web-4999.us-west.example.net");
  });

  for (const theme of ["light", "dark"] as Theme[]) {
    test(`looks right when open: ${theme}`, async ({ page, example }) => {
      const basic = await example("select/basic", { theme });
      await basic.getByRole("combobox").click();
      await page.keyboard.press("ArrowDown");
      await expect(page.getByRole("listbox")).toBeVisible();
      await expect(page).toHaveScreenshot(["select", `open-basic-${theme}.png`], { clip: await around(page, basic) });

      const grouped = await example("select/grouped", { theme });
      await grouped.getByRole("combobox").press("ArrowDown");
      await expect(page.getByRole("listbox")).toBeVisible();
      await expect(page).toHaveScreenshot(["select", `open-grouped-${theme}.png`], {
        clip: await around(page, grouped),
      });

      const combobox = await example("select/combobox", { theme });
      await combobox.getByRole("combobox").fill("europe");
      await expect(page.getByRole("listbox")).toBeVisible();
      await expect(page).toHaveScreenshot(["select", `open-combobox-${theme}.png`], {
        clip: await around(page, combobox),
      });

      const large = await example("select/large", { theme });
      await large.getByRole("combobox").fill("eu-west");
      await expect(page.getByRole("listbox")).toBeVisible();
      await expect(page).toHaveScreenshot(["select", `open-large-${theme}.png`], { clip: await around(page, large) });
    });
  }
});
