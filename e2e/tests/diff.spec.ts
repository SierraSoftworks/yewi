import { expect, test } from "../fixtures";

test.describe("Diff", () => {
  test("folds unchanged lines and reveals them with the mouse", async ({ example }) => {
    const stage = await example("diff/compare");
    const hidden = stage.getByText('driver = "docker"');
    const folds = stage.getByRole("button", { name: /unchanged lines/ });

    await expect(hidden).toHaveCount(0);
    await expect(folds).toHaveCount(3);

    await folds.nth(1).click();
    await expect(hidden).toBeVisible();
    await expect(folds).toHaveCount(2);
  });

  test("reveals folded lines from the keyboard and keeps focus in the diff", async ({ page, example }) => {
    const stage = await example("diff/compare");
    const region = stage.getByRole("group", { name: "caddy.nomad.hcl" });
    const first = stage.getByRole("button", { name: /unchanged lines/ }).first();

    await page.keyboard.press("Tab");
    await expect(region).toBeFocused();
    await expect(region).not.toHaveCSS("outline-style", "none");

    await page.keyboard.press("Tab");
    await expect(first).toBeFocused();
    await expect(first).not.toHaveCSS("outline-style", "none");

    await page.keyboard.press("Enter");
    await expect(stage.getByText('job "caddy" {')).toBeVisible();
    await expect(region).toBeFocused();
  });

  test("announces additions and removals, not only their colour", async ({ example }) => {
    const stage = await example("diff/compare");
    await expect(stage.getByText("6 lines added, 3 lines removed")).toBeAttached();
    await expect(stage.getByRole("insertion").filter({ hasText: "count = 2" })).toBeVisible();
    await expect(stage.getByRole("deletion").filter({ hasText: "count = 1" })).toBeVisible();
    // Only the edited part of a line is marked.
    await expect(stage.getByRole("insertion").filter({ hasText: "caddy:2.8" }).locator("mark")).toHaveText("8");
  });

  test("copies the text without line numbers or markers", async ({ page, example }) => {
    const stage = await example("diff/unified");
    const added = stage.getByRole("insertion").filter({ hasText: "backoff.min" });
    await added.selectText();
    const selected = await page.evaluate(() => getSelection()?.toString());
    expect(selected).toBe("        backoff.min(self.max)");
    await expect(stage.getByText("src/retry.rs")).toBeVisible();
  });

  test("switches between side by side and unified", async ({ example }) => {
    const stage = await example("diff/split");
    const removed = stage.getByRole("deletion").filter({ hasText: "8080" });
    const added = stage.getByRole("insertion").filter({ hasText: "8443" });
    const sameRow = async () => {
      const [a, b] = [await removed.boundingBox(), await added.boundingBox()];
      return a!.y === b!.y;
    };

    expect(await sameRow()).toBe(true);

    await stage.getByRole("button", { name: "Unified" }).click();
    await expect(stage.getByRole("button", { name: "Unified" })).toHaveAttribute("aria-pressed", "true");
    expect(await sameRow()).toBe(false);
  });

  test("stacks the two sides when there is no room for both", async ({ page, example }) => {
    await page.setViewportSize({ width: 420, height: 800 });
    const stage = await example("diff/split");
    const removed = await stage.getByRole("deletion").filter({ hasText: "8080" }).boundingBox();
    const added = await stage.getByRole("insertion").filter({ hasText: "8443" }).boundingBox();
    expect(added!.y).toBeGreaterThan(removed!.y);
    expect(await stage.locator(".yi-diff__scroll").evaluate((el) => el.scrollWidth <= el.clientWidth)).toBe(true);
  });

  test("draws custom line content", async ({ example }) => {
    const stage = await example("diff/tokens");
    await expect(stage.getByTitle("Secret nomad_server, value fingerprint b21c")).toBeVisible();
    await expect(stage.getByText("«secret:")).toHaveCount(0);
  });

  test("scrolls a long diff inside itself from the keyboard", async ({ page, example }) => {
    const stage = await example("diff/long");
    const region = stage.getByRole("group", { name: "firewall.rules" });
    await region.focus();
    await page.keyboard.press("End");
    await expect.poll(() => region.evaluate((el) => el.scrollTop)).toBeGreaterThan(0);
    // Wrapped lines never scroll sideways.
    expect(await region.evaluate((el) => el.scrollWidth <= el.clientWidth)).toBe(true);
  });
});
