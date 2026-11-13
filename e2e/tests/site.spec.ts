// The reference site itself: navigation, search and example chrome.
import { expect, hydrated, test } from "../fixtures";

test.describe("Reference site", () => {
  test("navigates between pages without a full reload", async ({ page }) => {
    await page.goto("/");
    await hydrated(page);
    await page.evaluate(() => ((window as any).stillHere = true));

    await page.getByRole("navigation", { name: "Pages" }).getByRole("link", { name: "Button", exact: true }).click();
    await expect(page).toHaveURL(/\/components\/button$/);
    await expect(page.getByRole("heading", { level: 1 })).toHaveText("Button");
    expect(await page.evaluate(() => (window as any).stillHere)).toBe(true);
  });

  test("search finds a page by what it is for", async ({ page }) => {
    await page.goto("/");
    await hydrated(page);
    await page.getByRole("searchbox", { name: "Search the documentation" }).fill("accent");
    await page.getByRole("search").getByRole("link", { name: /Colour/ }).click();
    await expect(page.getByRole("heading", { level: 1 })).toHaveText("Colour");
  });

  test("an example shows its source and links to its isolated page", async ({ page }) => {
    await page.goto("/components/button");
    await hydrated(page);
    const example = page.locator("#variants");

    await expect(example.locator("pre")).toBeHidden();
    await example.getByRole("button", { name: "Show source" }).click();
    await expect(example.locator("pre")).toContainText("ButtonVariant::Primary");

    await example.getByRole("link", { name: "Open isolated" }).click();
    await expect(page).toHaveURL(/\/_examples\/button\/variants$/);
    await expect(page.locator("[data-example]")).toBeVisible();
  });

  test("documents props from the library's doc comments", async ({ page }) => {
    await page.goto("/components/button");
    const api = page.locator("#api");
    await expect(api.getByRole("row", { name: /^loading/ }).first()).toContainText("spinner");
    await expect(api.getByRole("row", { name: /^on_click/ }).first()).toContainText("Callback");
    await expect(api.getByRole("row", { name: /--yi-button-bg-hover/ })).toBeVisible();
  });

  test("the menu opens the page list on a narrow screen", async ({ page }) => {
    await page.setViewportSize({ width: 390, height: 844 });
    await page.goto("/components/button");
    await hydrated(page);
    const pages = page.getByRole("navigation", { name: "Pages" });

    await expect(pages).toBeHidden();
    await page.getByRole("button", { name: "Menu" }).click();
    await pages.getByRole("link", { name: "Icon" }).click();
    await expect(page.getByRole("heading", { level: 1 })).toHaveText("Icon");
    await expect(pages).toBeHidden();
  });

  test("unknown addresses get the not-found page", async ({ page }) => {
    const response = await page.goto("/components/does-not-exist");
    expect(response?.status()).toBe(404);
    await expect(page.getByRole("heading", { level: 1 })).toHaveText("Page not found");
  });

  test("pages only preload the scripts every page runs", async ({ page }) => {
    await page.goto("/components/button");
    const preloads = await page.locator('link[rel="modulepreload"]').evaluateAll((links) => links.map((link) => link.getAttribute("href")));
    expect(preloads.length).toBeGreaterThan(0);
    expect(preloads.filter((href) => href?.includes("/vendor/"))).toEqual([]);
  });

  test("API tables cover generic components", async ({ page }) => {
    await page.goto("/components/table");
    await expect(page.locator("#api").getByRole("row", { name: /^rows/ }).first()).toBeVisible();
  });
});

test.describe("Review gallery", () => {
  test("a link inside an example leaves the gallery in place", async ({ page, context }) => {
    await page.goto("/_gallery");
    await hydrated(page);
    await page.locator('[data-gallery-link="link"]').click();
    const heading = page.getByRole("heading", { level: 1 });
    await expect(heading).toHaveText("Link");

    const opened: unknown[] = [];
    context.on("page", (popup) => opened.push(popup));
    const address = page.url();
    await page.locator('[data-example="link/basic"]').getByRole("link", { name: "Documentation" }).click();
    await page.locator('[data-example="link/external"]').getByRole("link").first().click();

    await expect(heading).toHaveText("Link");
    expect(page.url()).toBe(address);
    expect(opened).toEqual([]);
  });

  test("offers each app's accent", async ({ page }) => {
    await page.goto("/_gallery");
    await hydrated(page);
    const accent = () => page.evaluate(() => getComputedStyle(document.documentElement).getPropertyValue("--yi-accent"));
    const before = await accent();

    await page.getByRole("combobox", { name: "Accent" }).click();
    for (const app of ["Symbols", "Automate", "Grey", "Analytics", "Buckle", "Rustak"]) {
      await expect(page.getByRole("option", { name: app, exact: true })).toBeVisible();
    }
    await page.getByRole("option", { name: "Buckle", exact: true }).click();
    await expect.poll(accent).not.toBe(before);
  });
});
