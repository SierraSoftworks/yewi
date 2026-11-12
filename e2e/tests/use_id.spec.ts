import { expect, test } from "../fixtures";

test.describe("use_id", () => {
  // Yew renders depth-first on the server and level by level on the client, so these pairs are
  // numbered differently by the two. Hydration must leave every label on its own input.
  test("labels stay wired to their own inputs after hydration", async ({ example }) => {
    const stage = await example("use-id/nested");

    for (const name of ["Street", "City", "Country"]) {
      const input = stage.getByLabel(name);
      await expect(input).toHaveCount(1);
      const row = stage.locator("label", { hasText: name }).locator("..");
      expect(await input.getAttribute("id")).toBe(await row.locator("input").getAttribute("id"));
    }

    const ids = await stage.locator("input").evaluateAll((inputs) => inputs.map((input) => input.id));
    expect(new Set(ids).size).toBe(3);
  });
});
