import { expect, test } from "../fixtures";

test.describe("AppBar", () => {
  test("lays out brand, content and actions from start to end", async ({ example }) => {
    const stage = await example("app-bar/slots");
    const x = async (name: string | RegExp, role: "button" | "img" = "button") =>
      (await stage.getByRole(role, { name }).first().boundingBox())!.x;

    const brand = (await stage.getByText("Grey", { exact: true }).boundingBox())!.x;
    expect(brand).toBeLessThan(await x("Dashboards"));
    expect(await x("Settings")).toBeLessThan(await x("Notifications"));
    await expect(stage.getByRole("button", { name: "Navigation" })).toHaveCount(0);
  });

  test("its content is reachable and operable by keyboard", async ({ page, example }) => {
    const stage = await example("app-bar/slots");
    await stage.getByRole("button", { name: "Dashboards" }).focus();
    await page.keyboard.press("Tab");
    await page.keyboard.press("Enter");
    await expect(stage.getByRole("status")).toHaveText("Showing Alerts");
    await expect(stage.getByRole("button", { name: "Alerts" })).toHaveAttribute("aria-pressed", "true");
  });

  test("the brand links home when given an href", async ({ example }) => {
    const stage = await example("app-bar/basic");
    await expect(stage.getByRole("link", { name: "Buckle overview" })).toHaveAttribute("href", "#");
  });
});
