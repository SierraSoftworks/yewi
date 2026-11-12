import { defineConfig, devices } from "@playwright/test";

const port = Number(process.env.PORT ?? 4173);

// Chromium is the reference engine and the only one with visual baselines.
// Set YEWI_E2E_BROWSERS=all to also run behaviour specs in Firefox and WebKit.
const allBrowsers = process.env.YEWI_E2E_BROWSERS === "all";

// Specs that sweep every example. Everything else is a behaviour spec.
const SWEEPS = /sweeps\/.*\.spec\.ts/;

export default defineConfig({
  testDir: "tests",
  outputDir: "test-results",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 1 : 0,
  reporter: [
    ["list"],
    ["html", { outputFolder: "report", open: "never" }],
    ["json", { outputFile: "test-results/report.json" }],
  ],
  snapshotPathTemplate: "baselines/{arg}{ext}",
  expect: {
    // An update run (`cargo xtask e2e --update-snapshots`) allows no differing pixels, so any
    // visible change is recorded. Sub-perceptual rendering noise is still ignored per pixel.
    toHaveScreenshot: {
      maxDiffPixelRatio: process.env.YEWI_E2E_UPDATE ? 0 : 0.002,
      animations: "disabled",
      caret: "hide",
    },
  },
  use: {
    baseURL: `http://localhost:${port}`,
    trace: process.env.YEWI_E2E_TRACE === "on" ? "on" : "retain-on-failure",
  },
  projects: [
    {
      // Screenshots, accessibility and hydration for every example.
      name: "examples",
      testMatch: SWEEPS,
      use: { ...devices["Desktop Chrome"] },
    },
    {
      // Interaction specs, recorded for human review.
      name: "behaviour",
      testIgnore: SWEEPS,
      use: { ...devices["Desktop Chrome"], video: { mode: "on", size: { width: 1280, height: 720 } } },
    },
    ...(allBrowsers
      ? [
          { name: "behaviour-firefox", testIgnore: SWEEPS, use: { ...devices["Desktop Firefox"] } },
          { name: "behaviour-webkit", testIgnore: SWEEPS, use: { ...devices["Desktop Safari"] } },
        ]
      : []),
  ],
  webServer: {
    command: `node serve.mjs ../site/dist ${port}`,
    port,
    reuseExistingServer: !process.env.CI,
  },
});
