import { expect, test } from "../fixtures";

// A 1x1 PNG, standing in for a picture served by Libravatar.
const PIXEL = Buffer.from(
  "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==",
  "base64",
);
const LIBRAVATAR = "https://seccdn.libravatar.org/**";

test.describe("Avatar", () => {
  test("falls back to initials when the picture fails, and recovers", async ({ example }) => {
    const stage = await example("avatar/image");
    const avatars = stage.getByRole("img", { name: "Ada Lovelace" });

    await expect(avatars).toHaveCount(3);
    await expect(stage.locator("img")).toHaveCount(3);

    await stage.getByRole("button", { name: "Break the picture" }).click();
    await expect(stage.getByRole("status")).toHaveText("Showing initials");
    await expect(stage.locator("img")).toHaveCount(0);
    await expect(avatars).toHaveCount(3);
    await expect(avatars.first()).toHaveText("AL");

    await stage.getByRole("button", { name: "Restore the picture" }).click();
    await expect(stage.locator("img")).toHaveCount(3);
  });

  test("loads the hashed picture for an email, sending only the hash", async ({ page, example }) => {
    const requested: string[] = [];
    await page.route(LIBRAVATAR, (route) => {
      requested.push(route.request().url());
      return route.fulfill({ status: 200, contentType: "image/png", body: PIXEL });
    });
    const stage = await example("avatar/hashed");
    await expect(stage.locator("img")).toHaveCount(0);

    await stage.getByRole("button", { name: "Look up the picture" }).click();
    const picture = stage.locator("img").first();
    await expect(picture).toHaveAttribute("src", /libravatar\.org\/avatar\/[0-9a-f]{64}\?/);
    await expect.poll(() => picture.evaluate((img: HTMLImageElement) => img.naturalWidth)).toBeGreaterThan(0);
    expect(requested.length).toBeGreaterThan(0);
    expect(requested.join(" ")).not.toContain("ada");
  });

  test("shows initials when neither Libravatar nor Gravatar has a picture", async ({ page, example }) => {
    await page.route(LIBRAVATAR, (route) => route.fulfill({ status: 404, body: "" }));
    const stage = await example("avatar/hashed");

    await stage.getByRole("button", { name: "Look up the picture" }).click();
    await expect(stage.getByRole("img", { name: "Ada Lovelace" })).toHaveText("AL");
    await expect(stage.locator("img")).toHaveCount(0);
  });
});

test.describe("AvatarGroup", () => {
  test("names the group and counts the people it does not show", async ({ example }) => {
    const stage = await example("avatar/group");
    const contributors = stage.getByRole("group", { name: "Contributors" });

    await expect(contributors.getByRole("img")).toHaveCount(5);
    const more = contributors.getByRole("img", { name: "2 more" });
    await expect(more).toHaveText("+2");
    await more.hover();
    await expect(more).toHaveAttribute("title", /Edsger Dijkstra/);
    await expect(stage.getByRole("group", { name: "Watchers" }).getByRole("img", { name: "125 more" })).toBeVisible();
  });
});

test.describe("UserChip", () => {
  test("as a button, activates with the mouse and the keyboard", async ({ page, example }) => {
    const stage = await example("avatar/interactive");
    const status = stage.getByRole("status");
    await expect(status).toHaveText("Unassigned");

    await stage.getByRole("button", { name: "Grace Hopper" }).click();
    await expect(status).toHaveText("Assigned to Grace Hopper");

    await page.keyboard.press("Tab");
    await expect(stage.getByRole("button", { name: "Alan Turing" })).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(status).toHaveText("Assigned to Alan Turing");

    await page.keyboard.press("Shift+Tab");
    await page.keyboard.press("Shift+Tab");
    await page.keyboard.press("Space");
    await expect(status).toHaveText("Assigned to Ada Lovelace");
  });

  test("as a link, is a real link with a visible focus indicator", async ({ page, example }) => {
    const stage = await example("avatar/interactive");
    const link = stage.getByRole("link", { name: "Barbara Liskov" });
    await expect(link).toHaveAttribute("href", "#barbara-liskov");

    await stage.getByRole("button", { name: "Alan Turing" }).focus();
    await page.keyboard.press("Tab");
    await expect(link).toBeFocused();
    await expect(link).not.toHaveCSS("outline-style", "none");
  });
});
