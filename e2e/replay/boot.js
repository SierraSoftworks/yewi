// Starts the packed review gallery. `viewer.mjs` inlines this file as one
// module script, with the site's wasm-bindgen glue pasted in at the marker
// below. Nothing is loaded by URL except the lazily imported vendor scripts
// (the map engine), which become `data:` URLs: file viewers commonly refuse
// `blob:` modules, and an inline module needs no loading at all.
//
// Data it reads from the page: `yi-assets` (gzip, base64) and `yi-boot` (file
// names, build stamp).
const yiBoot = JSON.parse(document.getElementById("yi-boot").textContent);
const yiAssets = JSON.parse(document.getElementById("yi-assets").textContent);
const yiSay = (text) => {
  const el = document.getElementById("gallery-boot");
  if (el) el.textContent = text;
};
const yiBytes = async (path) => {
  const packed = Uint8Array.from(atob(yiAssets[path]), (c) => c.charCodeAt(0));
  return new Uint8Array(await new Response(new Response(packed).body.pipeThrough(new DecompressionStream("gzip"))).arrayBuffer());
};

// Reopen on the page the reviewer last looked at, where storage is allowed.
{
  let start = "button";
  try {
    start = localStorage.getItem("yewi-gallery-page") || start;
  } catch {}
  document.getElementById("app").setAttribute("data-gallery", start);
  document.addEventListener(
    "click",
    (event) => {
      const slug = event.target.closest?.('a[href*="/_gallery/"]')?.getAttribute("href").split("/_gallery/")[1];
      try {
        if (slug) localStorage.setItem("yewi-gallery-page", slug);
      } catch {}
    },
    true,
  );
}

// Scripts a component imports on demand, by their path in `site/dist`.
globalThis.__yiVendor = {};
for (const path of yiBoot.vendor) {
  const bytes = await yiBytes(path);
  let binary = "";
  for (let i = 0; i < bytes.length; i += 0x8000) binary += String.fromCharCode.apply(null, bytes.subarray(i, i + 0x8000));
  globalThis.__yiVendor[path] = `data:text/javascript;base64,${btoa(binary)}`;
}

/*__GLUE__*/

yiSay("Starting the components…");
await __wbg_init({ module_or_path: await yiBytes(yiBoot.wasm) });
{
  const stamp = document.querySelector("[data-gallery-build]");
  if (stamp) stamp.textContent = yiBoot.build;
}
