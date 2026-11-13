import { examples, expect, test } from "../fixtures";

const FORM = examples.find((info) => info.path.endsWith("native-select/form"))!.path;

test.describe("NativeSelect", () => {
  test("reports the chosen option", async ({ example }) => {
    const stage = await example("native-select/basic");
    const region = stage.getByRole("combobox", { name: "Region" });

    await expect(region).toHaveValue("eu-west-1");
    await region.selectOption("Frankfurt");
    await expect(stage.getByRole("status")).toHaveText("Region eu-central-1");
  });

  test("submits a form without a client runtime", async ({ browser, baseURL }) => {
    const context = await browser.newContext({ javaScriptEnabled: false });
    const page = await context.newPage();
    await page.goto(`${baseURL}${FORM}`);

    const plan = page.getByRole("combobox", { name: "Plan" });
    await expect(plan).toHaveValue("team");
    await plan.selectOption("Enterprise");

    // The placeholder is not a choice, so the browser holds the form back.
    await page.getByRole("button", { name: "Save" }).click();
    expect(new URL(page.url()).search).toBe("");

    await page.getByRole("combobox", { name: "Billing" }).selectOption("Yearly");
    await page.getByRole("button", { name: "Save" }).click();
    await page.waitForURL(/plan=enterprise/);
    expect(new URL(page.url()).searchParams.get("billing")).toBe("yearly");
    await context.close();
  });
});
