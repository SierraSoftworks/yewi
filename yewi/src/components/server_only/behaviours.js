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

  // App shell: the menu button opens the navigation drawer natively
  // (`commandfor`). Where the sidebar sits beside the page, that press hides
  // or shows the column instead, remembered under `data-yi-collapse-key`.
  const collapse = (shell, hidden) => {
    shell.classList.toggle("yi-app-shell--collapsed", hidden);
    for (const button of shell.querySelectorAll("[data-yi-shell-toggle]")) button.setAttribute("aria-expanded", String(!hidden));
  };
  on("click", "[data-yi-shell-toggle]", (button, event) => {
    const shell = button.closest(".yi-app-shell");
    const body = shell?.querySelector(".yi-app-shell__body");
    if (!body || getComputedStyle(body).getPropertyValue("--yi-app-shell-narrow").trim() === "1") return;
    event.preventDefault();
    const hidden = !shell.classList.contains("yi-app-shell--collapsed");
    collapse(shell, hidden);
    const key = shell.dataset.yiCollapseKey;
    try {
      if (key && hidden) localStorage.setItem(key, "1");
      else if (key) localStorage.removeItem(key);
    } catch {
      // Not remembered, then.
    }
  });
  const restore = () => {
    for (const shell of doc.querySelectorAll("[data-yi-collapse-key]")) {
      try {
        if (localStorage.getItem(shell.dataset.yiCollapseKey) === "1") collapse(shell, true);
      } catch {
        // Storage is refused: the shell stays as rendered.
      }
    }
  };

  // Popovers: `data-yi-popover` on the wrapper of a trigger is `click` or
  // `hover`, and its surface is a native popover among its children.
  const surfaceOf = (wrapper) => wrapper.querySelector(":scope > [popover]");
  const isOpen = (surface) => surface.matches(":popover-open");
  const controlOf = (wrapper) => {
    const inner = [...wrapper.querySelectorAll("a[href],button,input,select,textarea,summary,[tabindex]")].find((element) => !surfaceOf(wrapper)?.contains(element));
    return inner ?? (wrapper.hasAttribute("tabindex") ? wrapper : undefined);
  };
  const items = (menu) => [...menu.querySelectorAll("[role^=menuitem]:not([aria-disabled=true])")];
  // A press outside an open popover closes it before the click arrives, so
  // the click on its own trigger must not open it again.
  let pressed;
  doc.addEventListener("pointerdown", (event) => {
    const wrapper = event.target instanceof Element && event.target.closest("[data-yi-popover=click]");
    const surface = wrapper && surfaceOf(wrapper);
    pressed = surface && isOpen(surface) && !surface.contains(event.target) ? surface : undefined;
  }, true);
  const toggle = (wrapper, event, focus) => {
    const surface = surfaceOf(wrapper);
    if (!surface || surface.contains(event.target)) return;
    const was = event.detail !== 0 && pressed === surface;
    pressed = undefined;
    if (was || isOpen(surface)) return surface.hidePopover();
    surface.showPopover();
    const menu = surface.querySelector("[data-yi-menu]");
    if (menu) (focus === "last" ? items(menu).at(-1) : items(menu)[0])?.focus();
  };
  on("click", "[data-yi-popover=click]", (wrapper, event) => toggle(wrapper, event));
  on("keydown", "[data-yi-popover=click]", (wrapper, event) => {
    const surface = surfaceOf(wrapper);
    if (!surface || surface.contains(event.target)) return;
    const opens = { ArrowDown: "first", ArrowUp: "last" }[event.key];
    const menu = surface.querySelector("[data-yi-menu]");
    if (opens && menu) {
      event.preventDefault();
      if (!isOpen(surface)) surface.showPopover();
      (opens === "last" ? items(menu).at(-1) : items(menu)[0])?.focus();
    } else if ((event.key === "Enter" || event.key === " ") && event.target === wrapper) {
      event.preventDefault();
      toggle(wrapper, event);
    }
  });
  // `toggle` does not bubble.
  doc.addEventListener("toggle", (event) => {
    const wrapper = event.target instanceof Element && event.target.parentElement?.closest("[data-yi-popover]");
    if (!wrapper || surfaceOf(wrapper) !== event.target) return;
    const control = controlOf(wrapper);
    const shown = event.newState === "open";
    if (wrapper.dataset.yiPopover === "click") control?.setAttribute("aria-expanded", String(shown));
    else if (shown) control?.setAttribute("aria-describedby", event.target.id);
    else control?.removeAttribute("aria-describedby");
  }, true);
  // Hover: shown while the pointer rests on the trigger or the surface, or
  // the trigger has keyboard focus.
  const intents = new WeakMap();
  const intent = (wrapper, change) => {
    const state = intents.get(wrapper) ?? {};
    intents.set(wrapper, Object.assign(state, change));
    clearTimeout(state.timer);
    const surface = surfaceOf(wrapper);
    if (!surface) return;
    const apply = () => {
      const want = Boolean(state.hover || state.focus);
      if (want !== isOpen(surface)) surface.togglePopover(want);
    };
    if (change.hover) state.timer = setTimeout(apply, Number(wrapper.dataset.yiDelay ?? 0));
    else apply();
  };
  const crossing = (wrapper, event) => !(event.relatedTarget instanceof Node && wrapper.contains(event.relatedTarget));
  on("mouseover", "[data-yi-popover=hover]", (wrapper, event) => {
    if (crossing(wrapper, event)) intent(wrapper, { hover: true });
  });
  on("mouseout", "[data-yi-popover=hover]", (wrapper, event) => {
    if (crossing(wrapper, event)) intent(wrapper, { hover: false });
  });
  on("focusin", "[data-yi-popover=hover]", (wrapper, event) => {
    if (event.target.matches(":focus-visible")) intent(wrapper, { focus: true });
  });
  on("focusout", "[data-yi-popover=hover]", (wrapper, event) => {
    if (crossing(wrapper, event)) intent(wrapper, { focus: false });
  });
  doc.addEventListener("keydown", (event) => {
    if (event.key !== "Escape") return;
    for (const surface of doc.querySelectorAll("[data-yi-popover=hover] > :popover-open")) intent(surface.parentElement, { hover: false, focus: false });
  });

  // Menus: `data-yi-menu`. Arrow keys, Home, End and a letter move between
  // items, and choosing one closes the popover the menu is in.
  on("keydown", "[data-yi-menu]", (menu, event) => {
    const all = items(menu);
    const at = all.indexOf(doc.activeElement);
    const letter = event.key.length === 1 && event.key !== " " && !event.ctrlKey && !event.metaKey && !event.altKey;
    const starts = (item) => item.textContent.trim().toLowerCase().startsWith(event.key.toLowerCase());
    const next = {
      ArrowDown: () => all[(at + 1) % all.length],
      ArrowUp: () => all.at(at < 0 ? -1 : at - 1),
      Home: () => all[0],
      End: () => all.at(-1),
    }[event.key]?.() ?? (letter ? [...all.slice(at + 1), ...all.slice(0, at + 1)].find(starts) : undefined);
    if (!next) return;
    event.preventDefault();
    next.focus();
  });
  on("click", "[data-yi-menu] [role^=menuitem]", (item, event) => {
    if (item.matches("[aria-disabled=true]")) return event.preventDefault();
    // After the press has done its own work: a hidden button submits nothing.
    setTimeout(() => item.closest("[popover]:popover-open")?.hidePopover());
  });

  // File drop: `data-yi-file-drop` on the zone. Dropped files go into its
  // input, and the label names what was chosen.
  const picker = (zone) => zone.querySelector("input[type=file]");
  on("dragover", "[data-yi-file-drop]", (zone, event) => {
    if (!event.dataTransfer.types.includes("Files")) return;
    event.preventDefault();
    zone.classList.toggle("yi-file-drop--over", !picker(zone).disabled);
  });
  on("dragleave", "[data-yi-file-drop]", (zone) => zone.classList.remove("yi-file-drop--over"));
  on("drop", "[data-yi-file-drop]", (zone, event) => {
    event.preventDefault();
    zone.classList.remove("yi-file-drop--over");
    const input = picker(zone);
    if (input.disabled) return;
    const kept = new DataTransfer();
    for (const file of event.dataTransfer.files) if (input.multiple || !kept.items.length) kept.items.add(file);
    input.files = kept.files;
    input.dispatchEvent(new Event("change", { bubbles: true }));
  });
  on("change", "[data-yi-file-drop] input[type=file]", (input) => {
    const label = input.closest("[data-yi-file-drop]").querySelector(".yi-file-drop__label");
    label.dataset.yiLabel ??= label.textContent;
    label.textContent = [...input.files].map((file) => file.name).join(", ") || label.dataset.yiLabel;
  });

  // What the server rendered as open: `data-yi-open` on a `<dialog>` or a
  // popover.
  const open = () => {
    for (const dialog of doc.querySelectorAll("dialog[data-yi-open]:not([open])")) dialog.showModal();
    for (const surface of doc.querySelectorAll("[popover][data-yi-open]:not(:popover-open)")) surface.showPopover();
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
    restore();
    open();
  };
  if (doc.readyState === "loading") doc.addEventListener("DOMContentLoaded", start);
  else start();
})();
