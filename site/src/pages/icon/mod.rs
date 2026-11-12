crate::page! {
    title: "Icon",
    category: Basic,
    summary: "A stroked line icon that inherits the text colour and scales with the text around it.",
    examples: [
        basic => "Basic": "Icons are constants in `yewi::icons`, the Lucide set. Only the icons an app names are compiled into it.",
        sizes => "Size and colour": "An icon is `1.15em` square by default and follows `color`. Set `size` to any CSS length when it stands alone.",
        labelled => "Meaningful icons": "An icon without a `label` is decorative and hidden from assistive technology. Give it a `label` when it carries meaning on its own.",
        custom => "Custom icons": "Define app icons with `IconData::new`: path data on a 24 unit grid, drawn as a stroke.",
        gallery => "Gallery": "Search the bundled set. The name maps to the constant: `arrow-right` is `icons::ARROW_RIGHT`.",
    ],
}
