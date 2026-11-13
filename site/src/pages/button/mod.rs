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
        icon_only => "Icon buttons": "`IconButton` is a square button showing only an icon, for toolbars and repeated row actions where the icon is familiar. `label` is required: it names the button for screen readers and appears on hover.",
        toggle => "Toggle buttons": "`pressed` turns a `Button` or `IconButton` into a toggle that reports its state. Keep the label the same in both states and let the pressed state carry the change.",
        group => "Button group": "`ButtonGroup` joins several actions on one thing into a single control. Give it a `label` saying what the actions apply to. Each button remains its own tab stop.",
        group_toolbar => "Icon and vertical groups": "Groups take icon buttons and mixed sizes, and `Orientation::Vertical` stacks them for controls docked to an edge such as map zoom.",
    ],
}
