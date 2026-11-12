use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let layout = FieldLayout::Inline;
    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 32rem" }}>
            <Field {layout} label="Display name">
                <TextInput value="Production" />
            </Field>
            <Field {layout} label="Webhook URL" hint="Called after every rollout." required=true>
                <TextInput kind={TextInputKind::Url} placeholder="https://" />
            </Field>
            <Field {layout} label="Retries" error="Must be between 0 and 10.">
                <NumberInput value={12.0} min={0.0} max={10.0} />
            </Field>
        </Stack>
    }
}
