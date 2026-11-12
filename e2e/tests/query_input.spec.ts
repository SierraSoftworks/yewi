import type { Locator, Page } from "@playwright/test";
import { expect, test, type Theme } from "../fixtures";

/** The example plus the room under it that its suggestions occupy. */
async function around(page: Page, stage: Locator, below = 150) {
  const box = (await stage.boundingBox())!;
  const viewport = page.viewportSize()!;
  return { x: 0, y: 0, width: Math.min(box.x + box.width, viewport.width), height: box.y + box.height + below };
}

test.describe("QueryInput", () => {
  test("suggests fields, then values, and completes them from the keyboard", async ({ page, example }) => {
    const stage = await example("query-input/basic");
    const input = stage.getByRole("combobox", { name: "Search activity" });
    const list = page.getByRole("listbox", { name: "Suggestions" });
    const options = list.getByRole("option");

    await page.keyboard.press("Tab");
    await expect(input).toBeFocused();
    await expect(input).toHaveAttribute("aria-expanded", "true");
    await expect(options).toHaveText([/^workflow:/, /^status:/, /^partition:/]);
    await expect(options.nth(1)).toContainText("Match the last run");

    await page.keyboard.type("st");
    await expect(options).toHaveText([/^status:/]);

    // Nothing is highlighted until asked for.
    await expect(input).not.toHaveAttribute("aria-activedescendant", /.+/);
    await page.keyboard.press("ArrowDown");
    await expect(options.first()).toHaveAttribute("aria-selected", "true");
    await expect(input).toHaveAttribute("aria-activedescendant", (await options.first().getAttribute("id"))!);
    await page.keyboard.press("Enter");
    await expect(input).toHaveValue("status:");
    await expect(options).toHaveText(["succeeded", "failed", "running"]);

    await page.keyboard.type("i");
    await expect(options).toHaveText(["failed", "running"]);
    await page.keyboard.press("ArrowUp");
    await expect(options.last()).toHaveAttribute("aria-selected", "true");
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Tab");
    await expect(input).toHaveValue("status:failed ");
    await expect(input).toBeFocused();

    // Typing carries on after the completed term, and Enter runs the query.
    await page.keyboard.type("budget");
    await expect(list).toBeHidden();
    await expect(input).toHaveAttribute("aria-expanded", "false");
    await page.keyboard.press("Enter");
    await expect(stage.getByRole("status")).toHaveText("Searched for status:failed budget");
  });

  test("completes the token at the caret and leaves the rest alone", async ({ page, example }) => {
    const stage = await example("query-input/basic");
    const input = stage.getByRole("combobox");
    const options = page.getByRole("option");

    await input.fill("part budget");
    await page.keyboard.press("Home");
    for (let i = 0; i < 4; i++) await page.keyboard.press("ArrowRight");
    await expect(options).toHaveText([/^partition:/]);
    await options.first().click();
    await expect(input).toHaveValue("partition: budget");
    await expect(input).toBeFocused();

    // The caret sits after the scope, so its values come next.
    await expect(options).toHaveText(["cron", "webhooks", "retry queue"]);
    await options.last().click();
    await expect(input).toHaveValue('partition:"retry queue" budget');
    await page.keyboard.type("x");
    await expect(input).toHaveValue('partition:"retry queue" xbudget');
  });

  test("Escape and a press outside close the list, and typing brings it back", async ({ page, example }) => {
    const stage = await example("query-input/basic");
    const input = stage.getByRole("combobox");
    const list = page.getByRole("listbox");

    await input.click();
    await expect(list).toBeVisible();
    await page.keyboard.press("Escape");
    await expect(list).toBeHidden();
    await expect(input).toBeFocused();

    await page.keyboard.type("w");
    await expect(list).toBeVisible();
    await page.keyboard.press("Escape");
    await page.keyboard.press("ArrowDown");
    await expect(list.getByRole("option").first()).toHaveAttribute("aria-selected", "true");

    await page.mouse.click(5, 5);
    await expect(list).toBeHidden();
  });

  test("edits a draft: Enter applies it, Escape reverts it, and a problem is shown", async ({ page, example }) => {
    const stage = await example("query-input/expression");
    const input = stage.getByRole("combobox", { name: "Filter query" });
    const status = stage.getByRole("status");
    const options = page.getByRole("option");

    await input.click();
    await page.keyboard.press("End");
    await expect(page.getByRole("listbox")).toBeHidden();
    await page.keyboard.type(" && cou");
    await expect(stage.getByText("to apply")).toBeVisible();
    await expect(options).toHaveText([/^country/]);
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Enter");
    await expect(input).toHaveValue('browser == "Firefox" && country ');

    await page.keyboard.type("== ");
    await expect(options).toHaveText(["DE", "FR", "GB", "US", "ZA"]);
    await page.keyboard.type('"');
    await expect(page.getByRole("listbox")).toBeVisible();

    // An unfinished expression is refused, and says why.
    await page.keyboard.press("Enter");
    await expect(page.getByRole("alert")).toContainText("Unterminated string");
    await expect(input).toHaveAttribute("aria-invalid", "true");
    await expect(status).toHaveText('Applied: browser == "Firefox"');

    await page.keyboard.type("z");
    await expect(page.getByRole("alert")).toBeHidden();
    await expect(options).toHaveText(["ZA"]);
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Tab");
    await expect(input).toHaveValue('browser == "Firefox" && country == "ZA" ');
    await page.keyboard.press("Enter");
    await expect(status).toHaveText('Applied: browser == "Firefox" && country == "ZA" ');
    await expect(stage.getByText("to apply")).toBeHidden();

    // Escape closes the list first, then reverts the draft.
    await input.fill("boun");
    await page.keyboard.press("Escape");
    await expect(input).toHaveValue("boun");
    await page.keyboard.press("Escape");
    await expect(input).toHaveValue('browser == "Firefox" && country == "ZA" ');
  });

  test("takes its label, hint and error from a Field", async ({ example }) => {
    const stage = await example("query-input/field");
    const input = stage.getByRole("combobox", { name: "Only run for" });
    await expect(input).toHaveAttribute("aria-invalid", "true");
    await expect(input).toHaveAccessibleDescription(/Unsupported field: owner/);

    await input.fill('tag == "release"');
    await expect(input).not.toHaveAttribute("aria-invalid", "true");
    await expect(input).toHaveAccessibleDescription(/Available fields/);
  });

  test("shows focus on the whole box, and is inert while disabled", async ({ page, example }) => {
    const stage = await example("query-input/states");
    await page.keyboard.press("Tab");
    const input = stage.getByRole("combobox", { name: "Small" });
    await expect(input).toBeFocused();
    const box = input.locator("xpath=..");
    await expect(box).toHaveCSS("outline-style", "solid");
    await expect(box).not.toHaveCSS("outline-width", "0px");
    await expect(stage.getByRole("combobox", { name: "Disabled" })).toBeDisabled();
  });

  for (const theme of ["light", "dark"] as Theme[]) {
    test(`looks right when open: ${theme}`, async ({ page, example }) => {
      const basic = await example("query-input/basic", { theme });
      await basic.getByRole("combobox").click();
      await page.keyboard.press("ArrowDown");
      await page.keyboard.press("ArrowDown");
      await expect(page.getByRole("listbox")).toBeVisible();
      await expect(page).toHaveScreenshot(["query-input", `open-fields-${theme}.png`], {
        clip: await around(page, basic),
      });

      const expression = await example("query-input/expression", { theme });
      await expression.getByRole("combobox").fill('browser == "Firefox" && country == ');
      await page.keyboard.press("ArrowDown");
      await expect(page.getByRole("listbox")).toBeVisible();
      await expect(page).toHaveScreenshot(["query-input", `open-values-${theme}.png`], {
        clip: await around(page, expression, 200),
      });

      await expression.getByRole("combobox").fill('browser == "Firefox');
      await page.keyboard.press("Enter");
      await expect(page.getByRole("alert")).toBeVisible();
      await expect(page).toHaveScreenshot(["query-input", `open-error-${theme}.png`], {
        clip: await around(page, expression, 80),
      });
    });
  }
});
