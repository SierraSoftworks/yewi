import { expect, test } from "../fixtures";

test.describe("Pagination", () => {
  test("moves with previous, next and the page numbers", async ({ example }) => {
    const stage = await example("pagination/basic");
    const status = stage.getByRole("status");
    const previous = stage.getByRole("button", { name: "Previous page" });

    await expect(previous).toBeDisabled();
    await expect(stage.getByRole("button", { name: "Page 1", exact: true })).toHaveAttribute("aria-current", "page");

    await stage.getByRole("button", { name: "Next page" }).click();
    await expect(status).toHaveText("Showing page 2 of 12");
    await expect(previous).toBeEnabled();

    await stage.getByRole("button", { name: "Page 12" }).click();
    await expect(status).toHaveText("Showing page 12 of 12");
    await expect(stage.getByRole("button", { name: "Next page" })).toBeDisabled();
    await expect(stage.getByRole("button", { name: "Page 12" })).toHaveAttribute("aria-current", "page");

    await previous.click();
    await expect(status).toHaveText("Showing page 11 of 12");
  });

  test("keeps its controls in place while paging", async ({ example }) => {
    const stage = await example("pagination/basic");
    const next = stage.getByRole("button", { name: "Next page" });
    const start = await next.boundingBox();

    for (let page = 2; page <= 7; page++) {
      await next.click();
      await expect(stage.getByRole("status")).toHaveText(`Showing page ${page} of 12`);
      expect((await next.boundingBox())!.x).toBe(start!.x);
    }
  });

  test("works from the keyboard with a visible focus indicator", async ({ page, example }) => {
    const stage = await example("pagination/basic");

    await page.keyboard.press("Tab");
    const first = stage.getByRole("button", { name: "Page 1", exact: true });
    await expect(first).toBeFocused();
    await expect(first).not.toHaveCSS("outline-style", "none");

    await page.keyboard.press("Tab");
    await page.keyboard.press("Enter");
    await expect(stage.getByRole("status")).toHaveText("Showing page 2 of 12");
  });

  test("compact shows a status, and an unknown total ends when there is no next page", async ({ example }) => {
    const stage = await example("pagination/compact");
    const results = stage.getByRole("navigation", { name: "Results" });
    const audit = stage.getByRole("navigation", { name: "Audit log" });

    await expect(results).toContainText("Page 3 of 12");
    await results.getByRole("button", { name: "Next page" }).click();
    await expect(results).toContainText("Page 4 of 12");

    await expect(audit).toContainText("Page 4");
    await expect(audit.getByRole("button", { name: "Next page" })).toBeDisabled();
    await expect(audit.getByRole("button", { name: "Previous page" })).toBeEnabled();
  });

  test("disabled holds every control", async ({ example }) => {
    const stage = await example("pagination/sizes");
    for (const button of await stage.getByRole("navigation", { name: "Loading" }).getByRole("button").all()) {
      await expect(button).toBeDisabled();
    }
  });

  test("as links, every control has a real destination", async ({ example }) => {
    const stage = await example("pagination/links");

    await expect(stage.getByRole("button")).toHaveCount(0);
    await expect(stage.getByRole("link", { name: "Page 9" })).toHaveAttribute("href", "?page=9");
    await expect(stage.getByRole("link", { name: "Next page" })).toHaveAttribute("href", "?page=2");
    const previous = stage.getByRole("link", { name: "Previous page" });
    await expect(previous).toHaveAttribute("aria-disabled", "true");
    await expect(previous).not.toHaveAttribute("href");
  });

  test("with a router, pages through the query without leaving the page", async ({ page, example }) => {
    const stage = await example("pagination/router");
    const url = page.url();
    const status = stage.getByRole("status");

    await stage.getByRole("link", { name: "Page 4", exact: true }).click();
    await expect(status).toHaveText("Route: /events?page=4");
    await expect(stage.getByRole("link", { name: "Page 4", exact: true })).toHaveAttribute("aria-current", "page");

    await stage.getByRole("link", { name: "Next page" }).click();
    await expect(status).toHaveText("Route: /events?page=5");
    expect(page.url()).toBe(url);
  });
});
