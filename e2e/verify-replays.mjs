#!/usr/bin/env node
// Checks the review gallery's recordings: plays each one in the packed viewer
// and compares what the example reads afterwards with what it read at the end
// of the spec that was recorded. A mismatch means the replay does not show
// what the spec did (see the limits listed in CONTRIBUTING.md).
//
//   node verify-replays.mjs [viewer.html] [example prefix]
import { writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { chromium } from "@playwright/test";

const here = dirname(fileURLToPath(import.meta.url));
const file = resolve(process.argv[2] ?? join(here, "gallery/viewer.html"));
const only = process.argv[3];

const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 1280, height: 720 }, timezoneId: "UTC" });
const errors = [];
page.on("pageerror", (error) => errors.push(error.message.split("\n")[0]));
await page.goto(pathToFileURL(file).href);
await page.locator(".gallery-example").first().waitFor();

const recordings = await page.evaluate(() => window.__yiReplay?.recordings.map(({ example, title, end }) => ({ example, title, end })) ?? []);
const text = (value) => (value ?? "").replace(/\s+/g, " ").trim();
let same = 0;
let unchecked = 0;
const different = [];
const inexact = [];
for (const [index, recording] of recordings.entries()) {
  if (only && !recording.example.startsWith(only)) continue;
  await page.evaluate((slug) => {
    document.querySelectorAll("[data-gallery-reset]").forEach((button) => button.click());
    document.querySelector(`[data-gallery-link="${slug}"]`).click();
  }, recording.example.split("/")[0]);
  await page.locator(`[data-gallery-replays="${recording.example}"] button`).first().waitFor();
  const end = await page.evaluate((i) => window.__yiReplay.play(i, true), index);
  if (recording.end === undefined) unchecked += 1;
  else if (text(end) === text(recording.end)) same += 1;
  else {
    inexact.push(`${recording.example}: ${recording.title}`);
    different.push(`${recording.example}: ${recording.title}\n    spec:   ${text(recording.end).slice(0, 200)}\n    replay: ${text(end).slice(0, 200)}`);
  }
}
await browser.close();
// The next `viewer.mjs` run marks these as approximate.
if (!only) writeFileSync(join(here, "replay/inexact.json"), `${JSON.stringify(inexact.sort(), null, 2)}\n`);

for (const entry of different) console.log(`differs  ${entry}`);
for (const error of new Set(errors)) console.log(`error    ${error}`);
console.log(`replays: ${same} match, ${different.length} differ, ${unchecked} not checked (no end state recorded)`);
process.exit(different.length || errors.length ? 1 : 0);
