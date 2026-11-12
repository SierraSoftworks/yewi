import type { Page } from "@playwright/test";
import { expect, test } from "../fixtures";

const SHA = "4f9a2c1e8b7d6a5f4e3d2c1b0a9f8e7d6c5b4a39";

/** What is on the clipboard, read through a handle that survives the stubs below. */
const clipboardText = (page: Page) => page.evaluate(() => (window as any).__clipboard.readText());

test.describe("Copyable", () => {
  test.beforeEach(async ({ context, page }) => {
    await context.grantPermissions(["clipboard-read", "clipboard-write"]);
    await page.addInitScript(() => ((window as any).__clipboard = navigator.clipboard));
  });

  test("copies the full value with the mouse and confirms it", async ({ page, example }) => {
    const stage = await example("copyable/basic");
    const copy = stage.getByRole("button", { name: "Copy" }).first();

    await expect(copy).toHaveAccessibleDescription("4f9a2c1");
    await copy.click();

    await expect(stage.getByRole("status")).toHaveText(`Copied ${SHA}`);
    await expect(copy).toHaveAttribute("data-yi-state", "copied");
    await expect(page.locator("[data-yi-announcer]")).toHaveText("Copied");
    expect(await clipboardText(page)).toBe(SHA);
  });

  test("copies with the keyboard and returns to idle", async ({ page, example }) => {
    const stage = await example("copyable/basic");
    const copy = stage.getByRole("button", { name: "Copy" }).nth(1);

    await page.keyboard.press("Tab");
    await page.keyboard.press("Tab");
    await expect(copy).toBeFocused();
    await expect(copy).not.toHaveCSS("outline-style", "none");
    await page.keyboard.press("Enter");

    await expect(stage.getByRole("status")).toHaveText("Copied edge-fra-02");
    expect(await clipboardText(page)).toBe("edge-fra-02");
    await expect(copy).toBeFocused();
    await expect(copy).not.toHaveAttribute("data-yi-state", /.+/, { timeout: 4000 });
    await expect(page.locator("[data-yi-announcer]")).toHaveText("");
  });

  test("a labelled button confirms in place and a disabled one does nothing", async ({ page, example }) => {
    const stage = await example("copyable/button");

    await stage.getByRole("button", { name: "Copy command" }).last().click();
    await expect(stage.getByRole("button", { name: "Copied" })).toBeVisible();
    await expect(stage.getByRole("status")).toHaveText("Copied 1 times");
    expect(await clipboardText(page)).toBe("nomad job run caddy.hcl");

    const disabled = stage.getByRole("button", { name: "Copy", exact: true });
    await expect(disabled).toBeDisabled();
    await disabled.click({ force: true });
    await expect(stage.getByRole("status")).toHaveText("Copied 1 times");
  });

  test("use_clipboard drives a custom trigger", async ({ page, example }) => {
    const stage = await example("copyable/hook");
    await stage.getByRole("button", { name: "Share this rollout" }).click();
    await expect(stage.getByRole("status")).toHaveText("Link copied to the clipboard");
    expect(await clipboardText(page)).toBe("https://buckle.example.com/rollouts/1842");
  });

  test("still copies where the async clipboard is missing, as over plain HTTP", async ({ page, example }) => {
    await page.addInitScript(() => Object.defineProperty(Navigator.prototype, "clipboard", { get: () => undefined }));
    const stage = await example("copyable/basic");
    const copy = stage.getByRole("button", { name: "Copy" }).first();

    await copy.click();

    await expect(stage.getByRole("status")).toHaveText(`Copied ${SHA}`);
    expect(await clipboardText(page)).toBe(SHA);
    await expect(copy).toBeFocused();
  });

  test("reports a refusal and selects the value to copy by hand", async ({ page, example }) => {
    await page.addInitScript(() => {
      navigator.clipboard.writeText = () => Promise.reject(new DOMException("denied", "NotAllowedError"));
      document.execCommand = () => false;
    });
    const stage = await example("copyable/basic");
    const copy = stage.getByRole("button", { name: "Copy" }).first();

    await copy.click();

    await expect(copy).toHaveAttribute("data-yi-state", "failed");
    await expect(copy).toHaveAttribute("title", "Copy failed");
    await expect(page.locator("[data-yi-announcer]")).toHaveText("Copy failed");
    await expect(stage.getByRole("status")).toHaveText("Nothing copied yet");
    expect(await page.evaluate(() => getSelection()?.toString())).toBe("4f9a2c1");
  });

  test("a truncated value stays on one line inside its field", async ({ example }) => {
    const stage = await example("copyable/field");
    const [wrapped, truncated] = await Promise.all(
      [0, 1].map((i) => stage.locator(".yi-copyable").nth(i).boundingBox()),
    );
    expect(truncated!.height).toBeLessThan(wrapped!.height);
    expect(truncated!.width).toBeCloseTo(wrapped!.width, 0);
  });
});
