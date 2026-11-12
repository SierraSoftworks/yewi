import { expect, test } from "../fixtures";

test.describe("Text", () => {
  test("truncates to one line and clamps to the requested lines", async ({ example }) => {
    const stage = await example("text/truncate");
    const [single, two, three] = await stage.locator(".yi-text").evaluateAll((els) =>
      els.map((el) => ({
        height: el.getBoundingClientRect().height,
        lineHeight: parseFloat(getComputedStyle(el).lineHeight),
        clipped: el.scrollWidth > el.clientWidth || el.scrollHeight > el.clientHeight,
      })),
    );

    expect(single.clipped).toBe(true);
    expect(single.height).toBeLessThan(single.lineHeight * 1.5);
    expect(two.clipped).toBe(true);
    expect(Math.round(two.height / two.lineHeight)).toBe(2);
    expect(Math.round(three.height / three.lineHeight)).toBe(3);
  });

  test("truncated text keeps its full content for assistive technology", async ({ example }) => {
    const stage = await example("text/truncate");
    await expect(stage.locator(".yi-text").first()).toContainText("no longer pauses the whole rollout");
  });

  test("tabular digits share one width", async ({ example }) => {
    const stage = await example("text/numbers");
    await expect(stage.getByText("12,904")).toHaveCSS("font-variant-numeric", "tabular-nums");
  });

  test("status tones differ from neutral text", async ({ example }) => {
    const stage = await example("text/colours");
    const colour = (name: string) => stage.getByText(name, { exact: true }).evaluate((el) => getComputedStyle(el).color);
    const colours = await Promise.all(["caddy-ingress", "Healthy", "Degraded", "Failed"].map(colour));
    expect(new Set(colours).size).toBe(colours.length);
  });
});
