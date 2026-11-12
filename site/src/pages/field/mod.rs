crate::page! {
    title: "Field",
    category: Form,
    summary: "Gives a control its label, hint and error, and connects them for assistive technology. Every form control goes in one.",
    examples: [
        basic => "Label, hint and error": "The label names the control and the hint says what is expected. An error replaces the hint, marks the control invalid and is announced. Fields display errors; the caller decides them.",
        states => "Required and disabled": "`required` adds the marker and sets the native attribute on the control. `disabled` on the field disables whatever control is inside it.",
        inline => "Inline layout": "`FieldLayout::Inline` puts labels in a fixed column, which suits settings pages with short controls. It stacks again when the field gets too narrow. Set the column with `--yi-field-label-width`.",
        hidden_label => "Hidden label": "A control still needs a name when its purpose is clear from context. `hide_label` keeps the label for screen readers and removes it from view.",
        group => "Labelling a group": "With `group` the label names several controls at once and each control carries its own accessible name.",
        custom_control => "Custom controls": "`use_field_control` is how a control joins a field: it returns the id, description and state to put on the focusable element. Every yewi form control uses it, and an app's own controls can too.",
    ],
}
