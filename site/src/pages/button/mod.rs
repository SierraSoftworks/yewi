crate::page! {
    title: "Button",
    category: Basic,
    summary: "Buttons trigger actions. Use one primary button per view, and keep the other buttons quieter so the main action stands out.",
    examples: [
        variants => "Variants": "`Primary` is for the single main action of a view. `Default` suits most actions, `Subtle` suits toolbars and repeated row actions, and `Danger` marks something destructive.",
        sizes => "Sizes": "`Medium` is the default, at 32px. Use `Small` in dense rows and toolbars, and `Large` for a lone call to action.",
        with_icon => "With an icon": "An icon goes before the label and inherits its colour. Icons support the label, but they don't replace it.",
        states => "Disabled and loading": "A disabled button can't be focused or clicked. A loading button keeps focus and announces that it's busy, and it ignores clicks until the work finishes.",
        link => "As a link": "When you set `href`, the button renders a real link, so people can open it in a new tab and follow it without script.",
        icon_only => "Icon buttons": "`IconButton` is a square button that shows only an icon. Use it in toolbars and repeated row actions where the icon is familiar. `label` is required, because it names the button for screen readers and appears on hover.",
        toggle => "Toggle buttons": "`pressed` turns a `Button` or `IconButton` into a toggle that reports its state. Keep the label the same in both states, and let the pressed state show the change.",
        group => "Button group": "`ButtonGroup` joins several actions on the same thing into a single control. Give it a `label` that says what the actions apply to. Each button is still its own tab stop.",
        group_toolbar => "Icon and vertical groups": "Groups can hold icon buttons and mixed sizes. `Orientation::Vertical` stacks them, which suits controls docked to an edge, such as map zoom.",
    ],
}
