import { readFileSync } from "node:fs";
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

/** Resolves once the wasm has hydrated the page. */
export async function hydrated(page: Page) {
  await page.locator("html[data-hydrated]").waitFor({ state: "attached" });
  await page.evaluate(() => document.fonts.ready);
}

export interface Fixtures {
  /**
   * Opens one example on its isolated route, waits for it to become
   * interactive and returns its stage: `await example("button/states")`.
   */
  example: (id: string, options?: { theme?: Theme }) => Promise<Locator>;
  /** Console errors, warnings and uncaught exceptions seen so far. */
  problems: string[];
}

export const test = base.extend<Fixtures>({
  problems: async ({ page }, use) => {
    const problems: string[] = [];
    page.on("console", (message) => {
      if (["error", "warning"].includes(message.type())) problems.push(`${message.type()}: ${message.text()}`);
    });
    page.on("pageerror", (error) => problems.push(`uncaught: ${error.message}`));
    await use(problems);
  },

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
