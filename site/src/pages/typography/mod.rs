crate::page! {
    title: "Typography",
    category: Foundations,
    summary: "The type uses Montserrat for headings, Inter for text and controls, and Fira Code for code. Size, weight and spacing set the hierarchy before colour comes into it.",
    api: [],
    examples: [
        headings => "Headings": "Weight goes down as size goes up, so a large title reads as calm rather than loud. Only headings at body size go as heavy as semibold.",
        scale => "Type scale": "The type scale has eight sizes. `--yi-text-md` (14px) is the default for controls and dense UI, and `--yi-text-lg` is for longer reading text.",
        code => "Code and numbers": "Code is set in Fira Code, with ligatures turned off by default. Set `--yi-font-mono-ligatures: normal` to turn them on. Use `yi-tabular` wherever numbers need to line up in columns.",
    ],
}
