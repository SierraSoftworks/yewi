// Plays the recordings made by `recorder.js` against the live examples in the
// packed review gallery. Each recorded event is dispatched again at the same
// element; effects the browser only performs for real input (typing, focus
// moves, light dismiss) are applied from the recording.
//
// Synthetic input cannot set `:hover`, `:active` or `:focus-visible`, so the
// gallery's stylesheet is rewritten to also match `[data-yi-replay-hover]`,
// `[data-yi-replay-active]` and `[data-yi-replay-focus]` (see `viewer.mjs`).
(() => {
  const recordings = JSON.parse(document.getElementById("yi-replays")?.textContent || "[]");
  if (!recordings.length) return;
  const byExample = new Map();
  for (const recording of recordings) {
    if (!byExample.has(recording.example)) byExample.set(recording.example, []);
    byExample.get(recording.example).push(recording);
  }

  // For checking replays from outside (`verify-replays.mjs`): play one and resolve when done.
  // `fast` drops the pacing added for human eyes and keeps the recorded timing.
  window.__yiReplay = { recordings, play: (index, fast = false) => start(recordings[index], fast) };
  async function start(recording, fast) {
    const slot = document.querySelector(`[data-gallery-replays="${recording.example}"]`);
    const button = [...slot.children][byExample.get(recording.example).indexOf(recording)];
    await toggle(recording, button, fast);
    return slot.closest(".gallery-example").querySelector("[data-example]").innerText;
  }

  const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
  const frame = () => new Promise((resolve) => requestAnimationFrame(() => resolve()));

  // ---- Controls under each example ----------------------------------------------
  let playing = null; // { stop: boolean, button }
  const fill = () => {
    for (const slot of document.querySelectorAll("[data-gallery-replays]")) {
      const id = slot.getAttribute("data-gallery-replays");
      if (slot.dataset.filled === id) continue;
      slot.dataset.filled = id;
      slot.replaceChildren(
        ...(byExample.get(id) ?? []).map((recording) => {
          const button = document.createElement("button");
          button.type = "button";
          button.className = "gallery-replay";
          button.textContent = recording.title;
          button.title = recording.inexact
            ? "Approximate: this replay ends differently from the spec (it needs a faked clock, a file, the clipboard or a resized window)"
            : "Replay this behaviour spec on the example above";
          if (recording.inexact) button.dataset.inexact = "";
          button.addEventListener("click", () => toggle(recording, button));
          return button;
        }),
      );
    }
  };
  new MutationObserver(fill).observe(document.getElementById("app"), { childList: true, subtree: true, attributeFilter: ["data-gallery-replays"] });
  fill();

  async function toggle(recording, button, fast = false) {
    const mine = playing?.button === button;
    if (playing) playing.stop = true;
    if (mine) return;
    while (playing) await sleep(30);
    const run = (playing = { stop: false, button, fast });
    button.dataset.state = "playing";
    try {
      cursor.dataset.fast = fast ? "1" : "";
      await play(recording, button.closest(".gallery-example"), run);
    } catch (error) {
      console.error("replay failed", error);
    } finally {
      delete button.dataset.state;
      cleanup();
      playing = null;
    }
  }

  // ---- Overlays: a cursor and the keys being pressed -----------------------------
  const cursor = document.createElement("div");
  cursor.className = "gallery-cursor";
  cursor.hidden = true;
  const keys = document.createElement("div");
  keys.className = "gallery-keys";
  keys.hidden = true;
  document.body.append(cursor, keys);
  let cursorAt = null;
  let keysTimer;

  async function moveCursor(x, y) {
    const first = cursorAt === null;
    const far = !first && Math.hypot(x - cursorAt.x, y - cursorAt.y) > 4;
    cursorAt = { x, y };
    if (first) cursor.style.transition = "none";
    cursor.style.translate = `${x}px ${y}px`;
    if (first) {
      cursor.hidden = false;
      await frame();
      cursor.style.transition = "";
    } else if (far && !cursor.dataset.fast) {
      await sleep(360);
    }
  }

  const KEY_NAMES = { " ": "Space", ArrowUp: "↑", ArrowDown: "↓", ArrowLeft: "←", ArrowRight: "→" };
  function showKey(event) {
    const parts = [];
    if (event.m & 2) parts.push("Ctrl");
    if (event.m & 1) parts.push("Alt");
    if (event.m & 4) parts.push("Meta");
    if (event.m & 8 && event.k.length > 1) parts.push("Shift");
    if (!["Control", "Alt", "Meta", "Shift"].includes(event.k)) parts.push(KEY_NAMES[event.k] ?? event.k);
    keys.replaceChildren(
      ...parts.map((part) => {
        const kbd = document.createElement("kbd");
        kbd.textContent = part;
        return kbd;
      }),
    );
    keys.hidden = false;
    clearTimeout(keysTimer);
    keysTimer = setTimeout(() => (keys.hidden = true), 1200);
  }

  // ---- State that only real input sets ---------------------------------------------
  const marked = { hover: [], active: [], focus: [] };
  function mark(kind, element, chain) {
    const attribute = `data-yi-replay-${kind}`;
    for (const el of marked[kind]) el.removeAttribute(attribute);
    marked[kind] = [];
    for (let el = element; el && el.nodeType === 1; el = chain ? el.parentElement : null) {
      el.setAttribute(attribute, "");
      marked[kind].push(el);
    }
  }
  function cleanup() {
    for (const kind of Object.keys(marked)) mark(kind, null);
    cursor.hidden = true;
    cursorAt = null;
    keys.hidden = true;
  }

  // ---- Playback ---------------------------------------------------------------------
  const MOUSE = /^(mouse|pointer|click|dblclick|contextmenu|wheel)/;
  const NO_BUBBLE = /(enter|leave)$/;

  async function play(recording, section, run) {
    section.querySelector("[data-gallery-reset]")?.click();
    await sleep(60);
    for (let i = 0; i < 100 && section.querySelector("[data-yi-pending]"); i++) await sleep(100);
    const stage = section.querySelector("[data-example]");
    stage.scrollIntoView({ block: "nearest", behavior: run.fast ? "instant" : "smooth" });
    await sleep(run.fast ? 30 : 350);

    // Keys typed "at the document" go wherever focus is; the pointer is then outside the example.
    const resolve = (path, type) => {
      if (path === "doc") return /^key/.test(type) && stage.contains(document.activeElement) ? document.activeElement : document.body;
      let el = stage;
      for (const index of path) el = el?.children[index];
      return el ?? null;
    };

    // Events the browser fires by itself in response to ours (a click on a label
    // clicks its input, a click on a checkbox changes it). The recording holds
    // them too, so each one seen here cancels its recorded twin.
    const ours = new WeakSet();
    let natural = [];
    const watch = (e) => {
      if (!ours.has(e)) natural.push({ type: e.type, target: e.target });
    };
    // Script focus is "visible" or not by the browser's own guess, which follows the
    // reviewer's real input rather than the recording. Components that ask get the recorded answer.
    const matches = Element.prototype.matches;
    Element.prototype.matches = function (selector) {
      return matches.call(this, typeof selector === "string" ? selector.replaceAll(":focus-visible", "[data-yi-replay-focus]") : selector);
    };
    const WATCHED = ["click", "input", "change", "focusin", "toggle"];
    for (const type of WATCHED) window.addEventListener(type, watch, true);
    const happened = (type, target) => {
      const index = natural.findIndex((n) => n.type === type && n.target === target);
      if (index < 0) return false;
      natural.splice(index, 1);
      return true;
    };
    const send = (target, event) => {
      ours.add(event);
      target.dispatchEvent(event);
    };

    let previous = 0;
    let lastAction = 0;
    try {
      for (const event of recording.events) {
        if (run.stop) return;
        // Specs run faster than anyone can follow: keep short gaps, cap long ones,
        // and leave room before each deliberate action.
        let wait = Math.min(event.t - previous, 900);
        previous = event.t;
        const deliberate =
          event.type === "pointerdown" || (event.type === "keydown" && !event.r) || ((event.type === "input" || event.type === "toggle") && !MOUSE.test(event.type));
        if (deliberate && !run.fast) {
          const typing = event.type === "keydown" && event.k.length === 1 && !(event.m & 7);
          wait = Math.max(wait, (typing ? 90 : 420) - (performance.now() - lastAction));
        }
        if (wait > 4) await sleep(wait);
        if (run.stop) return;

        const target = resolve(event.p, event.type);
        if (!target || !target.isConnected) continue;
        const { type } = event;

        if (MOUSE.test(type)) {
          const rect = target.getBoundingClientRect();
          const clientX = rect.left + rect.width * event.x;
          const clientY = rect.top + rect.height * event.y;
          if (target !== document.body) await moveCursor(clientX, clientY);
          if (run.stop) return;
          if (type === "click" && happened("click", target)) continue;
          const init = {
            bubbles: !NO_BUBBLE.test(type),
            cancelable: true,
            composed: true,
            view: window,
            clientX,
            clientY,
            screenX: clientX,
            screenY: clientY,
            button: event.b,
            buttons: event.bs,
            detail: event.d,
            altKey: !!(event.m & 1),
            ctrlKey: !!(event.m & 2),
            metaKey: !!(event.m & 4),
            shiftKey: !!(event.m & 8),
          };
          if (type === "pointerover" || type === "mouseover" || type === "pointermove") mark("hover", target, true);
          if (type === "pointerdown") {
            mark("active", target, true);
            cursor.dataset.down = "";
            lastAction = performance.now();
          }
          if (type === "pointerup") {
            mark("active", null);
            delete cursor.dataset.down;
          }
          if (type === "wheel") send(target, new WheelEvent(type, { ...init, deltaX: event.dx, deltaY: event.dy }));
          else if (type.startsWith("pointer")) send(target, new PointerEvent(type, { ...init, pointerId: 1, pointerType: event.pt || "mouse", isPrimary: true }));
          else send(target, new MouseEvent(type, init));
          continue;
        }

        switch (type) {
          case "keydown":
          case "keyup":
            if (type === "keydown") {
              showKey(event);
              lastAction = performance.now();
            }
            send(target, new KeyboardEvent(type, {
              bubbles: true, cancelable: true, composed: true, view: window,
              key: event.k, code: event.c, repeat: !!event.r,
              altKey: !!(event.m & 1), ctrlKey: !!(event.m & 2), metaKey: !!(event.m & 4), shiftKey: !!(event.m & 8),
            }));
            break;
          case "input":
          case "change":
            if (happened(type, target)) break;
            if ("ck" in event) target.checked = event.ck;
            else if ("v" in event) target.value = event.v;
            else if ("tx" in event) target.textContent = event.tx;
            send(target, type === "input" ? new InputEvent(type, { bubbles: true, composed: true }) : new Event(type, { bubbles: true }));
            lastAction = performance.now();
            break;
          case "focusin":
            if (happened(type, target) || document.activeElement === target) {
              mark("focus", event.fv ? target : null);
              break;
            }
            mark("focus", event.fv ? target : null);
            target.focus({ preventScroll: true });
            happened("focusin", target);
            break;
          case "focusout":
            if (document.activeElement === target) target.blur();
            mark("focus", null);
            break;
          case "toggle":
            if (happened(type, target)) break;
            if (target instanceof HTMLDetailsElement) target.open = event.s === "open";
            else if (target.hasAttribute("popover") && target.matches(":popover-open") !== (event.s === "open")) {
              if (event.s === "open") target.showPopover();
              else target.hidePopover();
              happened("toggle", target);
            }
            break;
          case "cancel":
            send(target, new Event("cancel", { cancelable: true }));
            break;
          case "close":
            if (target instanceof HTMLDialogElement && target.open) target.close();
            break;
          case "scroll":
          case "scrollend":
            target.scrollTo({ left: event.sx, top: event.sy, behavior: "instant" });
            break;
        }
      }
      await sleep(run.fast ? 150 : 900);
    } finally {
      Element.prototype.matches = matches;
      for (const type of WATCHED) window.removeEventListener(type, watch, true);
    }
  }
})();
