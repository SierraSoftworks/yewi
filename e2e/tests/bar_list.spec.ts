import { expect, test } from "../fixtures";

test.describe("BarList", () => {
  test("filters by a row with the mouse and clears it again", async ({ example }) => {
    const stage = await example("bar-list/filter");
    const status = stage.getByRole("status");
    const row = (name: string) => stage.getByRole("button", { name: new RegExp(`^${name}`) });

    await expect(status).toHaveText("Showing events for docs");
    await expect(row("Documentation")).toHaveAttribute("aria-pressed", "true");
    await expect(row("Website")).toHaveAttribute("aria-pressed", "false");

    await row("Website").click();
    await expect(row("Website")).toHaveAttribute("aria-pressed", "true");
    await expect(row("Documentation")).toHaveAttribute("aria-pressed", "false");

    await row("Website").click();
    await expect(status).toHaveText("Showing all events");
    await expect(row("Website")).toHaveAttribute("aria-pressed", "false");
  });

  test("filters and runs the row action from the keyboard", async ({ page, example }) => {
    const stage = await example("bar-list/filter");
    const status = stage.getByRole("status");

    await page.keyboard.press("Tab");
    const first = stage.getByRole("button", { name: /^Website/ });
    await expect(first).toBeFocused();
    await expect(first).not.toHaveCSS("outline-style", "none");
    await page.keyboard.press("Enter");
    await expect(status).toHaveText("Showing events for website");
    await page.keyboard.press("Space");
    await expect(status).toHaveText("Showing all events");
    await expect(first).toBeFocused();

    await page.keyboard.press("Tab");
    const manage = stage.getByRole("button", { name: "Manage: Website" });
    await expect(manage).toBeFocused();
    await expect(manage).toHaveCSS("opacity", "1");
    await page.keyboard.press("Enter");
    await expect(status).toHaveText("Managing website");
  });

  test("reveals the row action on hover and keeps it off rows for missing values", async ({ example }) => {
    const stage = await example("bar-list/filter");
    const manage = stage.getByRole("button", { name: "Manage: Documentation" });

    await expect(manage).toHaveCSS("opacity", "0");
    await stage.getByRole("button", { name: /^Documentation/ }).hover();
    await expect(manage).toHaveCSS("opacity", "1");
    await manage.click();
    await expect(stage.getByRole("status")).toHaveText("Managing docs");
    await expect(stage.getByRole("button", { name: /^Manage:/ })).toHaveCount(3);
  });

  test("expands and collapses a long list", async ({ example }) => {
    const stage = await example("bar-list/overflow");
    const list = stage.getByRole("list", { name: "Pages by views" });
    const toggle = stage.getByRole("button", { name: "Show all 24" });

    await expect(list.getByRole("listitem")).toHaveCount(5);
    await toggle.click();
    await expect(list.getByRole("listitem")).toHaveCount(24);

    const collapse = stage.getByRole("button", { name: "Show fewer" });
    await expect(collapse).toHaveAttribute("aria-expanded", "true");
    await expect(collapse).toBeFocused();
    await collapse.click();
    await expect(list.getByRole("listitem")).toHaveCount(5);
  });

  test("pages through a long list", async ({ example }) => {
    const stage = await example("bar-list/overflow");
    const list = stage.getByRole("list", { name: "Pages by visitors" });
    const pages = stage.getByRole("navigation", { name: "Pages by visitors pages" });

    await expect(list.getByRole("listitem")).toHaveCount(5);
    await expect(list.getByRole("listitem").first()).toContainText("page-01");
    await expect(pages).toContainText("Page 1 of 5");

    await pages.getByRole("button", { name: /next/i }).click();
    await expect(pages).toContainText("Page 2 of 5");
    await expect(list.getByRole("listitem").first()).toContainText("page-06");

    for (let i = 0; i < 3; i++) await pages.getByRole("button", { name: /next/i }).click();
    await expect(list.getByRole("listitem")).toHaveCount(4);
    await expect(pages.getByRole("button", { name: /next/i })).toBeDisabled();
  });

  test("draws each bar in proportion to the largest row", async ({ example }) => {
    const stage = await example("bar-list/basic");
    const rows = stage.getByRole("list", { name: "Top pages" }).getByRole("listitem");
    const bar = (index: number) =>
      rows.nth(index).evaluate((item) => {
        const row = item.firstElementChild!;
        return parseFloat(getComputedStyle(row, "::before").width) / row.getBoundingClientRect().width;
      });

    expect(await bar(0)).toBeCloseTo(1, 1);
    expect(await bar(1)).toBeCloseTo(2140 / 4812, 1);
    await expect(rows.first()).toContainText("46%");
  });

  test("switches between breakdowns with tabs", async ({ example }) => {
    const stage = await example("bar-list/tabbed");

    await expect(stage.getByRole("list", { name: "Browsers" })).toBeVisible();
    await stage.getByRole("tab", { name: "Countries" }).click();
    await expect(stage.getByRole("list", { name: "Countries" })).toContainText("Germany");
    await expect(stage.getByRole("list", { name: "Browsers" })).toHaveCount(0);
  });

  test("rows with href are real links", async ({ example }) => {
    const stage = await example("bar-list/tones");
    await expect(stage.getByRole("link", { name: /search/ })).toHaveAttribute("href", "#search");
  });
});
