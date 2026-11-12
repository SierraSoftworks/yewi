import { expect, test } from "../fixtures";

test.describe("Field", () => {
  test("clicking the label focuses the control", async ({ example }) => {
    const stage = await example("field/basic");
    await stage.getByText("Email", { exact: true }).click();
    await expect(stage.getByRole("textbox", { name: "Email" })).toBeFocused();
  });

  test("swaps the hint for an error and marks the control invalid", async ({ example }) => {
    const stage = await example("field/basic");
    const email = stage.getByRole("textbox", { name: "Email" });

    await expect(stage.getByRole("alert")).toContainText("Enter an address");
    await expect(email).toHaveAttribute("aria-invalid", "true");
    await expect(email).toHaveAccessibleDescription(/Enter an address/);
    await expect(stage.getByText("Deployment notices")).toHaveCount(0);

    await email.fill("ben@example.com");
    await expect(stage.getByRole("status")).toHaveText("Looks good");
    await expect(stage.getByRole("alert")).toHaveCount(0);
    await expect(email).not.toHaveAttribute("aria-invalid", "true");
    await expect(email).toHaveAccessibleDescription("Deployment notices are sent here.");
  });

  test("passes required and disabled to the control", async ({ example }) => {
    const stage = await example("field/states");
    await expect(stage.getByRole("textbox", { name: "Project name" })).toHaveJSProperty("required", true);
    await expect(stage.getByRole("textbox", { name: "Region" })).toBeDisabled();
  });

  test("keeps a hidden label as the accessible name", async ({ example }) => {
    const stage = await example("field/hidden-label");
    await expect(stage.getByRole("searchbox", { name: "Filter services" })).toBeVisible();
  });

  test("names a group and describes it with the hint", async ({ example }) => {
    const stage = await example("field/group");
    const group = stage.getByRole("group", { name: "Latency alert range (ms)" });
    await expect(group).toHaveAccessibleDescription("Alerts fire outside this range.");
    await expect(group.getByRole("spinbutton", { name: "Minimum" })).toHaveValue("50");
    await expect(group.getByRole("spinbutton", { name: "Maximum" })).toHaveValue("400");
  });

  test("wires up a custom control", async ({ example }) => {
    const stage = await example("field/custom-control");
    const slider = stage.getByRole("slider", { name: "Rollout speed" });
    await expect(slider).toHaveAccessibleDescription("Share of instances replaced per step.");
  });
});
