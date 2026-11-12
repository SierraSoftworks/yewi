import { expect, test } from "../fixtures";

test.describe("Pill", () => {
  test("shows each state as it changes", async ({ example }) => {
    const stage = await example("pill/live");
    const status = stage.getByRole("status");

    await expect(status).toHaveText("Rollout is idle");
    await stage.getByRole("button", { name: "Start rollout" }).click();
    await expect(status).toHaveText("Rollout is rolling");
    await expect(stage.getByText("rolling", { exact: true })).toBeVisible();

    await stage.getByRole("button", { name: "Finish rollout" }).click();
    await expect(status).toHaveText("Rollout is healthy");
    await expect(stage.getByText("healthy", { exact: true })).toBeVisible();
  });

  test("tones and bold pills are told apart by colour", async ({ example }) => {
    const stage = await example("pill/emphasis");
    const fill = (name: string, nth: number) =>
      stage.getByText(name, { exact: true }).nth(nth).evaluate((pill) => getComputedStyle(pill).backgroundColor);

    // "rolling" and "failing" appear soft in the first row and bold in the second.
    expect(await fill("rolling", 0)).not.toBe(await fill("rolling", 1));
    expect(await fill("rolling", 1)).not.toBe(await fill("failing", 1));
  });

  test("with href is a link that keyboard users can reach and see", async ({ page, example }) => {
    const stage = await example("pill/link");
    const link = stage.getByRole("link", { name: "rolling" });

    await expect(link).toHaveAttribute("href", "#rollout-42");
    await page.keyboard.press("Tab");
    await expect(link).toBeFocused();
    await expect(link).not.toHaveCSS("outline-style", "none");

    await page.keyboard.press("Enter");
    await expect(page).toHaveURL(/#rollout-42$/);
  });
});
