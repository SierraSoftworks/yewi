import { expect, test } from "../fixtures";

test.describe("Sidebar", () => {
  test("moves through destinations by keyboard and marks the chosen one", async ({ page, example }) => {
    const stage = await example("sidebar/basic");
    const nav = stage.getByRole("navigation", { name: "Sections" });

    await nav.getByRole("button", { name: "Overview" }).focus();
    await page.keyboard.press("Tab");
    await page.keyboard.press("Tab");
    await page.keyboard.press("Enter");

    await expect(stage.getByRole("status")).toHaveText("Showing Hosts");
    await expect(nav.locator("[aria-current]")).toHaveText("Hosts");
  });

  test("groups name their destinations", async ({ example }) => {
    const stage = await example("sidebar/groups");
    const clients = stage.getByRole("group", { name: "Clients" });
    await expect(clients.getByRole("link")).toHaveText(["Users", "Devices"]);
    await expect(stage.getByRole("link", { name: "Dashboard" })).toHaveAttribute("aria-current", "page");
  });

  test("an external item opens a new tab and says so", async ({ example }) => {
    const stage = await example("sidebar/nested");
    const docs = stage.getByRole("link", { name: /Documentation.*new tab/ });
    await expect(docs).toHaveAttribute("target", "_blank");
    await expect(stage.getByRole("link", { name: "Website" })).toHaveAttribute("aria-current", "page");
  });
});
