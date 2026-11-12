import type { Locator, Page } from "@playwright/test";
import { expect, test } from "../fixtures";

interface Fake {
  name: string;
  type: string;
  bytes?: number;
}

/** Fires `type` on `zone` as a drag carrying `files` would. */
async function drag(page: Page, zone: Locator, type: "dragover" | "dragleave" | "drop", files: Fake[]) {
  const dataTransfer = await page.evaluateHandle((fakes) => {
    const transfer = new DataTransfer();
    for (const fake of fakes) {
      transfer.items.add(new File([new Uint8Array(fake.bytes ?? 4)], fake.name, { type: fake.type }));
    }
    return transfer;
  }, files);
  await zone.dispatchEvent(type, { dataTransfer });
}

const pack: Fake = { name: "field-team.zip", type: "application/zip" };

test.describe("FileDrop", () => {
  test("opens the picker from the keyboard and reports the chosen file", async ({ page, example }) => {
    const stage = await example("file-drop/basic");
    const input = stage.getByLabel("Drop a package here, or choose one");
    await expect(input).toHaveAccessibleDescription(/One at a time/);

    await page.keyboard.press("Tab");
    await expect(input).toBeFocused();
    await expect(stage.locator("label")).toHaveCSS("outline-style", "solid");

    const chooser = page.waitForEvent("filechooser");
    await page.keyboard.press("Enter");
    await (await chooser).setFiles({ name: "atak.zip", mimeType: "application/zip", buffer: Buffer.from("zip!") });
    await expect(stage.getByRole("status")).toHaveText("Chosen: atak.zip (4 bytes)");
  });

  test("opens the picker on a click anywhere in the zone", async ({ page, example }) => {
    const stage = await example("file-drop/basic");
    const chooser = page.waitForEvent("filechooser");
    await stage.locator("label").click({ position: { x: 12, y: 12 } });
    expect((await chooser).isMultiple()).toBe(false);
  });

  test("highlights while files are dragged over and takes the drop", async ({ page, example }) => {
    const stage = await example("file-drop/basic");
    const zone = stage.locator("label");
    const border = () => zone.evaluate((element) => getComputedStyle(element).borderColor);
    const resting = await border();

    await drag(page, zone, "dragover", [pack]);
    await expect.poll(border).not.toBe(resting);
    await drag(page, zone, "dragleave", [pack]);
    await expect.poll(border).toBe(resting);

    await drag(page, zone, "dragover", [pack]);
    await drag(page, zone, "drop", [pack]);
    await expect(stage.getByRole("status")).toHaveText("Chosen: field-team.zip (4 bytes)");
    await expect.poll(border).toBe(resting);
  });

  test("choosing the same file twice reports it twice", async ({ example }) => {
    const stage = await example("file-drop/multiple");
    const input = stage.getByLabel("Drop map tiles here, or choose them");
    const tile = { name: "tile.png", mimeType: "image/png", buffer: Buffer.from("png") };

    await input.setInputFiles(tile);
    await input.setInputFiles(tile);
    await expect(stage.getByRole("status")).toHaveText("2 added");
  });

  test("turns away dropped files that break the limits, with the reason", async ({ page, example }) => {
    const stage = await example("file-drop/multiple");
    await drag(page, stage.locator("label"), "drop", [
      { name: "north.png", type: "image/png" },
      { name: "notes.txt", type: "text/plain" },
      { name: "huge.png", type: "image/png", bytes: 1024 * 1024 + 1 },
      { name: "south.jpg", type: "image/jpeg" },
    ]);

    await expect(stage.getByRole("list", { name: "Added images" }).getByRole("listitem")).toHaveText([
      "north.png",
      "south.jpg",
    ]);
    await expect(stage.getByRole("alert")).toHaveText(
      "Skipped: notes.txt is not an image, huge.png is larger than 1 MB.",
    );
  });

  test("joins a field: named by its label, and its error clears once a file is chosen", async ({ example }) => {
    const stage = await example("file-drop/field");
    const input = stage.getByLabel(/^Certificate/);
    await expect(input).toHaveAttribute("type", "file");

    await stage.getByRole("button", { name: "Import" }).click();
    await expect(stage.getByRole("alert")).toHaveText("Choose a certificate to import.");
    await expect(input).toHaveAttribute("aria-invalid", "true");

    await input.setInputFiles({ name: "team.pem", mimeType: "application/x-pem-file", buffer: Buffer.from("pem") });
    await expect(input).not.toHaveAttribute("aria-invalid", "true");
    await expect(input).toHaveAccessibleDescription(/Chosen: team\.pem/);
  });

  test("takes nothing while busy, and again once the upload finishes", async ({ page, example }) => {
    const stage = await example("file-drop/upload");
    const zone = stage.locator("label");

    await drag(page, zone, "drop", [pack]);
    await expect(zone).toContainText("Uploading field-team.zip…");
    await expect(zone.locator("input")).toBeDisabled();

    await drag(page, zone, "drop", [{ name: "other.zip", type: "application/zip" }]);
    await expect(zone).toContainText("Uploading field-team.zip…");

    await stage.getByRole("button", { name: "Finish upload" }).click();
    await expect(stage.getByRole("status")).toHaveText("1 uploaded");
    await expect(zone.locator("input")).toBeEnabled();
  });

  test("ignores drags that carry no files", async ({ page, example }) => {
    const stage = await example("file-drop/basic");
    const zone = stage.locator("label");
    const border = () => zone.evaluate((element) => getComputedStyle(element).borderColor);
    const resting = await border();

    const dataTransfer = await page.evaluateHandle(() => {
      const transfer = new DataTransfer();
      transfer.setData("text/plain", "not a file");
      return transfer;
    });
    await zone.dispatchEvent("dragover", { dataTransfer });
    await page.waitForTimeout(100);
    expect(await border()).toBe(resting);
  });
});
