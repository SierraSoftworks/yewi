import { expect, test } from "../fixtures";

test.describe("Link", () => {
  test("is underlined in running text and only on hover when alone", async ({ example }) => {
    const stage = await example("link/basic");
    const inline = stage.getByRole("link", { name: "runbook" });
    const alone = stage.getByRole("link", { name: "Changelog" });

    await expect(inline).toHaveCSS("text-decoration-line", "underline");
    await expect(alone).toHaveCSS("text-decoration-line", "none");
    await alone.hover();
    await expect(alone).toHaveCSS("text-decoration-line", "underline");
  });

  test("external links open a new tab safely and say so", async ({ example }) => {
    const stage = await example("link/external");
    const link = stage.getByRole("link", { name: "Yew (opens in a new tab)" });

    await expect(link).toHaveAttribute("target", "_blank");
    await expect(link).toHaveAttribute("rel", /noopener/);
    await expect(link).toHaveAttribute("href", "https://yew.rs");
  });

  test("without href it is a button that works with mouse and keyboard", async ({ page, example }) => {
    const stage = await example("link/action");
    const status = stage.getByRole("status");
    const retry = stage.getByRole("button", { name: "Retry now" });

    await retry.click();
    await expect(status).toHaveText("Retried 1 times");

    await retry.focus();
    await page.keyboard.press("Enter");
    await page.keyboard.press("Space");
    await expect(status).toHaveText("Retried 3 times");
  });

  test("cannot be followed or clicked while disabled", async ({ page, example }) => {
    const stage = await example("link/action");
    const url = page.url();

    await expect(stage.getByRole("button", { name: "edit the probe" })).toBeDisabled();

    const link = stage.getByRole("link", { name: "Audit log" });
    await expect(link).toHaveAttribute("aria-disabled", "true");
    await link.click({ force: true });
    expect(page.url()).toBe(url);
    await expect(stage.getByRole("status")).toHaveText("Retried 0 times");
  });

  test("shows a visible focus indicator for keyboard users", async ({ page, example }) => {
    const stage = await example("link/basic");
    await page.keyboard.press("Tab");
    const focused = stage.getByRole("link", { name: "runbook" });
    await expect(focused).toBeFocused();
    await expect(focused).not.toHaveCSS("outline-style", "none");
  });
});

test.describe("Link to a route", () => {
  test("navigates in place and marks the current route", async ({ page, example }) => {
    const stage = await example("link/router");
    const status = stage.getByRole("status");
    const overview = stage.getByRole("link", { name: "Overview" });
    const hosts = stage.getByRole("link", { name: "Hosts" });
    const url = page.url();

    await expect(hosts).toHaveAttribute("href", "/hosts");
    await expect(overview).toHaveAttribute("aria-current", "page");

    await hosts.click();
    await expect(status).toHaveText("Showing: Hosts");
    await expect(hosts).toHaveAttribute("aria-current", "page");
    await expect(overview).not.toHaveAttribute("aria-current", "page");

    await stage.getByRole("link", { name: "Settings" }).focus();
    await page.keyboard.press("Enter");
    await expect(status).toHaveText("Showing: Settings");
    expect(page.url()).toBe(url);
  });

  test("leaves modified clicks to the browser", async ({ example }) => {
    const stage = await example("link/router");
    await stage.getByRole("link", { name: "Hosts" }).click({ modifiers: ["ControlOrMeta"] });
    await stage.getByRole("link", { name: "Settings" }).click({ modifiers: ["Shift"] });
    await expect(stage.getByRole("status")).toHaveText("Showing: Overview");
  });
});
