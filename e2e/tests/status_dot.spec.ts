import { expect, test } from "../fixtures";

test.describe("StatusDot", () => {
  test("is announced only when it carries the status alone", async ({ example }) => {
    const labelled = await example("status-dot/standalone");
    await expect(labelled.getByRole("img", { name: "Failing" })).toBeVisible();
    await expect(labelled.getByRole("img", { name: "Healthy", exact: true })).toHaveCount(2);

    const beside = await example("status-dot/tones");
    await expect(beside.getByRole("img")).toHaveCount(0);
    await expect(beside.getByText("failing")).toBeVisible();
  });

  test("pulses only when active, and never for reduced motion", async ({ page, example }) => {
    const stage = await example("status-dot/active");
    const animations = () =>
      stage.locator("[aria-hidden=true]").evaluateAll((dots) => dots.map((dot) => getComputedStyle(dot).animationName !== "none"));

    expect(await animations()).toEqual([true, true, false]);

    await page.emulateMedia({ reducedMotion: "reduce" });
    expect(await animations()).toEqual([false, false, false]);
  });

  test("follows the surrounding text colour without a tone", async ({ example }) => {
    const stage = await example("status-dot/inherited");
    const colours = await stage.locator("[aria-hidden=true]").evaluateAll((dots) =>
      dots.map((dot) => [getComputedStyle(dot).backgroundColor, getComputedStyle(dot.parentElement!).color]),
    );
    expect(colours).toHaveLength(3);
    for (const [dot, text] of colours) expect(dot).toBe(text);
  });
});
