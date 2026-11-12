#!/usr/bin/env node
// Packs the built reference site into one HTML file: the review gallery.
//
// The file runs the real components (the site's wasm, compressed and inlined)
// behind a menu of every component, with theme, accent and stage width
// controls (`site/src/gallery.rs`). Behaviour specs from the last test run are
// included as event recordings that replay on the live examples
// (`replay/recorder.js`, `replay/player.js`). No screenshots, no videos.
//
//   node viewer.mjs [out.html]      (default: gallery/viewer.html)
//
// Needs `cargo xtask site` (or `e2e`) to have built `site/dist`.
import { execFileSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { gzipSync } from "node:zlib";

const here = dirname(fileURLToPath(import.meta.url));
const repo = join(here, "..");
const dist = join(repo, "site/dist");
const out = resolve(process.argv[2] ?? join(here, "gallery/viewer.html"));

if (!existsSync(join(dist, "yewi.css"))) {
  console.error("site/dist is missing or not prerendered: run `cargo xtask site` first");
  process.exit(1);
}

const walk = (dir) => readdirSync(dir).flatMap((name) => (statSync(join(dir, name)).isDirectory() ? walk(join(dir, name)) : [join(dir, name)]));
const files = walk(dist).map((file) => relative(dist, file).replaceAll("\\", "/"));
const one = (pattern, what) => {
  const found = files.filter((file) => pattern.test(file));
  if (found.length !== 1) throw new Error(`expected one ${what} in site/dist, found ${found.length}`);
  return found[0];
};
const glue = one(/^[^/]+\.js$/, "wasm-bindgen module");
const wasm = one(/^[^/]+_bg\.wasm$/, "wasm binary");
const siteCss = one(/^(?!yewi\.css$)[^/]+\.css$/, "site stylesheet");

// ---- Styles ---------------------------------------------------------------------
// A replay cannot make an element `:hover`, so those selectors also answer to
// attributes the player sets. Specificity is unchanged.
const replayable = (css) =>
  css
    .replace(/(?<!hover\s*):hover(?![\w-])/g, ":is(:hover,[data-yi-replay-hover])")
    .replace(/(?<![\w-]):active(?![\w-])/g, ":is(:active,[data-yi-replay-active])")
    .replace(/:focus-visible(?![\w-])/g, ":is(:focus-visible,[data-yi-replay-focus])");
const style = (css) => css.replaceAll("</style", "<\\/style");

const fonts = readFileSync(join(dist, "fonts/fonts.css"), "utf8").replace(/url\((["']?)([^)"']+\.woff2)\1\)/g, (_, _q, ref) => {
  return `url(data:font/woff2;base64,${readFileSync(join(dist, "fonts", ref)).toString("base64")})`;
});

// ---- Code -------------------------------------------------------------------------
// The wasm-bindgen module and the snippets it imports are joined into one inline
// module script, so starting the gallery loads nothing by URL. Scripts a snippet
// imports lazily (`new URL("./vendor/x.js", HERE)`) stay separate, gzipped, and
// are handed over as `data:` URLs through `globalThis.__yiVendor`.
const vendor = new Set();
const modules = new Map(); // dist path -> { name, code }
const posix = (from, ref) => {
  const parts = from.split("/").slice(0, -1);
  for (const part of ref.split("/")) {
    if (part === "..") parts.pop();
    else if (part !== ".") parts.push(part);
  }
  return parts.join("/");
};
const lazy = (path, text) =>
  text.replace(/new URL\((["'])(\.{1,2}\/[^"']+)\1,\s*[\w.]+\)(\.href)?/g, (whole, _quote, ref) => {
    const target = posix(path, ref);
    if (!files.includes(target)) return whole;
    vendor.add(target);
    return `globalThis.__yiVendor[${JSON.stringify(target)}]`;
  });
// A snippet becomes a function scope that returns its exports.
const snippet = (path) => {
  if (!modules.has(path)) {
    const names = [];
    const code = lazy(path, readFileSync(join(dist, path), "utf8")).replace(/^export\s+((?:async\s+)?function\*?|const|let|class)\s+([\w$]+)/gm, (_, kind, name) => {
      names.push(name);
      return `${kind} ${name}`;
    });
    if (/^\s*(export|import)\s/m.test(code)) throw new Error(`${path}: only top-level \`export function|const|let|class\` is supported in a snippet`);
    const name = `yiModule${modules.size}`;
    modules.set(path, { name, code: `const ${name} = (() => {\n${code}\nreturn { ${names.join(", ")} };\n})();` });
  }
  return modules.get(path).name;
};
const main = readFileSync(join(dist, glue), "utf8")
  .replace(/^import\s+(.+?)\s+from\s+(["'])(.+?)\2;?$/gm, (_, what, _quote, ref) => {
    const name = snippet(posix(glue, ref));
    const star = /^\*\s+as\s+([\w$]+)$/.exec(what);
    return star ? `const ${star[1]} = ${name};` : `const ${what.replace(/\s+as\s+/g, ": ")} = ${name};`;
  })
  .replace(/^export\s*\{[^}]*\};?$/gm, "");
if (/^\s*(export|import)\s/m.test(main)) throw new Error(`${glue}: an import or export form viewer.mjs does not know`);
const code = [...modules.values()].map((module) => module.code).join("\n") + "\n" + main;

const assets = {};
for (const file of [wasm, ...vendor]) assets[file] = gzipSync(readFileSync(join(dist, file)), { level: 9 }).toString("base64");

// ---- Recordings -------------------------------------------------------------------
const recordings = [];
// Recordings `verify-replays.mjs` found to end differently from their spec.
const inexactFile = join(here, "replay/inexact.json");
const inexact = new Set(existsSync(inexactFile) ? JSON.parse(readFileSync(inexactFile, "utf8")) : []);
const report = join(here, "test-results/report.json");
if (existsSync(report)) {
  const visit = (suite) => {
    for (const spec of suite.specs ?? []) {
      for (const test of spec.tests ?? []) {
        const result = test.results?.at(-1);
        const replay = result?.status === "passed" && result.attachments?.find((a) => a.name === "replay" && a.path && existsSync(a.path));
        if (!replay) continue;
        const segments = JSON.parse(readFileSync(replay.path, "utf8"));
        segments.forEach((segment, index) => {
          const title = segments.length > 1 ? `${spec.title} (${index + 1}/${segments.length})` : spec.title;
          recordings.push({
            example: segment.example,
            title,
            events: segment.events,
            end: segment.end,
            inexact: inexact.has(`${segment.example}: ${title}`) || undefined,
          });
        });
      }
    }
    (suite.suites ?? []).forEach(visit);
  };
  (JSON.parse(readFileSync(report, "utf8")).suites ?? []).forEach(visit);
}

// ---- Page ---------------------------------------------------------------------------
let build = "";
try {
  const git = (...args) => execFileSync("git", args, { cwd: repo, encoding: "utf8", stdio: ["ignore", "pipe", "ignore"] }).trim();
  build = `${git("rev-parse", "--short", "HEAD")}${git("status", "--porcelain", "--untracked-files=no") ? "+" : ""} · ${git("log", "-1", "--format=%cs")}`;
} catch {}

const json = (value) => JSON.stringify(value).replaceAll("<", "\\u003c");
const script = (name) => readFileSync(join(here, "replay", name), "utf8").replaceAll("</script", "<\\/script");

const html = `<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>yewi review gallery</title>
<script>(function(){try{var t=localStorage.getItem("yewi-theme");if(t==="light"||t==="dark")document.documentElement.setAttribute("data-theme",t)}catch(e){}})()</script>
<style>${style(fonts)}</style>
<style id="yewi-styles">${style(replayable(readFileSync(join(dist, "yewi.css"), "utf8")))}</style>
<style>${style(replayable(readFileSync(join(dist, siteCss), "utf8")))}</style>
</head>
<body>
<div id="app"><p id="gallery-boot" style="padding:2rem;font:14px/1.5 system-ui,sans-serif">Unpacking the gallery. If this message stays, this viewer does not run scripts: download the file and open it in a browser.</p></div>
<script type="application/json" id="yi-replays">${json(recordings)}</script>
<script type="application/json" id="yi-assets">${json(assets)}</script>
<script type="application/json" id="yi-boot">${json({ wasm, build, vendor: [...vendor] })}</script>
<script>
addEventListener("unhandledrejection", (event) => yiFailed(event.reason));
addEventListener("error", (event) => yiFailed(event.error ?? event.message));
function yiFailed(error) {
  const el = document.getElementById("gallery-boot");
  if (el) el.textContent = "The gallery could not start here: " + (error && error.message || error) + ". It needs a viewer that runs scripts and WebAssembly; the downloaded file opens in any current browser.";
}
</script>
<script type="module">${script("boot.js").replace("/*__GLUE__*/", () => code.replaceAll("</script", "<\\/script"))}</script>
<script>${script("player.js")}</script>
</body>
</html>
`;

mkdirSync(dirname(out), { recursive: true });
writeFileSync(out, html);
console.log(`viewer: ${(html.length / 1048576).toFixed(1)} MB, ${recordings.length} recordings -> ${relative(process.cwd(), out)}`);
