import { expect, test } from "../fixtures";

test.describe("JsonView", () => {
  test("opens and folds a node with the mouse, creating its children on demand", async ({ example }) => {
    const stage = await example("json-view/basic");
    const os = stage.locator("summary").filter({ hasText: "os" });

    // Folded: a one-line hint, and nothing rendered beneath it.
    await expect(os).toContainText('family: "debian"');
    await expect(stage.getByText("kernel", { exact: true })).toHaveCount(0);

    await os.click();
    await expect(stage.getByText('"6.12.48-amd64"', { exact: true })).toBeVisible();
    await expect(os.locator("..")).toHaveJSProperty("open", true);

    await os.click();
    await expect(stage.getByText('"6.12.48-amd64"', { exact: true })).toHaveCount(0);
    await expect(os.locator("..")).toHaveJSProperty("open", false);
  });

  test("is operable from the keyboard", async ({ page, example }) => {
    const stage = await example("json-view/basic");
    const root = stage.locator("summary").first();
    const os = stage.locator("summary").filter({ hasText: "os" });

    await page.keyboard.press("Tab");
    await expect(stage.getByRole("button", { name: "Copy JSON" })).toBeFocused();
    await page.keyboard.press("Tab");
    await expect(root).toBeFocused();
    await expect(root).not.toHaveCSS("outline-style", "none");

    await page.keyboard.press("Tab");
    await expect(os).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(stage.getByText('"debian"', { exact: true })).toBeVisible();
    await page.keyboard.press("Space");
    await expect(stage.getByText('"6.12.48-amd64"', { exact: true })).toHaveCount(0);
    await expect(os).toBeFocused();

    // Folding the root takes the whole document off the page.
    await page.keyboard.press("Shift+Tab");
    await page.keyboard.press("Enter");
    await expect(stage.locator("summary")).toHaveCount(1);
  });

  test("copies the whole document, including what is folded", async ({ context, page, example }) => {
    await context.grantPermissions(["clipboard-read", "clipboard-write"]);
    const stage = await example("json-view/basic");

    await stage.getByRole("button", { name: "Copy JSON" }).click();
    await expect(page.locator("[data-yi-announcer]")).toHaveText("Copied");

    const copied = await page.evaluate(() => navigator.clipboard.readText());
    expect(copied).toContain('\n  "hostname": "edge-fra-02",');
    expect(JSON.parse(copied).interfaces[1]).toEqual({ name: "wg0", addresses: ["10.44.0.7"], mtu: 1420 });
  });

  test("the source variant is the document, character for character", async ({ example }) => {
    const stage = await example("json-view/source");
    const text = await stage.locator("pre").innerText();

    expect(text).toContain('\n  "attempt": 2,');
    expect(JSON.parse(text)).toMatchObject({ workflow: "deploy-caddy", approved_by: null, targets: ["edge-fra-01", "edge-fra-02"] });
  });

  test("a large document is rendered a page at a time", async ({ page, example }) => {
    const stage = await example("json-view/large");
    const packages = stage.locator(".yi-json-view__children .yi-json-view__children > details");

    // 5,000 packages in the document, 50 on the page.
    await expect(stage.getByText("[5000]")).toBeVisible();
    await expect(packages).toHaveCount(50);
    await expect(stage.getByText("4950 hidden")).toBeVisible();
    expect(await stage.locator("*").count()).toBeLessThan(1000);

    // Showing more appends: what was open stays open, where it was.
    await packages.nth(10).locator("summary").click();
    await stage.getByRole("button", { name: "Show 50 more" }).click();
    await expect(packages).toHaveCount(100);
    await expect(stage.getByText("4900 hidden")).toBeVisible();
    await expect(packages.nth(10)).toHaveJSProperty("open", true);
    await expect(packages.nth(10).getByText('"pkg-0010"', { exact: true })).toBeVisible();
    await expect(stage.locator(".yi-json-view__children .yi-json-view__children > details[open]")).toHaveCount(1);

    // The capped view scrolls, and can be scrolled from the keyboard.
    const region = stage.getByRole("region", { name: "inventory" });
    await region.focus();
    await page.keyboard.press("End");
    await expect.poll(() => region.evaluate((el) => el.scrollTop)).toBeGreaterThan(0);
  });

  test("invalid text is reported instead of shown", async ({ example }) => {
    const stage = await example("json-view/parse");
    await expect(stage.locator(".yi-json-view")).toHaveCount(1);
    await expect(stage.getByText("The response is not valid JSON")).toBeVisible();
    await expect(stage.getByText(/at byte \d+/)).toBeVisible();
  });
});
