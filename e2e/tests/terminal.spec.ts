import type { Locator } from "@playwright/test";
import { expect, test } from "../fixtures";

/** How far the output is from being scrolled to its end, in pixels. */
const distanceFromEnd = (log: Locator) => log.evaluate((el) => el.scrollHeight - el.scrollTop - el.clientHeight);

test.describe("Terminal", () => {
  test("follows new output until the reader scrolls up, then resumes at the end", async ({ example }) => {
    const stage = await example("terminal/streaming");
    const log = stage.getByRole("log", { name: "deploy output" });
    const more = stage.getByRole("button", { name: "Print more output" });
    const status = stage.getByRole("status");

    await more.click();
    await expect(status).toHaveText("7 lines printed");
    await expect(log).toContainText("[  7]");
    await expect.poll(() => distanceFromEnd(log)).toBeLessThanOrEqual(4);

    // Reading earlier output pauses following.
    await log.hover();
    await log.evaluate((el) => el.scrollTo({ top: 0 }));
    await more.click();
    await expect(status).toHaveText("10 lines printed");
    await expect(log).toContainText("[ 10]");
    expect(await log.evaluate((el) => el.scrollTop)).toBe(0);

    // Returning to the end resumes it.
    await log.evaluate((el) => el.scrollTo({ top: el.scrollHeight }));
    await expect.poll(() => distanceFromEnd(log)).toBeLessThanOrEqual(4);
    await more.click();
    await expect(log).toContainText("[ 13]");
    await expect.poll(() => distanceFromEnd(log)).toBeLessThanOrEqual(4);
  });

  test("a scrolling body can be reached and scrolled from the keyboard", async ({ page, example }) => {
    const stage = await example("terminal/streaming");
    const log = stage.getByRole("log", { name: "deploy output" });
    await stage.getByRole("button", { name: "Print more output" }).click();
    await expect(log).toContainText("[  7]");

    await page.keyboard.press("Shift+Tab");
    await expect(log).toBeFocused();
    await expect(log).not.toHaveCSS("outline-style", "none");

    await page.keyboard.press("Home");
    await expect.poll(() => log.evaluate((el) => el.scrollTop)).toBe(0);
    await page.keyboard.press("End");
    await expect.poll(() => distanceFromEnd(log)).toBeLessThanOrEqual(4);
  });

  test("selecting a command leaves the prompt out", async ({ page, example }) => {
    const stage = await example("terminal/session");
    const command = stage.getByText("trunk build --release");

    await command.click({ clickCount: 3 });
    const selected = await page.evaluate(() => window.getSelection()?.toString().trim());
    expect(selected).toBe("trunk build --release");
  });

  test("renders ANSI output as text, without escape codes", async ({ example }) => {
    const stage = await example("terminal/ansi");
    const terminal = stage.getByRole("group", { name: "cargo test" });

    await expect(terminal).toContainText("Compiling yewi v0.1.0");
    await expect(terminal).toContainText("test table::keeps_selection ... FAILED");
    await expect(terminal).not.toContainText("[0m");

    const colour = (text: string) => terminal.getByText(text, { exact: true }).first().evaluate((el) => getComputedStyle(el).color);
    expect(await colour("ok")).not.toBe(await colour("FAILED"));
  });

  for (const theme of ["light", "dark"] as const) {
    test(`is dark in the ${theme} theme`, async ({ example }) => {
      const stage = await example("terminal/session", { theme });
      const terminal = stage.getByRole("group", { name: /workstation/ });

      await expect(terminal).toHaveCSS("color-scheme", "dark");
      const lightness = await terminal.evaluate((el) => {
        const canvas = document.createElement("canvas").getContext("2d")!;
        canvas.fillStyle = getComputedStyle(el).backgroundColor;
        canvas.fillRect(0, 0, 1, 1);
        const [r, g, b] = canvas.getImageData(0, 0, 1, 1).data;
        return (r + g + b) / 3;
      });
      expect(lightness).toBeLessThan(64);
    });
  }
});
