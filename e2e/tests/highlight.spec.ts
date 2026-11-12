import { expect, test } from "../fixtures";

const colour = (el: Element) => getComputedStyle(el).color;

test.describe("Highlight", () => {
  test("colours token kinds differently and changes with the theme", async ({ example }) => {
    const seen: Record<string, string[]> = {};
    for (const theme of ["light", "dark"] as const) {
      const stage = await example("highlight/tokens", { theme });
      const code = stage.locator("pre").first();
      const plain = await code.evaluate(colour);
      const [keyword, string, comment] = await Promise.all(
        ["keyword", "string", "comment"].map((kind) => code.locator(`.yi-tok-${kind}`).first().evaluate(colour)),
      );
      expect(new Set([plain, keyword, string, comment]).size).toBe(4);
      seen[theme] = [keyword, string, comment];
    }
    expect(seen.light).not.toEqual(seen.dark);
  });

  test("a token colour can be overridden for a subtree", async ({ example }) => {
    const stage = await example("highlight/tokens");
    const [standard, overridden] = await Promise.all(
      [0, 1].map((i) => stage.locator("pre").nth(i).locator(".yi-tok-comment").first().evaluate(colour)),
    );
    expect(overridden).not.toBe(standard);
  });

  test("the text is the source, whatever the language", async ({ example }) => {
    const stage = await example("highlight/inline");
    await expect(stage.locator("p code").first()).toHaveText('std::env::var("LOG_LEVEL")');
    await expect(stage.locator("pre code")).toContainText("WHERE status >= 500 -- server errors only");
  });
});
