import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { expect, test as base, type Locator, type Page } from "@playwright/test";

export { expect };

export type Theme = "light" | "dark";

export interface ExampleInfo {
  page: string;
  example: string;
  title: string;
  path: string;
}

const recorder = fileURLToPath(new URL("./replay/recorder.js", import.meta.url));
const dist = fileURLToPath(new URL("../site/dist/", import.meta.url));
const manifest = <T>(name: string): T => {
  try {
    return JSON.parse(readFileSync(dist + name, "utf8"));
  } catch {
    throw new Error(`site/dist/${name} is missing: run \`cargo xtask site\` first`);
  }
};

/** Every example on the reference site, as the prerenderer listed them. */
export const examples = manifest<ExampleInfo[]>("examples.json");
/** Every documentation route. */
export const pages = manifest<string[]>("pages.json");

/**
 * Resolves once the wasm has hydrated the page and nothing on it is still
 * loading. A component that finishes drawing after hydration (a map, a
 * canvas) marks itself `data-yi-pending` until it has.
 */
export async function hydrated(page: Page) {
  await page.locator("html[data-hydrated]").waitFor({ state: "attached" });
  await expect(page.locator("[data-yi-pending]")).toHaveCount(0, { timeout: 15_000 });
  await page.evaluate(() => document.fonts.ready);
}

/** Console noise that says nothing about the page: headless WebGL reads frames back in software. */
const NOISE = /GL Driver Message .* GPU stall due to ReadPixels/;

export interface Fixtures {
  /**
   * Opens one example on its isolated route, waits for it to become
   * interactive and returns its stage: `await example("button/states")`.
   */
  example: (id: string, options?: { theme?: Theme }) => Promise<Locator>;
  /** Console errors, warnings and uncaught exceptions seen so far. */
  problems: string[];
  /** Records what a behaviour spec does to each example, for the review gallery to replay. */
  replay: void;
}

export const test = base.extend<Fixtures>({
  problems: async ({ page }, use) => {
    const problems: string[] = [];
    page.on("console", (message) => {
      if (["error", "warning"].includes(message.type()) && !NOISE.test(message.text())) {
        problems.push(`${message.type()}: ${message.text()}`);
      }
    });
    page.on("pageerror", (error) => problems.push(`uncaught: ${error.message}`));
    await use(problems);
  },

  replay: [
    async ({ page }, use, testInfo) => {
      if (testInfo.project.name !== "behaviour") return use();
      // One segment per example document the spec opened, in order.
      const segments = new Map<string, { example: string; events: unknown[]; end?: string }>();
      await page.exposeBinding("__yiReplayEmit", (_source, { doc, example, event }) => {
        if (!segments.has(doc)) segments.set(doc, { example, events: [] });
        segments.get(doc)!.events.push(event);
      });
      await page.addInitScript({ path: recorder });
      await use();
      // What the example read at the end, so a replay can be checked against it.
      const end = await page
        .evaluate(() => (document.querySelector("[data-example]") as HTMLElement | null)?.innerText)
        .catch(() => undefined);
      const last = [...segments.values()].at(-1);
      if (last && end !== undefined) last.end = end;
      const recorded = [...segments.values()].filter((segment) => segment.events.length > 0);
      if (recorded.length === 0) return;
      const file = testInfo.outputPath("replay.json");
      writeFileSync(file, JSON.stringify(recorded));
      await testInfo.attach("replay", { path: file, contentType: "application/json" });
    },
    { auto: true },
  ],

  example: async ({ page }, use) => {
    await use(async (id, { theme } = {}) => {
      const [name, example] = id.split("/");
      if (!examples.some((e) => e.page === name && e.example === example)) {
        throw new Error(`no example "${id}"; known: ${examples.map((e) => `${e.page}/${e.example}`).join(", ")}`);
      }
      await page.goto(`/_examples/${id}${theme ? `?theme=${theme}` : ""}`);
      await hydrated(page);
      return page.locator("[data-example]");
    });
  },
});
