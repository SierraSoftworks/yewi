import { expect, test, type Theme } from "../fixtures";

const THEMES: Theme[] = ["light", "dark"];

test.describe("Dialog", () => {
  test("opens as a modal, holds the keyboard and returns focus on Escape", async ({ page, example }) => {
    const stage = await example("dialog/basic");
    const trigger = stage.getByRole("button", { name: "Review rollout" });
    const dialog = page.getByRole("dialog", { name: "Resume rollout web-42?" });

    await expect(dialog).toBeHidden();
    await trigger.focus();
    await page.keyboard.press("Enter");
    await expect(dialog).toBeVisible();
    expect(await dialog.evaluate((el) => el.matches(":modal"))).toBe(true);

    // Tab never leaves the dialog for the page behind it.
    for (let i = 0; i < 6; i++) {
      await page.keyboard.press("Tab");
      expect(await dialog.evaluate((el) => el.contains(document.activeElement) || document.activeElement === document.body)).toBe(true);
    }
    await expect(trigger).not.toBeFocused();

    await page.keyboard.press("Escape");
    await expect(dialog).toBeHidden();
    await expect(trigger).toBeFocused();
    await expect(stage.getByRole("status")).toHaveText("Rollout web-42 is paused");
  });

  test("closes from the close button and the backdrop, but not from a click inside", async ({ page, example }) => {
    const stage = await example("dialog/basic");
    const trigger = stage.getByRole("button", { name: "Review rollout" });
    const dialog = page.getByRole("dialog", { name: "Resume rollout web-42?" });

    await trigger.click();
    await dialog.getByText("Resuming continues").click();
    await expect(dialog).toBeVisible();
    await dialog.getByRole("button", { name: "Close" }).click();
    await expect(dialog).toBeHidden();

    await trigger.click();
    await expect(dialog).toBeVisible();
    await page.mouse.click(8, 8);
    await expect(dialog).toBeHidden();
    await expect(trigger).toBeFocused();
  });

  test("a press that starts inside and ends on the backdrop does not dismiss it", async ({ page, example }) => {
    const stage = await example("dialog/basic");
    await stage.getByRole("button", { name: "Review rollout" }).click();
    const dialog = page.getByRole("dialog", { name: "Resume rollout web-42?" });
    const box = (await dialog.boundingBox())!;

    await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
    await page.mouse.down();
    await page.mouse.move(8, 8);
    await page.mouse.up();
    await expect(dialog).toBeVisible();
  });

  test("runs a footer action and reports it", async ({ page, example }) => {
    const stage = await example("dialog/basic");
    await stage.getByRole("button", { name: "Review rollout" }).click();
    await page.getByRole("dialog").getByRole("button", { name: "Resume rollout" }).click();
    await expect(page.getByRole("dialog")).toBeHidden();
    await expect(stage.getByRole("status")).toHaveText("Rollout web-42 resumed");
  });

  test("each size opens its own dialog, widest last", async ({ page, example }) => {
    const stage = await example("dialog/sizes");
    const widths: number[] = [];
    for (const size of ["Small", "Medium", "Large"]) {
      await stage.getByRole("button", { name: size }).click();
      const dialog = page.getByRole("dialog", { name: `${size} dialog` });
      await expect(dialog).toBeVisible();
      widths.push((await dialog.boundingBox())!.width);
      await dialog.getByRole("button", { name: "Done" }).click();
      await expect(dialog).toBeHidden();
    }
    expect(widths[0]).toBeLessThan(widths[1]);
    expect(widths[1]).toBeLessThan(widths[2]);
  });

  test("fits a phone and scrolls long content with the actions in view", async ({ page, example }) => {
    await page.setViewportSize({ width: 390, height: 844 });
    const stage = await example("dialog/long");
    await stage.getByRole("button", { name: "Enrol this cluster" }).click();
    const dialog = page.getByRole("dialog", { name: "What enrolling a cluster means" });
    const box = (await dialog.boundingBox())!;

    expect(box.x).toBeGreaterThanOrEqual(0);
    expect(box.x + box.width).toBeLessThanOrEqual(390);
    expect(box.y + box.height).toBeLessThanOrEqual(844);
    await expect(dialog.getByRole("button", { name: "Accept and enrol" })).toBeInViewport();
    await expect(dialog.getByRole("paragraph").last()).not.toBeInViewport();
    await dialog.getByRole("paragraph").last().scrollIntoViewIfNeeded();
    await expect(dialog.getByRole("paragraph").last()).toBeInViewport();
  });

  test("when not dismissible, Escape and the backdrop do nothing", async ({ page, example }) => {
    const stage = await example("dialog/long");
    await stage.getByRole("button", { name: "Enrol this cluster" }).click();
    const dialog = page.getByRole("dialog", { name: "What enrolling a cluster means" });

    await expect(dialog.getByRole("button", { name: "Close" })).toHaveCount(0);
    // Twice: a browser stops honouring a cancelled Escape when it is repeated.
    await page.keyboard.press("Escape");
    await page.keyboard.press("Escape");
    await page.mouse.click(8, 8);
    await expect(stage.getByRole("status")).toHaveText("Not answered yet");

    await page.getByRole("dialog").getByRole("button", { name: "Accept and enrol" }).click();
    await expect(page.getByRole("dialog")).toBeHidden();
    await expect(stage.getByRole("status")).toHaveText("Accepted");
  });
});

test.describe("ConfirmDialog", () => {
  test("starts on the way out, so Enter cancels and the action needs a choice", async ({ page, example }) => {
    const stage = await example("dialog/confirm");
    const trigger = stage.getByRole("button", { name: "Forget host" });
    const dialog = page.getByRole("alertdialog", { name: "Forget pi-1?" });

    await trigger.click();
    await expect(dialog).toBeVisible();
    await expect(dialog).toHaveAccessibleDescription(/removed from the cluster/);
    await expect(dialog.getByRole("button", { name: "Cancel" })).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(dialog).toBeHidden();
    await expect(stage.getByRole("status")).toHaveText("pi-1 is registered");
    await expect(trigger).toBeFocused();

    await trigger.click();
    await dialog.getByRole("button", { name: "Forget pi-1" }).click();
    await expect(dialog).toBeHidden();
    await expect(stage.getByRole("status")).toHaveText("pi-1 was forgotten");
  });

  test("stays disabled until the statement is ticked and the word is typed", async ({ page, example }) => {
    const stage = await example("dialog/steps");
    const trigger = stage.getByRole("button", { name: "Delete cluster" });
    const dialog = page.getByRole("alertdialog", { name: "Delete cluster prod-eu?" });
    const confirm = dialog.getByRole("button", { name: "Delete cluster" });
    const statement = dialog.getByRole("checkbox", { name: "I have reviewed the 4 destructive steps" });
    const word = dialog.getByRole("textbox", { name: "Type prod-eu to confirm" });

    await trigger.click();
    await expect(statement).toBeFocused();
    await expect(confirm).toBeDisabled();

    await page.keyboard.press("Space");
    await expect(statement).toBeChecked();
    await expect(confirm).toBeDisabled();

    await word.fill("prod");
    await expect(confirm).toBeDisabled();
    await word.press("Enter");
    await expect(dialog).toBeVisible();

    await word.fill("prod-eu");
    await expect(confirm).toBeEnabled();

    // Backing out forgets the progress.
    await page.keyboard.press("Escape");
    await expect(dialog).toBeHidden();
    await trigger.click();
    await expect(statement).not.toBeChecked();
    await expect(word).toHaveValue("");
    await expect(confirm).toBeDisabled();

    await statement.check();
    await word.fill("prod-eu");
    await word.press("Enter");
    await expect(dialog).toBeHidden();
    await expect(stage.getByRole("status")).toHaveText("Cluster prod-eu was deleted");
  });

  test("stays open with the reason when the action fails", async ({ page, example }) => {
    const stage = await example("dialog/failure");
    await stage.getByRole("button", { name: "Abort rollout" }).click();
    const dialog = page.getByRole("alertdialog", { name: "Abort rollout web-42?" });
    const confirm = dialog.getByRole("button", { name: "Abort rollout" });

    await expect(dialog.getByRole("alert")).toHaveCount(0);
    await confirm.click();
    await expect(dialog.getByRole("alert")).toHaveText("The controller did not answer. Nothing was changed.");
    await expect(dialog).toBeVisible();

    await confirm.click();
    await expect(dialog).toBeHidden();
    await expect(stage.getByRole("status")).toHaveText("Rollout web-42 was aborted");
  });
});

// The sweeps capture each example at rest, which for an overlay is only its
// trigger. These are the open states.
test.describe("Dialog, open", () => {
  for (const theme of THEMES) {
    test(`looks right: ${theme}`, async ({ page, example }) => {
      await page.setViewportSize({ width: 1280, height: 800 });

      let stage = await example("dialog/basic", { theme });
      await stage.getByRole("button", { name: "Review rollout" }).click();
      await expect(page).toHaveScreenshot(["dialog", `open-basic-${theme}-desktop.png`]);

      stage = await example("dialog/steps", { theme });
      await stage.getByRole("button", { name: "Delete cluster" }).click();
      await page.getByRole("checkbox").check();
      await page.getByRole("textbox").fill("prod-eu");
      await expect(page).toHaveScreenshot(["dialog", `open-steps-${theme}-desktop.png`]);

      stage = await example("dialog/failure", { theme });
      await stage.getByRole("button", { name: "Abort rollout" }).click();
      await page.getByRole("alertdialog").getByRole("button", { name: "Abort rollout" }).click();
      await expect(page).toHaveScreenshot(["dialog", `open-failure-${theme}-desktop.png`]);

      await page.setViewportSize({ width: 390, height: 844 });
      stage = await example("dialog/long", { theme });
      await stage.getByRole("button", { name: "Enrol this cluster" }).click();
      await expect(page).toHaveScreenshot(["dialog", `open-long-${theme}-phone.png`]);
    });
  }
});
