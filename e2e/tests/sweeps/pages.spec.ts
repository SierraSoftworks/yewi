// Every documentation page hydrates cleanly and passes an accessibility scan.
import AxeBuilder from "@axe-core/playwright";
import { expect, hydrated, pages, test } from "../../fixtures";

for (const path of pages) {
  test(`${path} hydrates cleanly and is accessible`, async ({ page, problems }) => {
    const response = await page.goto(path);
    await hydrated(page);

    expect(response?.ok()).toBe(true);
    await expect(page.getByRole("heading", { level: 1 }).first()).toBeVisible();
    expect(problems).toEqual([]);

    // Example stages are scanned one by one in examples.spec.ts.
    const results = await new AxeBuilder({ page }).exclude(".docs-example__stage").analyze();
    expect(results.violations.map((v) => `${v.id}: ${v.help} (${v.nodes.map((n) => n.target).join(", ")})`)).toEqual([]);
  });
}
