#!/usr/bin/env node
// Builds e2e/gallery/: one page for a human (or a review agent) to judge a
// change by eye. It shows every example in both themes at both widths, marks
// what changed against a git ref with before and after, and lists the
// recordings of the behaviour specs from the last run.
//
//   node gallery.mjs [base-ref]     (default: origin/main, when it exists)
import { execFileSync } from "node:child_process";
import { cpSync, existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const repo = join(here, "..");
const out = join(here, "gallery");
const git = (...args) => execFileSync("git", args, { cwd: repo, encoding: "buffer", stdio: ["ignore", "pipe", "ignore"] });
const escape = (text) => String(text).replace(/[&<>"]/g, (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" })[c]);

rmSync(out, { recursive: true, force: true });
mkdirSync(out, { recursive: true });

const examples = JSON.parse(readFileSync(join(repo, "site/dist/examples.json"), "utf8"));
if (existsSync(join(here, "baselines"))) cpSync(join(here, "baselines"), join(out, "baselines"), { recursive: true });

// ---- What changed against the base ref ---------------------------------------
let base = process.argv[2];
if (!base) {
  try {
    git("rev-parse", "--verify", "--quiet", "origin/main");
    base = "origin/main";
  } catch {}
}
const changed = new Map(); // baseline path relative to e2e/baselines -> "added" | "changed"
if (base) {
  try {
    const status = git("diff", "--name-status", base, "--", "e2e/baselines").toString();
    for (const line of status.split("\n").filter(Boolean)) {
      const [code, path] = line.split("\t");
      const name = relative("e2e/baselines", path);
      if (code === "M") {
        const before = join(out, "before", name);
        mkdirSync(dirname(before), { recursive: true });
        writeFileSync(before, git("show", `${base}:${path}`));
        changed.set(name, "changed");
      } else if (code === "A") {
        changed.set(name, "added");
      }
    }
  } catch (error) {
    console.warn(`could not diff against ${base}: ${error.message.split("\n")[0]}`);
    base = undefined;
  }
}

// ---- Recordings from the last behaviour run ----------------------------------
const recordings = [];
const report = join(here, "test-results/report.json");
if (existsSync(report)) {
  const walk = (suite, trail) => {
    for (const spec of suite.specs ?? []) {
      for (const test of spec.tests ?? []) {
        const result = test.results?.at(-1);
        const video = result?.attachments?.find((a) => a.name === "video" && a.path && existsSync(a.path));
        if (!video) continue;
        const file = `videos/${recordings.length}.webm`;
        mkdirSync(join(out, "videos"), { recursive: true });
        cpSync(video.path, join(out, file));
        recordings.push({ title: [...trail, spec.title].join(" › "), file, status: result.status, project: test.projectName });
      }
    }
    for (const child of suite.suites ?? []) walk(child, child.title && !child.title.endsWith(".ts") ? [...trail, child.title] : trail);
  };
  for (const suite of JSON.parse(readFileSync(report, "utf8")).suites ?? []) walk(suite, []);
}

// ---- Page ---------------------------------------------------------------------
const shot = (id, theme, device) => {
  const name = `${id}-${theme}-${device}.png`;
  if (!existsSync(join(out, "baselines", name))) return `<figure class="missing">no ${theme} ${device} baseline</figure>`;
  const state = changed.get(name);
  const before = state === "changed" ? `<img src="before/${escape(name)}" alt="before" title="before" loading="lazy">` : "";
  return `<figure data-theme="${theme}" class="${state ?? ""}">${before}<img src="baselines/${escape(name)}" alt="${escape(`${id}, ${theme}, ${device}`)}" loading="lazy"><figcaption>${theme} · ${device}${state ? ` · <b>${state}</b>` : ""}</figcaption></figure>`;
};

const sections = examples.map((e) => {
  const id = `${e.page}/${e.example}`;
  const touched = ["light", "dark"].some((t) => ["desktop", "phone"].some((d) => changed.has(`${id}-${t}-${d}.png`)));
  return `<section id="${escape(id)}" class="${touched ? "touched" : ""}">
  <h2><a href="#${escape(id)}">${escape(e.title)}</a> <code>${escape(id)}</code></h2>
  <div class="row">${shot(id, "light", "desktop")}${shot(id, "dark", "desktop")}</div>
  <div class="row">${shot(id, "light", "phone")}${shot(id, "dark", "phone")}</div>
</section>`;
});

const videos = recordings.map(
  (r) => `<figure class="video ${r.status}"><video src="${r.file}" controls preload="none" muted></video><figcaption>${escape(r.title)} · ${r.status}</figcaption></figure>`,
);

writeFileSync(
  join(out, "index.html"),
  `<!doctype html>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>yewi review gallery</title>
<style>
  :root { color-scheme: light dark; font: 14px/1.5 system-ui, sans-serif; }
  body { margin: 0 auto; padding: 24px; max-width: 1500px; }
  h1 { font-weight: 400; } h2 { font-size: 16px; font-weight: 600; margin: 32px 0 8px; } h2 a { color: inherit; text-decoration: none; }
  code { font-size: 12px; opacity: .65; margin-left: 8px; }
  .row { display: flex; gap: 12px; flex-wrap: wrap; align-items: flex-start; margin-bottom: 12px; }
  figure { margin: 0; padding: 8px; border-radius: 8px; border: 1px solid #8884; display: flex; gap: 8px; flex-wrap: wrap; align-items: flex-start; }
  figure[data-theme=light] { background: #fff; color: #222; } figure[data-theme=dark] { background: #202228; color: #ddd; }
  figure img { max-width: 100%; height: auto; display: block; }
  figure.changed { border-color: #d98c00; } figure.added { border-color: #2a9d5c; }
  figure.changed img:first-child { outline: 1px dashed #d98c00; opacity: .85; }
  figcaption { flex-basis: 100%; font-size: 12px; opacity: .75; }
  .missing { font-size: 12px; opacity: .6; }
  .videos { display: grid; grid-template-columns: repeat(auto-fill, minmax(380px, 1fr)); gap: 12px; }
  .video video { width: 100%; } .video.failed { border-color: #c4372e; }
  body.only-changed section:not(.touched) { display: none; }
  label { user-select: none; }
</style>
<h1>yewi review gallery</h1>
<p>${examples.length} examples, ${recordings.length} recordings. ${
    base ? `${changed.size} screenshots differ from <code>${escape(base)}</code>; changed ones show before (dashed) then after.` : "No base ref to compare against."
  }</p>
<p><label><input type="checkbox" onchange="document.body.classList.toggle('only-changed', this.checked)"> Only examples that changed</label></p>
${sections.join("\n")}
<h1>Behaviour recordings</h1>
<div class="videos">${videos.join("\n") || "<p>No recordings: run the behaviour specs first.</p>"}</div>
`,
);
console.log(`gallery: ${examples.length} examples, ${changed.size} changed, ${recordings.length} recordings -> ${relative(repo, join(out, "index.html"))}`);
