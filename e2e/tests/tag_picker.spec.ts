import type { Locator, Page } from "@playwright/test";
import { expect, test, type Theme } from "../fixtures";

/** The area around an example that its panel can reach. */
async function around(page: Page, stage: Locator) {
  const box = (await stage.boundingBox())!;
  const viewport = page.viewportSize()!;
  return { x: 0, y: 0, width: Math.min(box.x + box.width + 160, viewport.width), height: box.y + box.height + 300 };
}

test.describe("TagPicker", () => {
  test("a press opens the list with the choices check-marked, and a press on an option adds or removes it", async ({
    page,
    example,
  }) => {
    const stage = await example("tag-picker/basic");
    const input = stage.getByRole("combobox", { name: "Labels" });
    const list = page.getByRole("listbox", { name: "Labels" });
    const tags = stage.getByRole("listitem");

    await expect(tags).toHaveText(["Bug", "Interface"]);
    await input.click();
    await expect(list).toBeVisible();
    await expect(list.getByRole("option", { selected: true })).toHaveCount(2);

    // The list stays open, so several can be picked in a row.
    await list.getByRole("option", { name: "Feature" }).click();
    await list.getByRole("option", { name: "Bug" }).click();
    await expect(list).toBeVisible();
    await expect(input).toBeFocused();
    await expect(tags).toHaveText(["Interface", "Feature"]);
    await expect(stage.getByRole("status")).toHaveText("Labels: ui, feature");

    // A disabled option cannot be chosen.
    await list.getByRole("option", { name: "Security" }).click({ force: true });
    await expect(tags).toHaveCount(2);

    await page.mouse.click(5, 5);
    await expect(list).toBeHidden();
  });

  test("works from the keyboard: typing narrows, Enter toggles, Backspace removes the last tag, Escape closes", async ({
    page,
    example,
  }) => {
    const stage = await example("tag-picker/basic");
    const input = stage.getByRole("combobox", { name: "Labels" });
    const list = page.getByRole("listbox");

    await input.fill("doc");
    await expect(list.getByRole("option")).toHaveText([/Documentation/]);
    await page.keyboard.press("Enter");
    await expect(input).toHaveValue("");
    await expect(stage.getByRole("status")).toHaveText("Labels: bug, ui, docs");

    await page.keyboard.press("ArrowDown");
    await expect(input).toHaveAttribute("aria-activedescendant", /option-\d+$/);
    await page.keyboard.press("Escape");
    await expect(list).toBeHidden();
    await expect(input).toBeFocused();

    await page.keyboard.press("Backspace");
    await page.keyboard.press("Backspace");
    await expect(stage.getByRole("status")).toHaveText("Labels: bug");

    await input.fill("zzz");
    await expect(page.getByRole("status").filter({ hasText: "No matches" })).toBeVisible();
    await page.keyboard.press("Tab");
    await expect(input).toHaveValue("");
    await expect(stage.getByRole("status")).toHaveText("Labels: bug");
  });

  test("a tag's remove button removes it and hands focus to the input", async ({ example }) => {
    const stage = await example("tag-picker/basic");
    await stage.getByRole("button", { name: "Remove Bug" }).click();
    await expect(stage.getByRole("listitem")).toHaveText(["Interface"]);
    await expect(stage.getByRole("combobox", { name: "Labels" })).toBeFocused();
  });

  test("free-form tags are added by Enter, a comma, a paste and leaving the field, without duplicates", async ({
    page,
    example,
  }) => {
    const stage = await example("tag-picker/free");
    const input = stage.getByRole("combobox", { name: "Allowed hosts" });
    const tags = stage.getByRole("listitem");

    await input.fill("cdn.example.net");
    await expect(page.getByText("Press Enter to add cdn.example.net")).toBeVisible();
    await page.keyboard.press("Enter");
    await input.pressSequentially("db.example.net,");
    await expect(input).toHaveValue("");
    await input.fill("a.example.net, b.example.net,api.example.net");
    await expect(tags).toHaveText(["api.example.net", "cdn.example.net", "db.example.net", "a.example.net", "b.example.net"]);

    await input.fill("api.example.net");
    await expect(page.getByRole("status").filter({ hasText: "already added" })).toBeVisible();
    await page.keyboard.press("Enter");
    await expect(tags).toHaveCount(5);

    await input.fill("last.example.net");
    await page.keyboard.press("Tab");
    await expect(tags).toHaveCount(6);
    await expect(stage.getByRole("status")).toContainText("6 allowed");
  });

  test("suggestions can be picked or typed past", async ({ page, example }) => {
    const stage = await example("tag-picker/suggestions");
    const input = stage.getByRole("combobox", { name: "Topics" });

    // Enter means the text until an option is highlighted.
    await input.fill("WASM");
    await page.keyboard.press("Enter");
    await input.fill("hydration");
    await page.keyboard.press("Enter");
    await input.fill("c");
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Enter");
    await expect(stage.getByRole("status")).toHaveText("Topics: rust, wasm, hydration, css");
  });

  test("a long list renders only the first matches", async ({ page, example }) => {
    const stage = await example("tag-picker/large");
    const input = stage.getByRole("combobox", { name: "Drain hosts" });

    await input.click();
    await expect(page.getByRole("option")).toHaveCount(50);
    await expect(page.getByText("Showing the first 50 of 5000")).toBeVisible();
    await input.fill("web-4999");
    await page.keyboard.press("Enter");
    await expect(stage.getByRole("status")).toHaveText("1 hosts chosen");
  });

  test("a disabled picker cannot be opened or changed", async ({ page, example }) => {
    const stage = await example("tag-picker/states");
    const input = stage.getByRole("combobox", { name: "Disabled" });
    await expect(input).toBeDisabled();
    await expect(stage.getByRole("button", { name: "Remove Monday" }).nth(2)).toBeDisabled();
    await input.click({ force: true });
    await expect(page.getByRole("listbox")).toBeHidden();
  });

  test("submits each value in a native form", async ({ page, example }) => {
    const stage = await example("tag-picker/form");
    await stage.getByRole("combobox", { name: "Token scopes" }).click();
    await page.getByRole("option", { name: "Administer" }).click();
    await expect(stage.locator('input[name="scope"]')).toHaveCount(2);
    await page.keyboard.press("Escape");
    await stage.getByRole("button", { name: "Create token" }).click();
    await expect(stage.getByRole("status")).toHaveText("Saved scope=read&scope=admin");
  });

  for (const theme of ["light", "dark"] as Theme[]) {
    test(`looks right when open: ${theme}`, async ({ page, example }) => {
      const basic = await example("tag-picker/basic", { theme });
      await basic.getByRole("combobox").click();
      await page.keyboard.press("ArrowDown");
      await expect(page.getByRole("listbox")).toBeVisible();
      await expect(page).toHaveScreenshot(["tag-picker", `open-basic-${theme}.png`], {
        clip: await around(page, basic),
      });

      const suggestions = await example("tag-picker/suggestions", { theme });
      await suggestions.getByRole("combobox").fill("a");
      await expect(page.getByRole("listbox")).toBeVisible();
      await expect(page).toHaveScreenshot(["tag-picker", `open-suggestions-${theme}.png`], {
        clip: await around(page, suggestions),
      });

      const free = await example("tag-picker/free", { theme });
      await free.getByRole("combobox").fill("cdn.example.net");
      await expect(page.getByText("Press Enter to add")).toBeVisible();
      await expect(page).toHaveScreenshot(["tag-picker", `open-free-${theme}.png`], { clip: await around(page, free) });
    });
  }
});
