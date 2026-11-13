crate::page! {
    title: "Icon",
    category: Basic,
    summary: "Icons are stroked line drawings that inherit the text colour and scale with the text around them.",
    examples: [
        basic => "Basic": "Icons are constants in `yewi::icons`, which holds the Lucide set. Only the icons your app names are compiled into it.",
        sizes => "Size and colour": "By default an icon is `1.15em` square and follows `color`. When it stands alone, set `size` to any CSS length.",
        labelled => "Meaningful icons": "An icon without a `label` is decorative and hidden from assistive technology. Give it a `label` when it carries meaning on its own.",
        custom => "Custom icons": "You can define your own icons with `IconData::new`. Give it path data on a 24 unit grid, and it's drawn as a stroke.",
        gallery => "Gallery": "Search the bundled set here. Each name maps to a constant, so `arrow-right` is `icons::ARROW_RIGHT`.",
    ],
}
