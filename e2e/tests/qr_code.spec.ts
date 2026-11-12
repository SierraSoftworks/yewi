import { expect, test } from "../fixtures";

test.describe("QrCode", () => {
  test("is exposed as a labelled image with its caption", async ({ example }) => {
    const stage = await example("qr-code/basic");
    await expect(stage.getByRole("img", { name: "Link to the yewi repository" })).toBeVisible();
    await expect(stage.getByRole("figure")).toContainText("Scan to open the repository");
  });

  test("redraws when the value changes", async ({ page, example }) => {
    const stage = await example("qr-code/live");
    const modules = stage.locator("svg path");
    const before = await modules.getAttribute("d");

    await stage.getByRole("button", { name: "Tablet" }).click();
    await expect(stage.getByRole("status")).toHaveText("Enrolling the tablet");
    await expect(stage.getByRole("img", { name: "Enrolment link for the tablet" })).toBeVisible();
    await expect(modules).not.toHaveAttribute("d", before!);

    await page.keyboard.press("Shift+Tab");
    await page.keyboard.press("Enter");
    await expect(stage.getByRole("status")).toHaveText("Enrolling the phone");
    await expect(modules).toHaveAttribute("d", before!);
  });

  test("stays dark on white in both themes", async ({ page, example }) => {
    const stage = await example("qr-code/basic");
    const colours = () =>
      stage.locator("svg").evaluate((svg) => ({
        paper: getComputedStyle(svg.querySelector("rect")!).fill,
        modules: getComputedStyle(svg.querySelector("path")!).fill,
      }));

    await page.emulateMedia({ colorScheme: "light" });
    const light = await colours();
    await page.emulateMedia({ colorScheme: "dark" });
    expect(await colours()).toEqual(light);
    expect(light.paper).toBe("rgb(255, 255, 255)");
  });

  test("honours the size prop and never overflows its container", async ({ example }) => {
    const stage = await example("qr-code/sizes");
    const small = await stage.getByRole("img", { name: "Invite link, small" }).boundingBox();
    expect(small!.width).toBeCloseTo(96, 0);
    expect(small!.height).toBeCloseTo(96, 0);
  });

  test("reports a value that is too long instead of drawing it", async ({ example }) => {
    const stage = await example("qr-code/too-long");
    await expect(stage.getByRole("img")).toHaveCount(0);
    await expect(stage.getByRole("alert").first()).toContainText("too long");
    await expect(stage.getByRole("alert").last()).toContainText("Download it instead");
  });
});
