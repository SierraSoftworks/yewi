// yewi behaviours: what components do on a page that loads no wasm.
// Everything is wired from `data-yi-*` attributes that components render
// inside `<ServerOnly>`, through listeners on the document, so the script can
// load at any point and content added later works too.
//
// `build.rs` strips comment lines and indentation: keep comments on their own
// lines, and do not write a string that spans lines.
(() => {
  if (window.yewiBehaviours) return;
  window.yewiBehaviours = true;

  const doc = document;
  const root = doc.documentElement;
  const on = (type, selector, handle) =>
    doc.addEventListener(type, (event) => {
      const target = event.target instanceof Element && event.target.closest(selector);
      if (target) handle(target, event);
    });

  // One polite live region for outcomes that have no other text.
  let announcer;
  const announce = (message) => {
    if (!announcer?.isConnected) {
      announcer = doc.createElement("div");
      announcer.className = "yi-visually-hidden";
      announcer.setAttribute("role", "status");
      doc.body.append(announcer);
    }
    announcer.textContent = message;
  };

  // Copy: `data-yi-copy` holds the text, `data-yi-copy-from` the id of the
  // element whose text is copied.
  const write = async (text) => {
    try {
      await navigator.clipboard.writeText(text);
      return true;
    } catch {
      // Over plain HTTP there is no async clipboard.
      const area = doc.createElement("textarea");
      area.value = text;
      area.style.cssText = "position:fixed;opacity:0";
      doc.body.append(area);
      area.select();
      try {
        return doc.execCommand("copy");
      } catch {
        return false;
      } finally {
        area.remove();
      }
    }
  };
  const resets = new WeakMap();
  on("click", "[data-yi-copy],[data-yi-copy-from]", async (button) => {
    const from = button.dataset.yiCopyFrom;
    const text = from ? (doc.getElementById(from)?.textContent ?? "") : button.dataset.yiCopy;
    const copied = await write(text);
    const message = copied ? button.dataset.yiCopied : button.dataset.yiFailed;
    clearTimeout(resets.get(button)?.timer);
    const title = resets.get(button)?.title ?? button.getAttribute("title");
    button.dataset.yiState = copied ? "copied" : "failed";
    if (title !== null) button.title = message;
    announce(message);
    const timer = setTimeout(() => {
      delete button.dataset.yiState;
      if (title !== null) button.title = title;
      announce("");
      resets.delete(button);
    }, 2000);
    resets.set(button, { timer, title });
  });

  // Number stepper: `data-yi-step` on the input is the step size, and
  // `data-yi-step-by` on a button the number of steps it moves.
  const decimals = (value) => (String(value).split(".")[1] ?? "").length;
  const bound = (input, name) => {
    const value = input.getAttribute(name);
    return value === null ? undefined : Number(value);
  };
  const read = (input) => {
    const text = input.value.trim().replace(",", ".");
    return text === "" || Number.isNaN(Number(text)) ? undefined : Number(text);
  };
  const clamp = (input, value) => {
    const [min, max] = [bound(input, "aria-valuemin"), bound(input, "aria-valuemax")];
    if (max !== undefined) value = Math.min(value, max);
    if (min !== undefined) value = Math.max(value, min);
    return value;
  };
  const show = (input, value) => {
    const text = value === undefined ? "" : String(value === 0 ? 0 : value);
    const changed = input.value !== text;
    input.value = text;
    if (value === undefined) input.removeAttribute("aria-valuenow");
    else input.setAttribute("aria-valuenow", text);
    for (const button of input.parentElement.querySelectorAll("[data-yi-step-by]")) {
      const limit = bound(input, Number(button.dataset.yiStepBy) < 0 ? "aria-valuemin" : "aria-valuemax");
      button.disabled = value !== undefined && limit !== undefined && value === limit;
    }
    if (changed) {
      input.dispatchEvent(new Event("input", { bubbles: true }));
      input.dispatchEvent(new Event("change", { bubbles: true }));
    }
  };
  const step = (input, steps) => {
    if (input.disabled || input.readOnly) return;
    const size = Number(input.dataset.yiStep);
    const base = read(input) ?? 0;
    // Rounded to the precision of its inputs, so 0.1 + 0.2 is 0.3.
    const scale = 10 ** Math.max(decimals(size), decimals(base));
    show(input, clamp(input, Math.round((base + steps * size) * scale) / scale));
  };
  const stepped = (button) => button.parentElement.parentElement.querySelector("input[data-yi-step]");
  // Keeps focus, and the caret, in the input.
  on("mousedown", "[data-yi-step-by]", (_, event) => event.preventDefault());
  on("click", "[data-yi-step-by]", (button) => {
    const input = stepped(button);
    if (input) step(input, Number(button.dataset.yiStepBy));
  });
  on("keydown", "input[data-yi-step]", (input, event) => {
    const steps = { ArrowUp: 1, ArrowDown: -1, PageUp: 10, PageDown: -10 }[event.key];
    const limit = { Home: "aria-valuemin", End: "aria-valuemax" }[event.key];
    if (steps) {
      event.preventDefault();
      step(input, steps);
    } else if (limit && bound(input, limit) !== undefined && !input.disabled && !input.readOnly) {
      event.preventDefault();
      show(input, bound(input, limit));
    }
  });
  on("focusout", "input[data-yi-step]", (input) => {
    const value = read(input);
    if (value !== undefined) show(input, clamp(input, value));
  });

  // Dialogs the server rendered as open: `data-yi-open` on a `<dialog>`.
  const open = () => {
    for (const dialog of doc.querySelectorAll("dialog[data-yi-open]:not([open])")) dialog.showModal();
  };

  // Theme toggle: system, light, dark, stored where `THEME_SCRIPT` reads it.
  const themes = ["system", "light", "dark"];
  const names = { system: "System", light: "Light", dark: "Dark" };
  const theme = () => (themes.includes(root.dataset.theme) ? root.dataset.theme : "system");
  const following = (name) => themes[(themes.indexOf(name) + 1) % themes.length];
  const label = () => {
    const text = `Theme: ${names[theme()]}. Switch to ${names[following(theme())]}.`;
    for (const toggle of doc.querySelectorAll("[data-yi-theme-toggle]")) {
      toggle.setAttribute("aria-label", text);
      toggle.title = text;
    }
  };
  on("click", "[data-yi-theme-toggle]", () => {
    const next = following(theme());
    root.setAttribute("data-yi-theme-transition", "");
    setTimeout(() => root.removeAttribute("data-yi-theme-transition"), 250);
    try {
      if (next === "system") {
        root.removeAttribute("data-theme");
        localStorage.removeItem("yewi-theme");
      } else {
        root.dataset.theme = next;
        localStorage.setItem("yewi-theme", next);
      }
    } catch {
      // Storage can be refused; the choice then lasts for this page.
    }
    label();
    window.dispatchEvent(new Event("yewi:theme"));
  });
  const start = () => {
    label();
    open();
  };
  if (doc.readyState === "loading") doc.addEventListener("DOMContentLoaded", start);
  else start();
})();
