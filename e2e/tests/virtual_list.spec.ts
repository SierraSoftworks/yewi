import type { Locator } from "@playwright/test";
import { expect, test } from "../fixtures";

/** Scrolls the element that scrolls `inside` to a fraction of its range and waits for the rows to follow. */
const scrollTo = async (scroller: Locator, fraction: number) => {
  await scroller.evaluate((element, fraction) => {
    element.scrollTop = (element.scrollHeight - element.clientHeight) * fraction;
  }, fraction);
};

test.describe("VirtualTable", () => {
  test("keeps a few dozen of 100,000 rows in the document wherever it is scrolled", async ({ example }) => {
    const stage = await example("virtual-list/table");
    const table = stage.getByRole("table", { name: "Requests" });
    const scroller = stage.locator(".yi-table__scroll");
    const rows = table.locator("tbody tr:not([aria-hidden])");

    await expect(table).toHaveAttribute("aria-rowcount", "100001");
    await expect(table.getByRole("rowheader", { name: "4800000" })).toBeVisible();
    expect(await rows.count()).toBeLessThan(60);

    await scrollTo(scroller, 0.5);
    await expect(table.getByRole("rowheader", { name: "4850000" })).toBeVisible();
    expect(await rows.count()).toBeLessThan(60);

    // The very last row is reachable and sits at the bottom edge.
    await scrollTo(scroller, 1);
    const last = table.getByRole("row", { name: /4899999/ });
    await expect(last).toBeInViewport({ ratio: 1 });
    await expect(last).toHaveAttribute("aria-rowindex", "100001");
    await expect(table.getByRole("columnheader", { name: "Route" })).toBeInViewport();
  });

  test("sorts all the rows, not just the ones drawn", async ({ page, example }) => {
    const stage = await example("virtual-list/table");
    const table = stage.getByRole("table", { name: "Requests" });
    const time = table.getByRole("columnheader", { name: "Time" });

    await time.getByRole("button").focus();
    await page.keyboard.press("Enter");
    await page.keyboard.press("Enter");
    await expect(time).toHaveAttribute("aria-sort", "descending");
    await expect(stage.getByRole("status")).toHaveText("Sorted by time, descending");
    // The slowest request is far down the unsorted data.
    await expect(table.locator("tbody tr:not([aria-hidden])").first()).toContainText("2413 ms");
  });

  test("selection survives rows leaving the document", async ({ page, example }) => {
    const stage = await example("virtual-list/selection");
    const table = stage.getByRole("table", { name: "Hosts" });
    const scroller = stage.locator(".yi-table__scroll");
    const status = stage.getByRole("status");

    await table.getByRole("checkbox", { name: "Select node-2", exact: true }).check();
    await expect(status).toHaveText("1 of 20000 hosts selected");

    await scrollTo(scroller, 1);
    await expect(table.getByRole("rowheader", { name: "node-20000" })).toBeVisible();
    await expect(table.getByRole("checkbox", { name: "Select node-2", exact: true })).toHaveCount(0);
    await table.getByRole("rowheader", { name: "node-20000" }).click();
    await expect(status).toHaveText("1 of 20000 hosts selected, opened node-20000");

    await scrollTo(scroller, 0);
    await expect(table.getByRole("checkbox", { name: "Select node-2", exact: true })).toBeChecked();

    const all = table.getByRole("checkbox", { name: "Select all rows" });
    await expect(all).toHaveJSProperty("indeterminate", true);
    await all.focus();
    await page.keyboard.press("Space");
    await expect(status).toHaveText("20000 of 20000 hosts selected, opened node-20000");
  });
});

test.describe("VirtualList", () => {
  test("draws only the rows near the view and names each row's place", async ({ example }) => {
    const stage = await example("virtual-list/list");
    const list = stage.getByRole("list", { name: "Log entries" });
    const items = list.getByRole("listitem");

    await expect(items.first()).toContainText("line 1");
    expect(await items.count()).toBeLessThan(60);

    await scrollTo(list, 1);
    const last = items.filter({ hasText: "line 10000" });
    await expect(last).toBeInViewport({ ratio: 1 });
    await expect(last).toHaveAttribute("aria-posinset", "10000");
    await expect(last).toHaveAttribute("aria-setsize", "10000");
    expect(await items.count()).toBeLessThan(60);
  });

  test("scrolls from the keyboard and shows focus", async ({ page, example }) => {
    const stage = await example("virtual-list/list");
    const list = stage.getByRole("list", { name: "Log entries" });

    await list.focus();
    await expect(list).not.toHaveCSS("outline-style", "none");
    await page.keyboard.press("End");
    await expect(list.getByRole("listitem").filter({ hasText: "line 10000" })).toBeInViewport();
    await page.keyboard.press("Home");
    await expect(list.getByRole("listitem").filter({ hasText: /line 1$/ })).toBeInViewport();
  });

  test("loads the next page when the end comes into range", async ({ example }) => {
    const stage = await example("virtual-list/load-more");
    const list = stage.getByRole("list", { name: "Commits" });
    const status = stage.getByRole("status");

    await expect(status).toHaveText("Loaded 100 of 1000 commits");
    await scrollTo(list, 1);
    await expect(status).toHaveText("Loaded 200 of 1000 commits");
    // The row that was at the bottom has not moved.
    await expect(list.getByRole("listitem").filter({ hasText: "#901" })).toBeInViewport();
  });
});
