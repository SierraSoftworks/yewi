import type { Locator } from "@playwright/test";
import { expect, test } from "../fixtures";

/** The text of the first cell of every body row, in display order. */
const firstColumn = (table: Locator) => table.locator("tbody tr > :first-child").allTextContents();

test.describe("Table", () => {
  test("sorts by a column on click and reverses on the next", async ({ example }) => {
    const stage = await example("table/sorting");
    const table = stage.getByRole("table", { name: "Hosts" });
    const header = (name: string) => table.getByRole("columnheader", { name });

    // Text sorts naturally: case-insensitive, digit runs by value.
    await expect(header("Host")).toHaveAttribute("aria-sort", "ascending");
    expect(await firstColumn(table)).toEqual(["db-1", "Edge-ams", "web-1", "web-2", "web-10"]);

    await header("Host").getByRole("button").click();
    await expect(header("Host")).toHaveAttribute("aria-sort", "descending");
    expect(await firstColumn(table)).toEqual(["web-10", "web-2", "web-1", "Edge-ams", "db-1"]);

    // A status column sorts in its declared order, not alphabetically.
    await header("State").getByRole("button").click();
    await expect(header("State")).toHaveAttribute("aria-sort", "ascending");
    await expect(header("Host")).not.toHaveAttribute("aria-sort");
    expect((await firstColumn(table)).slice(0, 2)).toEqual(["Edge-ams", "web-2"]);
    await expect(stage.getByRole("status")).toHaveText("Sorted by state, ascending");
  });

  test("sort headers work from the keyboard and show focus", async ({ page, example }) => {
    const stage = await example("table/sorting");
    const table = stage.getByRole("table", { name: "Hosts" });
    const memory = table.getByRole("columnheader", { name: "Memory" });

    await memory.getByRole("button").focus();
    await expect(memory.getByRole("button")).not.toHaveCSS("outline-style", "none");
    await page.keyboard.press("Enter");
    await expect(memory).toHaveAttribute("aria-sort", "ascending");
    await page.keyboard.press("Space");
    await expect(memory).toHaveAttribute("aria-sort", "descending");
    await expect(memory.getByRole("button")).toBeFocused();
    expect((await firstColumn(table))[0]).toBe("db-1");
  });

  test("server-side sorting reports the request and leaves the rows to the caller", async ({ example }) => {
    const stage = await example("table/server-sort");
    const table = stage.getByRole("table", { name: "Pages" });

    await expect(table.getByRole("columnheader", { name: "Views" })).toHaveAttribute("aria-sort", "descending");
    await table.getByRole("columnheader", { name: "Visitors" }).getByRole("button").click();
    await expect(stage.getByRole("status")).toHaveText("Requested /pages?sort=visitors");
    await expect(table.getByRole("columnheader", { name: "Visitors" })).toHaveAttribute("aria-sort", "ascending");
    expect((await firstColumn(table))[0]).toBe("/pricing");
  });

  test("Shift-click adds a tie-break column", async ({ example }) => {
    const stage = await example("table/dense");
    const table = stage.getByRole("table", { name: "Recent requests" });
    const header = (name: string) => table.getByRole("columnheader", { name });

    await header("Status").getByRole("button").click();
    await header("Time").getByRole("button").click({ modifiers: ["Shift"] });
    await expect(header("Status")).toHaveAttribute("aria-sort", "ascending");

    const rows = await table.locator("tbody tr").evaluateAll((rows) =>
      rows.slice(0, 40).map((row) => [Number(row.children[2].textContent), parseInt(row.children[3].textContent!)]),
    );
    for (let i = 1; i < rows.length; i++) {
      const [status, time] = rows[i];
      const [prevStatus, prevTime] = rows[i - 1];
      expect(status).toBeGreaterThanOrEqual(prevStatus);
      if (status === prevStatus) expect(time).toBeGreaterThanOrEqual(prevTime);
    }
  });

  test("the header stays in view while a tall table scrolls", async ({ example }) => {
    const stage = await example("table/dense");
    const table = stage.getByRole("table", { name: "Recent requests" });
    const header = table.getByRole("columnheader", { name: "Route" });

    const before = (await header.boundingBox())!;
    await table.locator("tbody tr").nth(200).scrollIntoViewIfNeeded();
    const after = (await header.boundingBox())!;
    expect(after.y).toBeCloseTo(before.y, 0);
    await expect(header).toBeInViewport();
  });

  test("selection follows rows through sorting, and the header box selects all", async ({ page, example }) => {
    const stage = await example("table/selection");
    const table = stage.getByRole("table", { name: "Hosts" });
    const status = stage.getByRole("status");
    const all = table.getByRole("checkbox", { name: "Select all rows" });

    await table.getByRole("checkbox", { name: "Select db-01" }).check();
    await expect(status).toHaveText("1 of 4 hosts selected");
    await expect(all).toHaveJSProperty("indeterminate", true);

    await table.getByRole("columnheader", { name: "Host" }).getByRole("button").click();
    await expect(table.getByRole("checkbox", { name: "Select db-01" })).toBeChecked();
    await expect(status).toHaveText("1 of 4 hosts selected");

    await all.focus();
    await page.keyboard.press("Space");
    await expect(status).toHaveText("4 of 4 hosts selected");
    await expect(all).toBeChecked();

    await stage.getByRole("button", { name: "Clear selection" }).click();
    await expect(status).toHaveText("0 of 4 hosts selected");
    await expect(table.getByRole("checkbox", { checked: true })).toHaveCount(0);
  });

  test("a row's details open and close from its disclosure button", async ({ page, example }) => {
    const stage = await example("table/expandable");
    const table = stage.getByRole("table", { name: "Task runs" });
    const toggle = table.getByRole("button", { name: "Details for Back up database" });

    await expect(toggle).toHaveAttribute("aria-expanded", "false");
    await expect(table.getByText("Upload timed out after 240 s")).toHaveCount(0);

    await toggle.focus();
    await page.keyboard.press("Enter");
    await expect(toggle).toHaveAttribute("aria-expanded", "true");
    await expect(table.getByText("Upload timed out after 240 s")).toBeVisible();
    await expect(toggle).toBeFocused();
    await expect(table).toHaveScreenshot(["table", "expandable-open.png"]);

    await toggle.click();
    await expect(table.getByText("Upload timed out after 240 s")).toHaveCount(0);
  });

  test("a row click opens the row, but not from a link inside it", async ({ page, example }) => {
    const stage = await example("table/row-click");
    // Keep the link from navigating away; the click still reaches the row.
    await page.evaluate(() => document.addEventListener("click", (event) => event.preventDefault(), true));
    const table = stage.getByRole("table", { name: "Workloads" });
    const status = stage.getByRole("status");

    await table.getByRole("link", { name: "caddy" }).click();
    await expect(status).toHaveText("Nothing opened");

    await table.getByRole("cell", { name: "us-east-2" }).click();
    await expect(status).toHaveText("Opened grafana");
  });

  test("moves through loading, loaded, refreshing and empty", async ({ example }) => {
    const stage = await example("table/states");
    const table = stage.getByRole("table", { name: "Open incidents" });
    const show = (name: string) => stage.getByRole("radio", { name }).check({ force: true });

    await expect(table).toHaveAttribute("aria-busy", "true");
    await expect(table.getByRole("rowheader")).toHaveCount(0);

    await show("Loaded");
    await expect(table).not.toHaveAttribute("aria-busy", "true");
    await expect(table.getByRole("rowheader")).toHaveCount(2);

    await show("Refreshing");
    await expect(table).toHaveAttribute("aria-busy", "true");
    await expect(table.getByRole("rowheader", { name: "Elevated 502s on edge-ams" })).toBeVisible();

    await show("Empty");
    await expect(table.getByText("No open incidents")).toBeVisible();
    await expect(table).not.toHaveAttribute("aria-busy", "true");
  });

  test("a hand-written table sorts through SortHeader", async ({ example }) => {
    const stage = await example("table/simple");
    const table = stage.getByRole("table", { name: "Rollout by stage" });
    const host = table.getByRole("columnheader", { name: "Host" });

    await expect(host).toHaveAttribute("aria-sort", "ascending");
    await host.getByRole("button").click();
    await expect(host).toHaveAttribute("aria-sort", "descending");
    expect((await firstColumn(table)).slice(0, 2)).toEqual(["Canary", "web-02"]);
  });

  test("in a flush card the first column starts under the card title", async ({ example }) => {
    const stage = await example("table/in-card");
    const left = (locator: Locator) => locator.evaluate((el) => {
      const range = document.createRange();
      range.selectNodeContents(el);
      return Math.round(range.getBoundingClientRect().left);
    });

    const title = await left(stage.getByRole("heading", { name: "Releases" }));
    expect(await left(stage.getByRole("columnheader", { name: "Version" }))).toBe(title);
    expect(await left(stage.getByRole("rowheader", { name: "2026.1.3" }))).toBe(title);
  });

  test("a table too wide for its container scrolls and is reachable by keyboard", async ({ page, example }) => {
    await page.setViewportSize({ width: 360, height: 640 });
    const stage = await example("table/basic");
    const scroller = stage.locator(".yi-table__scroll");

    expect(await scroller.evaluate((el) => el.scrollWidth > el.clientWidth)).toBe(true);
    await page.keyboard.press("Tab");
    await expect(scroller).toBeFocused();
    await expect(scroller).not.toHaveCSS("outline-style", "none");
    await page.keyboard.press("ArrowRight");
    await expect.poll(() => scroller.evaluate((el) => el.scrollLeft)).toBeGreaterThan(0);
  });
});
