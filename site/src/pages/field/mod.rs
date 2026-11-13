crate::page! {
    title: "Field",
    category: Form,
    summary: "A field gives a control its label, hint and error, and connects them for assistive technology. Every form control should go in one.",
    examples: [
        basic => "Label, hint and error": "The label names the control and the hint says what's expected. An error replaces the hint, marks the control invalid and is announced. The field only displays errors, so it's up to you to decide when there is one.",
        states => "Required and disabled": "`required` adds the required marker and sets the native attribute on the control. `disabled` on the field disables whatever control is inside it.",
        inline => "Inline layout": "`FieldLayout::Inline` puts the labels in a fixed column, which suits settings pages with short controls. The field stacks again when it gets too narrow. You can set the column width with `--yi-field-label-width`.",
        hidden_label => "Hidden label": "A control still needs a name even when its purpose is clear from context. `hide_label` keeps the label for screen readers but removes it from view.",
        group => "Labelling a group": "With `group`, the label names several controls at once, and each control carries its own accessible name.",
        custom_control => "Custom controls": "`use_field_control` is how a control joins a field. It returns the id, description and state to put on the focusable element. Every yewi form control uses it, and your app's own controls can too.",
    ],
}
