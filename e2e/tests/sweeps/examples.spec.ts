// Runs for every example on the site, with nothing to write per component:
// screenshots in both themes at two widths, an accessibility scan, and a
// check that hydration leaves the server-rendered markup alone.
import AxeBuilder from "@axe-core/playwright";
import { examples, expect, hydrated, test, type Theme } from "../../fixtures";

const THEMES: Theme[] = ["light", "dark"];
const VIEWPORTS = {
  desktop: { width: 1280, height: 800 },
  phone: { width: 390, height: 844 },
};

/** The markup a user would see, without Yew's hydration bookkeeping. */
const visibleMarkup = (html: string) => html.replace(/<!--.*?-->/gs, "").replace(/\s+/g, " ").trim();

for (const info of examples) {
  const id = `${info.page}/${info.example}`;

  test.describe(id, () => {
    for (const theme of THEMES) {
      for (const [device, viewport] of Object.entries(VIEWPORTS)) {
        test(`looks right: ${theme}, ${device}`, async ({ page, example }) => {
          await page.setViewportSize(viewport);
          const stage = await example(id, { theme });
          await expect(stage).toHaveScreenshot([info.page, `${info.example}-${theme}-${device}.png`]);
        });
      }

      test(`is accessible: ${theme}`, async ({ page, example }) => {
        await example(id, { theme });
        // Examples are fragments shown at any depth, so heading order is the host page's concern.
        const results = await new AxeBuilder({ page }).include("[data-example]").disableRules(["heading-order"]).analyze();
        expect(results.violations.map((v) => `${v.id}: ${v.help} (${v.nodes.map((n) => n.target).join(", ")})`)).toEqual([]);
      });
    }

    test("hydrates without changing the server render", async ({ browser, page, problems, baseURL }) => {
      const server = await browser.newContext({ javaScriptEnabled: false });
      const served = await server.newPage();
      await served.goto(`${baseURL}${info.path}`);
      const before = visibleMarkup(await served.locator("[data-example]").innerHTML());
      await server.close();

      await page.goto(info.path);
      await hydrated(page);
      const after = visibleMarkup(await page.locator("[data-example]").innerHTML());

      expect(after).toBe(before);
      expect(problems).toEqual([]);
    });
  });
}
