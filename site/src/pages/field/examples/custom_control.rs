use yew::prelude::*;
use yewi::prelude::*;

#[derive(Properties, PartialEq)]
struct RangeProps {
    value: u8,
    #[prop_or_default]
    disabled: bool,
}

/// An app's own control. The hook makes it behave like a built-in one.
#[function_component]
fn Range(props: &RangeProps) -> Html {
    let control = use_field_control(None, false, false, props.disabled);
    html! {
        <input
            type="range"
            id={control.id.clone()}
            value={props.value.to_string()}
            disabled={control.disabled}
            aria-describedby={control.described_by.clone()}
            aria-invalid={control.aria_invalid()}
        />
    }
}

#[function_component]
pub fn Example() -> Html {
    html! {
        <Field
            label="Rollout speed"
            hint="Share of instances replaced per step."
            attrs={attrs! { "style" => "max-inline-size: 22rem" }}
        >
            <Range value=40 />
        </Field>
    }
}
