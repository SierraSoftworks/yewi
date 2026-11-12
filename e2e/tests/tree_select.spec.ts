import type { Locator, Page } from "@playwright/test";
import { expect, test, type Theme } from "../fixtures";

/** The area around an example that its panel can reach. */
async function around(page: Page, stage: Locator) {
  const box = (await stage.boundingBox())!;
  const viewport = page.viewportSize()!;
  return { x: 0, y: 0, width: Math.min(box.x + box.width + 160, viewport.width), height: box.y + box.height + 300 };
}

test.describe("TreeSelect", () => {
  test("opens on the level holding the choice, and a press on an entry chooses it", async ({ page, example }) => {
    const stage = await example("tree-select/basic");
    const select = stage.getByRole("combobox", { name: "Data centre" });
    const list = page.getByRole("listbox", { name: "Data centre" });

    await expect(select).toHaveText("Dublin 1");
    await select.click();
    await expect(page.getByRole("combobox", { name: "Search" })).toBeFocused();
    await expect(list.getByRole("option")).toHaveText(["Dublin 1", "Dublin 2"]);
    await expect(list.getByRole("option", { selected: true })).toHaveText("Dublin 1");
    await expect(page.getByRole("navigation", { name: "Level" }).getByRole("button")).toHaveText([
      "All",
      "Europe",
      "Ireland",
    ]);

    await list.getByRole("option", { name: "Dublin 2" }).click();
    await expect(list).toBeHidden();
    await expect(select).toHaveText("Dublin 2");
    await expect(select).toBeFocused();
    await expect(stage.getByRole("status")).toHaveText("Deploying to dub-2");
  });

  test("walks the tree with the pointer: a branch opens, and the trail leads back up", async ({ page, example }) => {
    const stage = await example("tree-select/basic");
    const list = page.getByRole("listbox");

    await stage.getByRole("combobox", { name: "Data centre" }).click();
    await page.getByRole("navigation", { name: "Level" }).getByRole("button", { name: "All" }).click();
    await expect(list.getByRole("option")).toHaveText(["Europe", "Americas", "Asia Pacific"]);

    // A branch that is not an entry opens instead of being chosen.
    await list.getByRole("option", { name: "Americas" }).click();
    await list.getByRole("option", { name: "Brazil" }).click();
    await expect(list.getByRole("option")).toHaveText(["Sao Paulo"]);
    await expect(page.getByRole("combobox", { name: "Search" })).toBeFocused();
    await list.getByRole("option", { name: "Sao Paulo" }).click();
    await expect(stage.getByRole("status")).toHaveText("Deploying to gru-1");
  });

  test("works from the keyboard: arrows move and open branches, Enter chooses, Escape closes", async ({
    page,
    example,
  }) => {
    const stage = await example("tree-select/basic");
    const select = stage.getByRole("combobox", { name: "Data centre" });
    const search = page.getByRole("combobox", { name: "Search" });
    const list = page.getByRole("listbox");

    await select.press("ArrowDown");
    await expect(search).toBeFocused();
    // Left goes up a level at a time; right opens the highlighted branch.
    await page.keyboard.press("ArrowLeft");
    await expect(list.getByRole("option")).toHaveText(["Ireland", "Germany"]);
    await page.keyboard.press("ArrowLeft");
    await page.keyboard.press("End");
    await expect(search).toHaveAttribute("aria-activedescendant", /row-2$/);
    await page.keyboard.press("ArrowRight");
    await expect(list.getByRole("option")).toHaveText(["Japan", "Australia"]);
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Enter");
    await expect(list.getByRole("option")).toHaveText(["Sydney"]);

    await page.keyboard.press("Escape");
    await expect(list).toBeHidden();
    await expect(select).toBeFocused();
    await expect(stage.getByRole("status")).toHaveText("Deploying to dub-1");

    await select.press("Enter");
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Enter");
    await expect(stage.getByRole("status")).toHaveText("Deploying to dub-2");
  });

  test("closes on a second press, a press outside and Tab, without changing the choice", async ({ page, example }) => {
    const stage = await example("tree-select/basic");
    const select = stage.getByRole("combobox", { name: "Data centre" });
    const list = page.getByRole("listbox");

    await select.click();
    await expect(list).toBeVisible();
    await select.click();
    await expect(list).toBeHidden();

    await select.click();
    await page.mouse.click(5, 5);
    await expect(list).toBeHidden();

    await select.click();
    await expect(list).toBeVisible();
    // Tab closes the panel and lands back on the control.
    await page.keyboard.press("Tab");
    await expect(list).toBeHidden();
    await expect(select).toBeFocused();
    await expect(stage.getByRole("status")).toHaveText("Deploying to dub-1");
  });

  test("typing searches the whole tree by name or detail, and results say where they live", async ({
    page,
    example,
  }) => {
    const stage = await example("tree-select/search");
    const select = stage.getByRole("combobox", { name: "Account" });
    const search = page.getByRole("combobox", { name: "Search accounts" });
    const list = page.getByRole("listbox");

    await select.click();
    await search.fill("equip");
    await expect(list.getByRole("option")).toHaveText([/^EquipmentAssets › Fixed assets1510/, /^Office equipment/]);
    await search.fill("zzz");
    await expect(page.getByRole("status").filter({ hasText: "No matches" })).toBeVisible();
    await search.fill("5210");
    await page.keyboard.press("Enter");
    await expect(select).toContainText("Flights");
    await expect(stage.getByRole("status")).toHaveText("Posting to 5210");
  });

  test("an entry that holds others is chosen by a press, and opened by its arrow", async ({ page, example }) => {
    const stage = await example("tree-select/search");
    const select = stage.getByRole("combobox", { name: "Account" });
    const list = page.getByRole("listbox");

    await select.click();
    await page.getByRole("navigation", { name: "Level" }).getByRole("button", { name: "All" }).click();
    await list.getByRole("option", { name: "Assets" }).locator("[data-yi-enter]").click();
    await expect(list.getByRole("option")).toHaveText([/Current assets/, /Fixed assets/]);
    await list.getByRole("option", { name: "Fixed assets" }).click();
    await expect(list).toBeHidden();
    await expect(stage.getByRole("status")).toHaveText("Posting to 1500");
  });

  test("clears to nothing and accepts well-formed text the tree does not list", async ({ page, example }) => {
    const stage = await example("tree-select/custom");
    const select = stage.getByRole("combobox", { name: "Topic" });
    const search = page.getByRole("combobox", { name: "Search" });

    // The current value is not in the tree, and is shown as it stands.
    await expect(select).toHaveText("sensors/roof/wind");
    await select.click();
    await page.getByRole("option", { name: "Every topic" }).click();
    await expect(stage.getByRole("status")).toHaveText("Subscribed to everything");

    await select.click();
    await search.fill("Not A Topic");
    await expect(page.getByRole("option")).toHaveCount(0);
    await search.fill("sensors/attic/light");
    await page.getByRole("option", { name: "Use sensors/attic/light" }).click();
    await expect(stage.getByRole("status")).toHaveText("Subscribed to sensors/attic/light");
  });

  test("a large tree renders one level, or the first matches of a search", async ({ page, example }) => {
    const stage = await example("tree-select/large");
    const select = stage.getByRole("combobox", { name: "Rack" });

    await select.click();
    await expect(page.getByRole("option")).toHaveCount(5);
    await page.getByRole("combobox", { name: "Search" }).fill("rack 07");
    await expect(page.getByRole("option")).toHaveCount(40);
    await expect(page.getByText("Showing the first 40 of 120")).toBeVisible();
    await page.getByRole("combobox", { name: "Search" }).fill("us-west-3-h2-r07");
    await page.keyboard.press("Enter");
    await expect(stage.getByRole("status")).toHaveText("Rack us-west-3-h2-r07");
  });

  test("a disabled select does not open, and a named one submits its value", async ({ page, example }) => {
    const stage = await example("tree-select/states");
    const select = stage.getByRole("combobox", { name: "Disabled", exact: true });
    await expect(select).toBeDisabled();
    await select.click({ force: true });
    await expect(page.getByRole("listbox")).toBeHidden();
    await expect(stage.locator('input[name="country"]')).toHaveValue("br");
  });

  for (const theme of ["light", "dark"] as Theme[]) {
    test(`looks right when open: ${theme}`, async ({ page, example }) => {
      const basic = await example("tree-select/basic", { theme });
      await basic.getByRole("combobox").click();
      await expect(page.getByRole("listbox")).toBeVisible();
      await expect(page).toHaveScreenshot(["tree-select", `open-basic-${theme}.png`], {
        clip: await around(page, basic),
      });

      const search = await example("tree-select/search", { theme });
      await search.getByRole("combobox").click();
      await page.getByRole("combobox", { name: "Search accounts" }).fill("assets");
      await expect(page.getByRole("option")).toHaveCount(6);
      await expect(page).toHaveScreenshot(["tree-select", `open-search-${theme}.png`], {
        clip: await around(page, search),
      });

      const media = await example("tree-select/media", { theme });
      await media.getByRole("combobox").click();
      await page.keyboard.press("ArrowLeft");
      await expect(page.getByRole("option")).toHaveCount(2);
      await expect(page).toHaveScreenshot(["tree-select", `open-media-${theme}.png`], {
        clip: await around(page, media),
      });

      const custom = await example("tree-select/custom", { theme });
      await custom.getByRole("combobox").click();
      await expect(page.getByRole("option")).toHaveCount(3);
      await expect(page).toHaveScreenshot(["tree-select", `open-custom-${theme}.png`], {
        clip: await around(page, custom),
      });
    });
  }
});
