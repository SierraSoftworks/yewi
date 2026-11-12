// The colour tokens keep WCAG AA contrast for any accent an app might choose.
import { expect, test, type Theme } from "../fixtures";

const AA = 4.5;

const ACCENTS: Record<string, string> = {
  default: "light-dark(#409eff, #69b4ff)",
  buckle: "#b9852f",
  analytics: "#1c7aac",
  rustak: "#156a4f",
  automate: "#0f5a80",
  grey: "#2b79c2",
  symbols: "#6d4aff",
  red: "#c4372e",
  yellow: "#ffd400",
  "near black": "#111111",
  "near white": "#f2f2f2",
  ...Object.fromEntries([0, 60, 120, 180, 240, 300].map((hue) => [`hue ${hue}`, `oklch(0.7 0.17 ${hue})`])),
};

const SURFACES = ["--yw-bg", "--yw-surface", "--yw-surface-raised", "--yw-tone-1", "--yw-tone-2"];
const TONES = ["ok", "warn", "danger", "info", "active", "neutral"];

/** `[foreground, ...layers painted beneath it, bottom first]` */
const PAIRS: string[][] = [
  ...SURFACES.flatMap((surface) => ["--yw-fg", "--yw-fg-muted", "--yw-fg-subtle"].map((fg) => [fg, surface])),
  ["--yw-accent-text", "--yw-surface"],
  ["--yw-accent-text", "--yw-bg"],
  ["--yw-accent-text", "--yw-surface", "--yw-accent-soft"],
  ["--yw-on-solid", "--yw-accent-solid"],
  ["--yw-on-solid", "--yw-accent-solid-hover"],
  ...TONES.flatMap((tone) => [
    [`--yw-${tone}`, "--yw-surface"],
    [`--yw-${tone}`, "--yw-bg"],
    [`--yw-${tone}`, "--yw-surface", `--yw-${tone}-soft`],
    ["--yw-on-solid", `--yw-${tone}`],
  ]),
];

for (const theme of ["light", "dark"] as Theme[]) {
  test(`text tokens meet AA contrast for every accent: ${theme}`, async ({ page, example }) => {
    await example("colour/neutrals", { theme });

    const failures = await page.evaluate(
      ({ accents, pairs, theme, AA }) => {
        const probe = document.createElement("div");
        probe.setAttribute("data-yw-scope", "");
        probe.setAttribute("data-theme", theme);
        document.body.append(probe);
        const canvas = new OffscreenCanvas(1, 1).getContext("2d", { willReadFrequently: true })!;

        // Paints the layers over each other and reads back the resulting sRGB.
        const paint = (tokens: string[]) => {
          canvas.clearRect(0, 0, 1, 1);
          for (const token of tokens) {
            probe.style.color = `var(${token})`;
            canvas.fillStyle = getComputedStyle(probe).color;
            canvas.fillRect(0, 0, 1, 1);
          }
          return canvas.getImageData(0, 0, 1, 1).data;
        };
        const luminance = ([r, g, b]: Uint8ClampedArray) => {
          const [lr, lg, lb] = [r, g, b].map((c) => {
            const s = c / 255;
            return s <= 0.04045 ? s / 12.92 : ((s + 0.055) / 1.055) ** 2.4;
          });
          return 0.2126 * lr + 0.7152 * lg + 0.0722 * lb;
        };

        const failures: string[] = [];
        for (const [name, accent] of Object.entries(accents)) {
          probe.style.setProperty("--yw-accent", accent);
          for (const [foreground, ...layers] of pairs) {
            const [a, b] = [luminance(paint([...layers, foreground])), luminance(paint(layers))];
            const ratio = (Math.max(a, b) + 0.05) / (Math.min(a, b) + 0.05);
            if (ratio < AA) failures.push(`${name}: ${foreground} on ${layers.join(" + ")} is ${ratio.toFixed(2)}:1`);
          }
        }
        probe.remove();
        return failures;
      },
      { accents: ACCENTS, pairs: PAIRS, theme, AA },
    );

    expect(failures).toEqual([]);
  });
}
