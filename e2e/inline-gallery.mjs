// Inlines a gallery's images and videos as data URIs so the page works as a single file
// (for viewers that cannot load sibling files). Usage: node inline-gallery.mjs <gallery dir> [out file]
import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { join, extname, resolve } from "node:path";

const dir = resolve(process.argv[2] ?? "gallery");
const out = resolve(process.argv[3] ?? join(dir, "gallery.html"));
const types = { ".png": "image/png", ".webm": "video/webm", ".webp": "image/webp", ".svg": "image/svg+xml" };

let missing = 0;
const html = readFileSync(join(dir, "index.html"), "utf8").replace(/\b(src|poster)="([^"#:]+)"/g, (whole, attr, ref) => {
  const file = join(dir, decodeURIComponent(ref));
  const type = types[extname(file).toLowerCase()];
  if (!type || !existsSync(file)) {
    missing += 1;
    return whole;
  }
  return `${attr}="data:${type};base64,${readFileSync(file).toString("base64")}"`;
});

writeFileSync(out, html);
console.log(`wrote ${out} (${(html.length / 1048576).toFixed(1)} MB, ${missing} unresolved references)`);
