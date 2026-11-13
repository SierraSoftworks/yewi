crate::page! {
    title: "Input",
    category: Form,
    summary: "TextInput, TextArea and NumberInput give you outlined, controlled text entry. Put each one in a Field so it has a label.",
    examples: [
        basic => "Text input": "The input is controlled, so it shows `value` and reports each edit through `on_input`. `on_change` fires once the user commits the text, on blur or Enter.",
        on_blur => "Tidying on blur": "`on_blur_value` fires with the text whenever focus leaves the input, whether or not it changed, which is the moment to trim or validate it. `on_blur` gives the event itself.",
        kinds => "Kinds": "`kind` sets the native input type, which picks the on-screen keyboard and how autofill behaves. `mono` suits identifiers, tokens and paths.",
        dates => "Dates and times": "The `Date`, `Time` and `DateTime` kinds use the browser's own pickers and work without a client runtime. Whatever the browser shows, the value is ISO text such as `2026-01-15T13:30`, with no time zone.",
        sizes => "Sizes": "Sizes match `Button`, so inputs and buttons line up in a row. `Medium` is the default, at 32px.",
        states => "States": "Disabled inputs can't be focused and aren't submitted. Read-only inputs can be focused and copied from. `invalid` marks a problem, and inside a `Field` its `error` does this for you.",
        text_area => "Text area": "`rows` sets the height. `mono` also stops lines from wrapping, for values whose alignment matters.",
        autosize => "Growing with content": "`TextAreaResize::Auto` starts at `rows` lines and grows as the user types. You can cap its height with `--yi-input-max-height`.",
        number => "Number input": "An empty field is `None`, not zero. The arrow keys step the value, Page Up and Page Down step it by ten, and Home and End jump to the limits. Typed values are clamped to `min` and `max` on blur.",
        stepper => "Stepper": "By default, decrease and increase chips sit inside the field and stop at the limits, and `step` can be fractional. Set `stepper=false` for numbers people type rather than nudge, such as a port.",
        form => "In a form": "Inputs are native elements with a `name`, so a plain `<form>` submits them, and pressing Enter in any single-line input submits the form.",
    ],
}
