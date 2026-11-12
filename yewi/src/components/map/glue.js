// The map itself: MapLibre GL drawing into a shadow root.
//
// Rust (`mod.rs`) decides what is on the map and hands it over as GeoJSON;
// this file knows about pixels. It lives in a shadow root so that the engine's
// own DOM and stylesheet never touch the page: Yew keeps reconciling the
// markup it rendered, and the engine's unlayered CSS cannot override yewi's.
//
// Colours come from the `--yi-map-*` and tone tokens on the host element and
// are resolved again whenever the theme changes.

const HERE = import.meta.url;
const BUNDLED = {
  script: new URL("./vendor/maplibre-gl.js", HERE).href,
  worker: new URL("./vendor/maplibre-gl-worker.js", HERE).href,
};

const TONES = ["neutral", "ok", "warn", "danger", "info", "active"];
const LABEL = "yi-label:";
const HIT_REACH = 4;

const engines = new Map();

function loadEngine(engine) {
  const script = engine?.script ?? BUNDLED.script;
  if (!engines.has(script)) {
    const loading = import(/* @vite-ignore */ script).then((module) => {
      const maplibre = module.Map ? module : module.default;
      const worker = engine?.worker ?? (engine?.script ? null : BUNDLED.worker);
      if (worker) maplibre.setWorkerUrl(worker);
      return maplibre;
    });
    // A failure is not remembered: the next map tries again.
    loading.catch(() => engines.delete(script));
    engines.set(script, loading);
  }
  return engines.get(script);
}

let world;
function loadWorld() {
  world ??= import(/* @vite-ignore */ new URL("./vendor/world-110m.js", HERE).href).then((module) => decodeTopology(module.default));
  world.catch(() => (world = undefined));
  return world;
}

// TopoJSON to GeoJSON, for polygons only: all the bundled outline needs.
function decodeTopology(topology) {
  const [kx, ky] = topology.transform.scale;
  const [dx, dy] = topology.transform.translate;
  const arcs = topology.arcs.map((arc) => {
    let x = 0;
    let y = 0;
    // Web Mercator has no poles: Antarctica's edge along 90°S is pulled in.
    return arc.map(([px, py]) => [(x += px) * kx + dx, Math.max(-85, Math.min(85, (y += py) * ky + dy))]);
  });
  // A ring that crosses the antimeridian is unwrapped past 180° rather than
  // left jumping across the world. One that never comes back encloses a pole
  // (Antarctica) and is already drawn edge to edge.
  const ring = (indexes) => {
    const raw = [];
    for (const index of indexes) {
      const arc = index < 0 ? arcs[~index].slice().reverse() : arcs[index];
      raw.push(...(raw.length ? arc.slice(1) : arc));
    }
    let shift = 0;
    const unwrapped = raw.map(([x, y], at) => {
      const step = at ? x - raw[at - 1][0] : 0;
      if (step > 180) shift -= 360;
      else if (step < -180) shift += 360;
      return [x + shift, y];
    });
    return shift === 0 ? unwrapped : raw;
  };
  const polygon = (rings) => rings.map(ring);
  const features = topology.objects.countries.geometries.map((geometry) => ({
    type: "Feature",
    properties: {},
    geometry:
      geometry.type === "Polygon"
        ? { type: "Polygon", coordinates: polygon(geometry.arcs) }
        : { type: "MultiPolygon", coordinates: geometry.arcs.map(polygon) },
  }));
  return { type: "FeatureCollection", features };
}

const SHADOW_CSS = `
:host { display: block; }
.yi-map-engine { position: absolute; inset: 0; }
.yi-map-engine > div { inline-size: 100%; block-size: 100%; font: inherit; }
.yi-map-probe { position: absolute; visibility: hidden; pointer-events: none; }
.maplibregl-canvas:focus-visible { outline: 2px solid var(--yi-focus); outline-offset: -2px; }
.maplibregl-ctrl-bottom-left .maplibregl-ctrl { margin: 0 0 var(--yi-space-sm) var(--yi-map-inset); }
.maplibregl-ctrl-scale {
  border-color: var(--yi-fg-muted);
  background: var(--yi-map-chrome-bg);
  color: var(--yi-fg-muted);
  font: 500 var(--yi-text-xs) / 1.4 var(--yi-font-body);
}
`;

const ratio = () => Math.min(globalThis.devicePixelRatio || 1, 2);
const frame = () => new Promise((resolve) => requestAnimationFrame(() => resolve()));

class Engine {
  constructor(host, options, onPick, onMove) {
    this.host = host;
    this.options = options;
    this.onPick = onPick;
    this.onMove = onMove;
    this.layers = new Map();
    this.order = [];
    this.selected = "";
    this.labels = new Set();
    this.cursor = "";
    this.destroyed = false;
  }

  async start() {
    const { host, options } = this;
    const outline = options.basemap.kind === "outline";
    const [maplibre, css, land] = await Promise.all([
      loadEngine(options.engine),
      options.engine?.stylesheet ? null : import(/* @vite-ignore */ new URL("./vendor/maplibre-gl-css.js", HERE).href).then((module) => module.default),
      outline ? loadWorld() : null,
    ]);
    if (this.destroyed) return;
    this.maplibre = maplibre;
    this.land = land;

    const root = host.shadowRoot ?? host.attachShadow({ mode: "open" });
    root.replaceChildren();
    if (options.engine?.stylesheet) {
      const link = Object.assign(document.createElement("link"), { rel: "stylesheet", href: options.engine.stylesheet });
      const loaded = new Promise((resolve, reject) => {
        link.addEventListener("load", resolve);
        link.addEventListener("error", () => reject(new Error(`Could not load ${link.href}.`)));
      });
      root.append(link);
      await loaded;
    } else {
      root.append(Object.assign(document.createElement("style"), { textContent: css }));
    }
    root.append(Object.assign(document.createElement("style"), { textContent: SHADOW_CSS }));
    this.probe = Object.assign(document.createElement("span"), { className: "yi-map-probe" });
    const frameElement = Object.assign(document.createElement("div"), { className: "yi-map-engine" });
    const container = document.createElement("div");
    frameElement.append(container);
    root.append(this.probe, frameElement);

    // Labels are drawn on a canvas, so their font has to be there first.
    this.font = `600 12px ${getComputedStyle(host).fontFamily || "sans-serif"}`;
    await document.fonts.load(this.font).catch(() => {});
    if (this.destroyed) return;

    this.palette = this.readPalette();
    const view = options.bounds
      ? { bounds: options.bounds, fitBoundsOptions: { padding: 48 } }
      : { center: options.center, zoom: options.zoom };
    const map = (this.map = new maplibre.Map({
      container,
      style: this.style(),
      ...view,
      minZoom: options.minZoom ?? 0,
      maxZoom: options.maxZoom ?? 22,
      interactive: options.interactive,
      attributionControl: false,
      fadeDuration: 0,
      dragRotate: false,
      pitchWithRotate: false,
      touchPitch: false,
      locale: { "Map.Title": options.label },
    }));
    map.touchZoomRotate?.disableRotation();
    map.keyboard?.disableRotation();
    if (options.scale) map.addControl(new maplibre.ScaleControl({ maxWidth: 96 }), "bottom-left");

    // Labels are drawn the first time the style asks for them.
    if (map.setMissingStyleImageResolver) map.setMissingStyleImageResolver((id) => this.drawLabel(id));
    else map.on("styleimagemissing", (event) => this.drawLabel(event.id));
    map.on("click", (event) => this.pick(event));
    map.on("mousemove", (event) => this.hover(event));
    map.on("moveend", () => {
      const centre = map.getCenter();
      this.onMove(centre.lng, centre.lat, map.getZoom());
    });

    // A tile that does not arrive is not fatal; a style that does not is.
    await new Promise((resolve, reject) => {
      map.once("load", resolve);
      map.on("error", (event) => {
        if (map.loaded() || event.sourceId || event.tile) {
          console.warn("yewi map:", event.error?.message ?? event);
        } else {
          reject(event.error ?? new Error("The map could not be drawn."));
        }
      });
    });
    if (this.destroyed) return;

    this.watchTheme();
  }

  // ---- Colours ----------------------------------------------------------

  // A CSS colour expression as something the engine can parse, whatever
  // notation the browser computed it in.
  colour(expression) {
    this.probe.style.color = "";
    this.probe.style.color = expression;
    const computed = getComputedStyle(this.probe).color;
    const canvas = (this.swatch ??= Object.assign(document.createElement("canvas"), { width: 1, height: 1 }));
    const context = canvas.getContext("2d", { willReadFrequently: true });
    context.clearRect(0, 0, 1, 1);
    context.fillStyle = computed;
    context.fillRect(0, 0, 1, 1);
    const [r, g, b, a] = context.getImageData(0, 0, 1, 1).data;
    return `rgba(${r},${g},${b},${(a / 255).toFixed(3)})`;
  }

  readPalette() {
    const token = (name) => this.colour(`var(--yi-map-${name})`);
    const palette = {
      water: token("water"),
      land: token("land"),
      border: token("border"),
      marker: token("marker"),
      ring: token("marker-ring"),
      label: token("label"),
      halo: token("label-halo"),
      selected: token("selected"),
      tones: Object.fromEntries(TONES.map((tone) => [tone, this.colour(`var(--yi-${tone})`)])),
    };
    const [r, g, b] = palette.halo.match(/\d+/g).map(Number);
    palette.dark = 0.2126 * r + 0.7152 * g + 0.0722 * b < 128;
    return palette;
  }

  watchTheme() {
    const restyle = () => {
      if (this.destroyed) return;
      const before = JSON.stringify(this.palette);
      const wasDark = this.palette.dark;
      this.palette = this.readPalette();
      if (JSON.stringify(this.palette) === before) return;
      const basemap = this.options.basemap;
      const swaps = basemap.kind === "style" && basemap.darkUrl && wasDark !== this.palette.dark;
      if (swaps) {
        this.map.setStyle(this.style(), { diff: false });
        this.map.once("style.load", () => this.restore());
      } else {
        this.repaint();
      }
    };
    // Tokens change with `data-theme`, a class or an inline style on any
    // ancestor, or with the system preference.
    this.observer = new MutationObserver(restyle);
    for (let node = this.host; node; node = node.parentElement) {
      this.observer.observe(node, { attributes: true, attributeFilter: ["data-theme", "class", "style"] });
    }
    this.media = globalThis.matchMedia("(prefers-color-scheme: dark)");
    this.media.addEventListener("change", restyle);
    this.restyle = restyle;
  }

  // ---- Basemap ----------------------------------------------------------

  style() {
    const { basemap } = this.options;
    const palette = this.palette;
    if (basemap.kind === "style") {
      return palette.dark && basemap.darkUrl ? basemap.darkUrl : basemap.url;
    }
    if (basemap.kind === "raster") {
      const source = (tiles) => ({ type: "raster", tiles, tileSize: basemap.tileSize, maxzoom: basemap.maxZoom });
      const sources = { "yi:tiles": source(basemap.tiles) };
      const layers = [
        { id: "yi:water", type: "background", paint: { "background-color": palette.water } },
        { id: "yi:tiles", type: "raster", source: "yi:tiles", paint: this.rasterPaint(false) },
      ];
      if (basemap.darkTiles) {
        sources["yi:tiles-dark"] = source(basemap.darkTiles);
        layers.push({ id: "yi:tiles-dark", type: "raster", source: "yi:tiles-dark", paint: this.rasterPaint(true) });
      }
      return { version: 8, sources, layers };
    }
    return {
      version: 8,
      sources: { "yi:world": { type: "geojson", data: this.land } },
      layers: [
        { id: "yi:water", type: "background", paint: { "background-color": palette.water } },
        { id: "yi:land", type: "fill", source: "yi:world", paint: { "fill-color": palette.land } },
        { id: "yi:borders", type: "line", source: "yi:world", paint: { "line-color": palette.border, "line-width": 0.75 } },
      ],
    };
  }

  // Raster tiles are drawn for a light page. Without a dark set, a dark page
  // dims and desaturates them; with one, the two sets swap.
  rasterPaint(darkSet) {
    const { basemap } = this.options;
    const dark = this.palette.dark;
    if (basemap.darkTiles) {
      return { "raster-opacity": darkSet === dark ? 1 : 0, "raster-fade-duration": 0 };
    }
    const dim = dark && basemap.dim;
    return {
      "raster-brightness-max": dim ? 0.62 : 1,
      "raster-saturation": dim ? -0.45 : 0,
      "raster-contrast": dim ? 0.1 : 0,
      "raster-fade-duration": 0,
    };
  }

  repaint() {
    const { map, palette } = this;
    const set = (layer, paint) => {
      if (!map.getLayer(layer)) return;
      for (const [name, value] of Object.entries(paint)) map.setPaintProperty(layer, name, value);
    };
    set("yi:water", { "background-color": palette.water });
    set("yi:land", { "fill-color": palette.land });
    set("yi:borders", { "line-color": palette.border });
    set("yi:tiles", this.rasterPaint(false));
    set("yi:tiles-dark", this.rasterPaint(true));
    for (const layer of this.layers.values()) {
      for (const [id, , paint] of this.parts(layer)) set(id, paint);
    }
    for (const id of this.labels) {
      if (map.hasImage(id)) map.updateImage(id, this.labelImage(id));
    }
  }

  // ---- Data layers ------------------------------------------------------

  toneColour(layer) {
    const fallback = layer.tone ? this.palette.tones[layer.tone] : this.palette.marker;
    const byTone = ["match", ["get", "tone"], ...TONES.flatMap((tone) => [tone, this.palette.tones[tone]]), fallback];
    return ["case", ["has", "color"], ["to-color", ["get", "color"], fallback], byTone];
  }

  // `[layer id, layer definition, paint]` for each engine layer a yewi layer
  // is drawn with, bottom first.
  parts(layer) {
    const source = `yi:data:${layer.id}`;
    const id = (part) => `${source}:${part}`;
    const colour = this.toneColour(layer);
    const chosen = ["==", ["to-string", ["get", "id"]], this.selected || "\u0000"];
    if (layer.kind === "line" || layer.kind === "area") {
      const line = [
        id("line"),
        { type: "line", source, layout: { "line-join": "round", "line-cap": "round" } },
        { "line-color": colour, "line-width": ["case", chosen, 4, 2.5] },
      ];
      if (layer.kind === "line") return [line];
      const fill = [
        id("fill"),
        { type: "fill", source, filter: ["==", ["geometry-type"], "Polygon"] },
        { "fill-color": colour, "fill-opacity": ["case", chosen, 0.3, 0.16] },
      ];
      return [fill, line];
    }
    return [
      [
        id("halo"),
        { type: "circle", source, filter: chosen },
        {
          "circle-radius": 13,
          "circle-color": this.palette.selected,
          "circle-opacity": 0.2,
          "circle-stroke-color": this.palette.selected,
          "circle-stroke-width": 2,
        },
      ],
      [
        id("dot"),
        { type: "circle", source },
        {
          "circle-radius": 6,
          "circle-color": colour,
          "circle-stroke-color": this.palette.ring,
          "circle-stroke-width": 2,
        },
      ],
      [
        id("label"),
        {
          type: "symbol",
          source,
          filter: ["has", "label"],
          layout: {
            "icon-image": ["concat", LABEL, ["to-string", ["get", "label"]]],
            "icon-anchor": "top",
            "icon-offset": [0, 9],
            "icon-padding": 1,
          },
        },
        {},
      ],
    ];
  }

  hitLayers() {
    const ids = [];
    for (const layer of this.layers.values()) {
      if (!layer.interactive) continue;
      for (const [id] of this.parts(layer)) {
        if (!id.endsWith(":halo") && !id.endsWith(":label") && this.map.getLayer(id)) ids.push(id);
      }
    }
    return ids;
  }

  addLayer(layer) {
    const { map } = this;
    const source = `yi:data:${layer.id}`;
    if (!map.getSource(source)) map.addSource(source, { type: "geojson", data: layer.parsed });
    for (const [id, definition, paint] of this.parts(layer)) {
      if (!map.getLayer(id)) map.addLayer({ id, ...definition, paint });
    }
  }

  removeLayer(layer) {
    const { map } = this;
    const source = `yi:data:${layer.id}`;
    for (const part of ["halo", "dot", "label", "fill", "line"]) {
      if (map.getLayer(`${source}:${part}`)) map.removeLayer(`${source}:${part}`);
    }
    if (map.getSource(source)) map.removeSource(source);
  }

  // After the style was replaced: everything this file added is gone.
  restore() {
    this.labels.clear();
    for (const id of this.order) this.addLayer(this.layers.get(id));
  }

  parse(data) {
    try {
      return JSON.parse(data);
    } catch (error) {
      console.error("yewi map: a layer's GeoJSON could not be parsed.", error);
      return { type: "FeatureCollection", features: [] };
    }
  }

  // Declares the layers, bottom first. While their ids, kinds and colours
  // stay the same only data is replaced, and only where the text differs.
  setLayers(ids, kinds, tones, interactive, data) {
    const next = new Map();
    let same = ids.length === this.order.length;
    ids.forEach((id, index) => {
      const previous = this.layers.get(id);
      const layer = {
        id,
        kind: kinds[index],
        tone: tones[index],
        interactive: interactive[index],
        data: data[index],
      };
      layer.parsed = previous?.data === layer.data ? previous.parsed : this.parse(layer.data);
      same &&=
        this.order[index] === id &&
        previous.kind === layer.kind &&
        previous.tone === layer.tone;
      next.set(id, layer);
    });
    if (same) {
      for (const [id, layer] of next) {
        if (this.layers.get(id).data !== layer.data) this.map.getSource(`yi:data:${id}`)?.setData(layer.parsed);
      }
    } else {
      for (const layer of this.layers.values()) this.removeLayer(layer);
    }
    this.layers = next;
    this.order = [...ids];
    if (!same) for (const layer of next.values()) this.addLayer(layer);
  }

  setData(id, data) {
    const layer = this.layers.get(id);
    if (!layer || layer.data === data) return;
    layer.data = data;
    layer.parsed = this.parse(data);
    this.map.getSource(`yi:data:${id}`)?.setData(layer.parsed);
  }

  setSelected(id) {
    this.selected = id ?? "";
    for (const layer of this.layers.values()) {
      for (const [part, definition, paint] of this.parts(layer)) {
        if (!this.map.getLayer(part)) continue;
        if (part.endsWith(":halo")) this.map.setFilter(part, definition.filter);
        else for (const [name, value] of Object.entries(paint)) this.map.setPaintProperty(part, name, value);
      }
    }
  }

  // ---- Labels -----------------------------------------------------------

  // Text as an image: the engine draws text from glyph tiles, which need a
  // font server; the browser already has the page's font.
  labelImage(id) {
    const text = id.slice(LABEL.length);
    const scale = ratio();
    const canvas = document.createElement("canvas");
    const context = canvas.getContext("2d", { willReadFrequently: true });
    context.font = this.font;
    canvas.width = Math.ceil((context.measureText(text).width + 8) * scale);
    canvas.height = Math.ceil(18 * scale);
    context.scale(scale, scale);
    context.font = this.font;
    context.textBaseline = "middle";
    context.lineJoin = "round";
    context.lineWidth = 3;
    context.strokeStyle = this.palette.halo;
    context.fillStyle = this.palette.label;
    context.strokeText(text, 4, 9);
    context.fillText(text, 4, 9);
    return context.getImageData(0, 0, canvas.width, canvas.height);
  }

  drawLabel(id) {
    if (!id.startsWith(LABEL) || this.map.hasImage(id)) return;
    this.map.addImage(id, this.labelImage(id), { pixelRatio: ratio() });
    this.labels.add(id);
  }

  // ---- Pointer ----------------------------------------------------------

  hits(point) {
    const layers = this.hitLayers();
    if (!layers.length) return [];
    const box = [
      [point.x - HIT_REACH, point.y - HIT_REACH],
      [point.x + HIT_REACH, point.y + HIT_REACH],
    ];
    const seen = new Set();
    const flat = [];
    for (const feature of this.map.queryRenderedFeatures(box, { layers })) {
      const layer = feature.source.slice("yi:data:".length);
      const id = String(feature.properties?.id ?? "");
      const key = `${layer}\u0000${id}`;
      if (seen.has(key)) continue;
      seen.add(key);
      flat.push(layer, id);
    }
    return flat;
  }

  pick(event) {
    this.onPick(event.lngLat.lng, event.lngLat.lat, this.hits(event.point));
  }

  hover(event) {
    if (this.cursor || this.hovering) return;
    this.hovering = true;
    requestAnimationFrame(() => {
      this.hovering = false;
      if (this.destroyed || this.cursor) return;
      this.map.getCanvas().style.cursor = this.hits(event.point).length ? "pointer" : "";
    });
  }

  setCursor(cursor) {
    this.cursor = cursor;
    this.map.getCanvas().style.cursor = cursor;
  }

  // ---- View -------------------------------------------------------------

  motion() {
    return globalThis.matchMedia("(prefers-reduced-motion: reduce)").matches ? { duration: 0 } : {};
  }

  flyTo(lng, lat, zoom) {
    this.map.easeTo({ center: [lng, lat], ...(Number.isFinite(zoom) ? { zoom } : {}), ...this.motion() });
  }

  fitBounds(west, south, east, north) {
    this.map.fitBounds(
      [
        [west, south],
        [east, north],
      ],
      { padding: 48, maxZoom: 14, ...this.motion() },
    );
  }

  // Fits everything the data layers hold.
  fitAll() {
    let [west, south, east, north] = [Infinity, Infinity, -Infinity, -Infinity];
    const visit = (coordinates) => {
      if (typeof coordinates[0] === "number") {
        west = Math.min(west, coordinates[0]);
        east = Math.max(east, coordinates[0]);
        south = Math.min(south, coordinates[1]);
        north = Math.max(north, coordinates[1]);
      } else {
        coordinates.forEach(visit);
      }
    };
    for (const layer of this.layers.values()) {
      const features = layer.parsed.features ?? [layer.parsed];
      for (const feature of features) {
        const geometry = feature.geometry ?? feature;
        if (geometry?.coordinates) visit(geometry.coordinates);
      }
    }
    if (west <= east) this.fitBounds(west, south, east, north);
  }

  zoomBy(delta) {
    this.map.easeTo({ zoom: this.map.getZoom() + delta, ...this.motion() });
  }

  // Resolves once the engine has drawn everything it currently has.
  async settled() {
    await frame();
    if (this.destroyed || this.map.loaded()) return;
    await new Promise((resolve) => this.map.once("idle", resolve));
  }

  // Clears the busy marks on `host` once the picture is complete.
  settle(host) {
    this.settled().then(() => {
      host.removeAttribute("aria-busy");
      host.removeAttribute("data-yi-pending");
    });
  }

  engine() {
    return this.map;
  }

  destroy() {
    if (this.destroyed) return;
    this.destroyed = true;
    this.observer?.disconnect();
    this.media?.removeEventListener("change", this.restyle);
    this.map?.remove();
    this.host.shadowRoot?.replaceChildren();
  }
}

// Draws a map into `host`. `options` is JSON; see `MapProps::options` in
// `mod.rs`. `done` is called exactly once: with the engine, or with null and
// why there is none.
export function create(host, options, onPick, onMove, done) {
  let engine;
  Promise.resolve()
    .then(() => {
      engine = new Engine(host, JSON.parse(options), onPick, onMove);
      return engine.start();
    })
    .then(
      () => done(engine, null),
      (error) => {
        engine?.destroy();
        done(null, error?.message ? String(error.message) : "The map could not be started.");
      },
    );
}

// Arrow keys move between the buttons of a toolbar, as its role promises.
export function toolbarKey(event) {
  const step = { ArrowRight: 1, ArrowDown: 1, ArrowLeft: -1, ArrowUp: -1 }[event.key];
  const edge = { Home: 0, End: -1 }[event.key];
  if (step === undefined && edge === undefined) return;
  const buttons = [...event.currentTarget.querySelectorAll("button:not(:disabled)")];
  const current = buttons.indexOf(document.activeElement);
  if (current < 0) return;
  const next = edge !== undefined ? buttons.at(edge) : buttons[(current + step + buttons.length) % buttons.length];
  next?.focus();
  event.preventDefault();
}
