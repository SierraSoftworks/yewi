crate::page! {
    title: "Button",
    category: Basic,
    summary: "Triggers an action. One primary button per view; everything else stays quiet.",
    examples: [
        variants => "Variants": "`Primary` is for the single main action of a view. `Default` suits most actions, `Subtle` suits toolbars and repeated row actions, and `Danger` marks something destructive.",
        sizes => "Sizes": "`Medium` is the 32px default. Use `Small` in dense rows and toolbars and `Large` for a lone call to action.",
        with_icon => "With an icon": "An icon goes before the label and inherits its colour. Icons support the label; they do not replace it.",
        states => "Disabled and loading": "A disabled button cannot be focused or clicked. A loading button keeps focus and announces that it is busy, and ignores clicks until the work finishes.",
        link => "As a link": "With `href` the button renders a real link, so it can be opened in a new tab and followed without script.",
    ],
}
