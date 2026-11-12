#!/usr/bin/env node
// Serves the prerendered site the way GitHub Pages does: `/a/b` resolves to
// `a/b.html`, unknown paths get `404.html`.
//
//   node serve.mjs [dist] [port]      (YEWI_SITE_BASE=/yewi/ for a sub-path)
import { createServer } from "node:http";
import { readFile, stat } from "node:fs/promises";
import { extname, join, normalize, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = fileURLToPath(new URL(".", import.meta.url));
const dist = resolve(process.argv[2] ?? join(here, "../site/dist"));
const port = Number(process.argv[3] ?? process.env.PORT ?? 4173);
const base = (process.env.YEWI_SITE_BASE ?? "/").replace(/\/$/, "");

const TYPES = {
  ".html": "text/html; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".json": "application/json",
  ".wasm": "application/wasm",
  ".woff2": "font/woff2",
  ".svg": "image/svg+xml",
  ".png": "image/png",
  ".txt": "text/plain; charset=utf-8",
};

const isFile = (path) => stat(path).then((s) => s.isFile(), () => false);

async function locate(pathname) {
  const path = normalize(decodeURIComponent(pathname)).replace(/^(\.\.[/\\])+/, "");
  for (const candidate of [path, `${path}.html`, join(path, "index.html")]) {
    const file = join(dist, candidate);
    if (file.startsWith(dist) && (await isFile(file))) return { file, status: 200 };
  }
  return { file: join(dist, "404.html"), status: 404 };
}

createServer(async (request, response) => {
  let { pathname } = new URL(request.url, "http://localhost");
  if (base && !pathname.startsWith(base)) {
    response.writeHead(302, { location: `${base}/` }).end();
    return;
  }
  pathname = pathname.slice(base.length) || "/";
  try {
    const { file, status } = await locate(pathname);
    const body = await readFile(file);
    response.writeHead(status, { "content-type": TYPES[extname(file)] ?? "application/octet-stream" }).end(body);
  } catch {
    response.writeHead(500).end("failed to read the site: build and prerender it first");
  }
}).listen(port, () => console.log(`serving ${dist} at http://localhost:${port}${base}/`));
