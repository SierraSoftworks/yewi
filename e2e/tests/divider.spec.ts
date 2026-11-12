import { expect, test } from "../fixtures";

test.describe("Divider", () => {
  test("is a separator that spans its container", async ({ example }) => {
    const stage = await example("divider/basic");
    const rule = stage.getByRole("separator");
    const box = (await rule.boundingBox())!;
    const parent = (await rule.locator("..").boundingBox())!;
    expect(box.width).toBeCloseTo(parent.width, 0);
    expect(box.height).toBeLessThanOrEqual(1);
  });

  test("vertical dividers stretch to the row and say which way they run", async ({ example }) => {
    const stage = await example("divider/vertical");
    const rules = stage.getByRole("separator");
    await expect(rules).toHaveCount(2);
    await expect(rules.first()).toHaveAttribute("aria-orientation", "vertical");

    const rule = (await rules.first().boundingBox())!;
    const button = (await stage.getByRole("button", { name: "Approve" }).boundingBox())!;
    expect(rule.height).toBeGreaterThanOrEqual(button.height - 1);
    expect(rule.width).toBeLessThanOrEqual(1);
  });

  test("labels are readable text", async ({ example }) => {
    const stage = await example("divider/labelled");
    for (const label of ["or", "Yesterday", "End of log"]) {
      await expect(stage.getByText(label, { exact: true })).toBeVisible();
    }
  });
});
