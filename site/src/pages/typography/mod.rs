crate::page! {
    title: "Typography",
    category: Foundations,
    summary: "Montserrat for headings, Inter for text and controls, Fira Code for code. Size, weight and spacing carry the hierarchy before colour does.",
    api: [],
    examples: [
        headings => "Headings": "Weight falls as size rises, so a large title reads as calm rather than loud. Only headings at body size reach semibold.",
        scale => "Type scale": "Eight sizes. `--yi-text-md` (14px) is the default for controls and dense UI, `--yi-text-lg` for longer reading text.",
        code => "Code and numbers": "Fira Code with ligatures off by default; set `--yi-font-mono-ligatures: normal` to turn them on. Use `yi-tabular` wherever numbers line up in columns.",
    ],
}
