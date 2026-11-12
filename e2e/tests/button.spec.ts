import { expect, test } from "../fixtures";

test.describe("Button", () => {
  test("activates with the mouse and the keyboard", async ({ page, example }) => {
    const stage = await example("button/states");
    const status = stage.getByRole("status");
    const finish = stage.getByRole("button", { name: "Finish saving" });

    await stage.getByRole("button", { name: "Save" }).click();
    await expect(status).toHaveText("Saved 1 times");

    await finish.click();
    await stage.getByRole("button", { name: "Save" }).focus();
    await page.keyboard.press("Enter");
    await expect(status).toHaveText("Saved 2 times");

    await finish.click();
    await page.keyboard.press("Shift+Tab");
    await page.keyboard.press("Space");
    await expect(status).toHaveText("Saved 3 times");
  });

  test("ignores clicks while loading, but stays focusable and announces it", async ({ example }) => {
    const stage = await example("button/states");
    const save = stage.getByRole("button", { name: "Save" });

    await save.click();
    await expect(save).toHaveAttribute("aria-busy", "true");
    await save.click({ force: true });
    await expect(stage.getByRole("status")).toHaveText("Saved 1 times");

    await save.focus();
    await expect(save).toBeFocused();

    await stage.getByRole("button", { name: "Finish saving" }).click();
    await expect(save).not.toHaveAttribute("aria-busy", "true");
  });

  test("cannot be focused or clicked while disabled", async ({ page, example }) => {
    const stage = await example("button/states");
    const unavailable = stage.getByRole("button", { name: "Unavailable" });

    await expect(unavailable).toBeDisabled();
    await page.keyboard.press("Tab");
    await page.keyboard.press("Tab");
    await page.keyboard.press("Tab");
    await expect(unavailable).not.toBeFocused();
  });

  test("with href is a real link", async ({ example }) => {
    const stage = await example("button/link");
    await expect(stage.getByRole("link", { name: "Source" })).toHaveAttribute("href", /github\.com/);
  });

  test("shows a visible focus indicator for keyboard users", async ({ page, example }) => {
    const stage = await example("button/variants");
    await page.keyboard.press("Tab");
    const focused = stage.getByRole("button", { name: "Approve" });
    await expect(focused).toBeFocused();
    await expect(focused).not.toHaveCSS("outline-style", "none");
  });
});
