import { expect, test } from "../fixtures";

test.describe("Pipeline", () => {
  test("opens a gate when its stage is approved", async ({ example }) => {
    const stage = await example("pipeline/gates");
    const production = stage.getByRole("listitem").filter({ hasText: "Production" });

    await expect(production).toHaveAttribute("aria-current", "step");
    await expect(production).toContainText("Gate: needs approval");

    await production.getByRole("button", { name: "Approve" }).click();
    await expect(stage.getByRole("status")).toHaveText("Production approved");
    await expect(production).toContainText("Gate: approved");
    await expect(production).toContainText("rolling out");
  });

  test("approves from the keyboard", async ({ page, example }) => {
    const stage = await example("pipeline/gates");
    await page.keyboard.press("Tab");
    await expect(stage.getByRole("button", { name: "Approve" })).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(stage.getByRole("status")).toHaveText("Production approved");
  });

  test("names every unit and lets the keyboard reach the linked ones", async ({ page, example }) => {
    const stage = await example("pipeline/rollout");
    const canary = stage.getByRole("group", { name: "Hosts in canary" });

    await expect(canary.getByRole("link")).toHaveCount(2);
    await expect(stage.getByRole("link", { name: "edge-06: failed" })).toHaveAttribute("href", "#edge-06");
    await expect(stage.getByRole("img", { name: "lab-02: offline" })).toBeVisible();

    await page.keyboard.press("Tab");
    const first = stage.getByRole("link", { name: "canary-01: healthy" });
    await expect(first).toBeFocused();
    await expect(first).not.toHaveCSS("outline-style", "none");
  });

  test("lays stages out in a row, and in a column when there is no room", async ({ page, example }) => {
    await page.setViewportSize({ width: 1280, height: 800 });
    const stage = await example("pipeline/basic");
    const stages = stage.getByRole("list", { name: "Release stages" }).locator("> li");
    await expect(stages).toHaveCount(3);
    expect((await stages.nth(1).boundingBox())!.x).toBeGreaterThan((await stages.nth(0).boundingBox())!.x);

    await page.setViewportSize({ width: 390, height: 844 });
    await expect.poll(async () => (await stages.nth(1).boundingBox())!.y).toBeGreaterThan((await stages.nth(0).boundingBox())!.y);
    expect((await stages.nth(1).boundingBox())!.x).toBe((await stages.nth(0).boundingBox())!.x);
  });
});
