import type { Locator } from "@playwright/test";
import { expect, test } from "../fixtures";

/** The document an example shows under its form. */
async function document(stage: Locator) {
  return JSON.parse((await stage.getByRole("status").textContent()) ?? "null");
}

test.describe("SchemaForm", () => {
  test("typing, choosing and switching edit the document, and clearing an optional value removes it", async ({
    page,
    example,
  }) => {
    const stage = await example("schema-form/basic");

    await stage.getByLabel("Name").fill("Nightly digest");
    await stage.getByLabel("Feed URL").fill("https://example.com/feed.xml");
    await stage.getByLabel("Check every (minutes)").fill("15");
    await stage.getByRole("switch", { name: "Enabled" }).uncheck();
    await stage.getByRole("combobox", { name: "Priority" }).click();
    await page.getByRole("option", { name: "Urgent" }).click();
    await stage.getByRole("combobox", { name: "Labels" }).fill("news,");

    await expect.poll(() => document(stage)).toEqual({
      name: "Nightly digest",
      feed: "https://example.com/feed.xml",
      priority: 3,
      interval: 15,
      enabled: false,
      labels: ["news"],
    });

    // An emptied optional value leaves the document; a required one stays as "".
    await stage.getByLabel("Check every (minutes)").fill("");
    await stage.getByLabel("Name").fill("");
    await stage.getByRole("combobox", { name: "Priority" }).click();
    await page.getByRole("option", { name: "Not set" }).click();
    await expect.poll(() => document(stage)).toEqual({
      name: "",
      feed: "https://example.com/feed.xml",
      enabled: false,
      labels: ["news"],
    });
  });

  test("an optional group is added and removed with its switch, and a variant switch keeps shared values", async ({
    page,
    example,
  }) => {
    const stage = await example("schema-form/nested");
    const area = stage.getByRole("group", { name: "Area" });
    const include = area.getByRole("switch", { name: "Include Area" });

    await expect(include).toBeChecked();
    await area.getByLabel("Latitude").fill("48.85");
    await expect(area.getByLabel("Latitude")).toBeFocused();

    await area.getByRole("combobox", { name: "Kind" }).click();
    await page.getByRole("option", { name: "Bounding box" }).click();
    await expect(area.getByLabel("Radius (km)")).toHaveCount(0);
    await expect(area.getByLabel("South")).toHaveValue("48.85");
    expect((await document(stage)).area).toEqual({ kind: "bbox", lat: 48.85, lon: -0.12, north: 0, east: 0 });

    await include.focus();
    await page.keyboard.press("Space");
    await expect(include).not.toBeChecked();
    await expect(include).toBeFocused();
    await expect(area.getByRole("combobox")).toHaveCount(0);
    expect(await document(stage)).not.toHaveProperty("area");

    await page.keyboard.press("Space");
    await expect(area.getByRole("combobox", { name: "Kind" })).toBeVisible();
    expect((await document(stage)).area.kind).toBe("circle");
  });

  test("list items are added with the schema's defaults and removed by their own button", async ({ page, example }) => {
    const stage = await example("schema-form/nested");
    const stations = stage.getByRole("group", { name: "Stations" });
    const add = stations.getByRole("button", { name: "Add" });

    await add.focus();
    await page.keyboard.press("Enter");
    await expect(add).toBeFocused();
    const second = stations.getByRole("group", { name: "Station 2" });
    await expect(second.getByLabel("Port")).toHaveValue("30005");
    await second.getByLabel("Host").fill("rx2.example.net");

    await stations.getByRole("button", { name: "Remove Station 1" }).click();
    await expect(stations.getByRole("group", { name: /Station \d/ })).toHaveCount(1);
    expect((await document(stage)).stations).toEqual([{ host: "rx2.example.net", port: 30005 }]);
  });

  test("errors show against the field or group they address and clear when the server accepts", async ({ example }) => {
    const stage = await example("schema-form/errors");
    const host = stage.getByLabel("Host");

    await expect(host).toHaveAttribute("aria-invalid", "true");
    await expect(host).toHaveAccessibleDescription("A host is required");
    await expect(stage.getByRole("group", { name: "Maintenance window" })).toContainText(
      "The window must start before it ends",
    );
    await expect(stage.getByRole("alert")).toHaveCount(3);

    await host.fill("db.example.net");
    await stage.getByLabel("Port").fill("5432");
    await stage.getByLabel("Until (hour)").fill("23");
    await stage.getByRole("button", { name: "Save" }).click();
    await expect(stage.getByRole("alert")).toHaveCount(0);
    await expect(host).not.toHaveAttribute("aria-invalid", "true");
    await expect(host).toHaveAccessibleDescription("The server's public name.");
    await expect(stage.getByRole("status")).toHaveText("Saved");
  });

  test("a custom control edits the same document", async ({ page, example }) => {
    const stage = await example("schema-form/custom");

    await stage.getByRole("combobox", { name: "Account" }).click();
    await page.getByRole("option", { name: "Personal" }).click();
    await stage.getByRole("combobox", { name: "Filter" }).fill("tag:rust");
    await page.keyboard.press("Escape");
    await stage.getByLabel("Task title").fill("Read it");
    await expect.poll(() => document(stage)).toEqual({ account: "acct_1", filter: "tag:rust", title: "Read it" });
  });

  test("a JSON box reports only text that parses, and a disabled form cannot be edited", async ({ example }) => {
    const stage = await example("schema-form/fallback");
    const env = stage.getByLabel("Environment");

    await env.fill('{"LOG_LEVEL": "debug", ');
    await expect(env).toHaveAttribute("aria-invalid", "true");
    expect((await document(stage)).env).toEqual({ LOG_LEVEL: "info" });

    await env.fill('{"LOG_LEVEL": "debug"}');
    await expect(env).not.toHaveAttribute("aria-invalid", "true");
    await expect(env).toHaveValue('{"LOG_LEVEL": "debug"}');
    expect((await document(stage)).env).toEqual({ LOG_LEVEL: "debug" });

    await stage.getByRole("switch", { name: "Saving" }).check();
    for (const name of ["Image", "Replicas", "Environment"]) {
      await expect(stage.getByLabel(name)).toBeDisabled();
    }
  });
});
