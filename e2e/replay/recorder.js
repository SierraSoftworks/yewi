// Records what a behaviour spec does to an example, as data: the input events
// the browser saw and the native effects that followed (values, focus, popover
// and scroll state). `player.js` dispatches the same events at the same
// elements of a live example in the review gallery, so a "recording" is a few
// kilobytes of JSON instead of a video.
//
// Runs as a Playwright init script (see `fixtures.ts`). Elements are addressed
// by their child-index path below the example's stage, which is the same DOM
// in the gallery.
(() => {
  const match = /\/_examples\/([^/?#]+)\/([^/?#]+)/.exec(location.pathname);
  if (!match || typeof window.__yiReplayEmit !== "function") return;
  const example = `${match[1]}/${match[2]}`;
  const doc = Math.random().toString(36).slice(2);
  const start = performance.now();
  const emit = (event) => {
    event.t = Math.round(performance.now() - start);
    window.__yiReplayEmit({ doc, example, event });
  };

  const stage = () => document.querySelector("[data-example]");
  // A path of child indexes from the stage, or "doc" for the document and
  // anything else outside the example (nothing in yewi renders outside its own tree).
  const locate = (target) => {
    const root = stage();
    if (!(target instanceof Element) || !root || !root.contains(target)) return "doc";
    const path = [];
    for (let el = target; el !== root; el = el.parentElement) path.unshift(Array.prototype.indexOf.call(el.parentElement.children, el));
    return path;
  };
  const mods = (e) => (e.altKey ? 1 : 0) | (e.ctrlKey ? 2 : 0) | (e.metaKey ? 4 : 0) | (e.shiftKey ? 8 : 0);
  const round = (n) => Math.round(n * 1000) / 1000;

  const on = (types, handler) => {
    for (const type of types) {
      window.addEventListener(
        type,
        (e) => {
          if (!e.isTrusted) return;
          const p = locate(e.target);
          const event = handler(e, type);
          if (event) emit({ type, p, ...event });
        },
        { capture: true, passive: true },
      );
    }
  };

  // ---- Pointer and mouse -------------------------------------------------------
  const lastMove = {};
  on(
    [
      "pointerdown", "pointerup", "pointermove", "pointerover", "pointerout", "pointerenter", "pointerleave",
      "mousedown", "mouseup", "mousemove", "mouseover", "mouseout", "mouseenter", "mouseleave",
      "click", "dblclick", "contextmenu", "wheel",
    ],
    (e, type) => {
      if (type === "pointermove" || type === "mousemove") {
        const now = performance.now();
        if (now - (lastMove[type] ?? -Infinity) < 30) return null;
        lastMove[type] = now;
      }
      const rect = e.target instanceof Element ? e.target.getBoundingClientRect() : { left: 0, top: 0, width: 0, height: 0 };
      return {
        // Where in the target, as fractions, so it survives a different layout.
        x: rect.width ? round((e.clientX - rect.left) / rect.width) : 0.5,
        y: rect.height ? round((e.clientY - rect.top) / rect.height) : 0.5,
        b: e.button,
        bs: e.buttons,
        m: mods(e),
        d: e.detail,
        ...(type === "wheel" ? { dx: e.deltaX, dy: e.deltaY } : {}),
        ...(type.startsWith("pointer") ? { pt: e.pointerType } : {}),
      };
    },
  );

  // ---- Keyboard ------------------------------------------------------------------
  on(["keydown", "keyup"], (e) => ({ k: e.key, c: e.code, m: mods(e), r: e.repeat ? 1 : 0 }));

  // ---- What the browser did in response -----------------------------------------
  const value = (el) => {
    if (el instanceof HTMLInputElement && (el.type === "checkbox" || el.type === "radio")) return { ck: el.checked };
    if (el instanceof HTMLInputElement && el.type === "file") return null;
    if ("value" in el) return { v: String(el.value) };
    if (el.isContentEditable) return { tx: el.textContent };
    return {};
  };
  on(["input", "change"], (e) => value(e.target));
  on(["focusin"], (e) => ({ fv: e.target.matches(":focus-visible") ? 1 : 0 }));
  on(["focusout"], (e) => (e.relatedTarget ? null : {}));
  on(["toggle"], (e) => ({ s: e.newState ?? (e.target.open ? "open" : "closed") }));
  on(["close", "cancel"], () => ({}));

  let lastScroll = 0;
  on(["scroll"], (e) => {
    if (!(e.target instanceof Element)) return null; // the page scrolls differently in the gallery
    const now = performance.now();
    if (now - lastScroll < 50) return null;
    lastScroll = now;
    return { sx: e.target.scrollLeft, sy: e.target.scrollTop };
  });
  on(["scrollend"], (e) => (e.target instanceof Element ? { sx: e.target.scrollLeft, sy: e.target.scrollTop } : null));
})();
