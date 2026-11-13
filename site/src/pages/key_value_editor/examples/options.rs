use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let headers = use_state(|| vec![KeyValue::new("Accept", "application/json")]);
    let on_change = {
        let headers = headers.clone();
        Callback::from(move |rows| headers.set(rows))
    };

    html! {
        <Stack gap={Space::Lg}>
            <Field label="Request headers" group=true hint="Up to three.">
                <KeyValueEditor
                    rows={(*headers).clone()}
                    {on_change}
                    row_label="header"
                    key_placeholder="Header"
                    value_placeholder="Value"
                    separator=""
                    add_label="Add header"
                    max_rows=3
                    attrs={attrs! { "style" => "--yi-key-value-editor-key-width: 12rem" }}
                />
            </Field>
            <Field label="Inherited labels" group=true hint="Set by the cluster, so they can't be changed here.">
                <KeyValueEditor
                    rows={vec![KeyValue::new("cluster", "fra-1"), KeyValue::new("owner", "platform")]}
                    row_label="label"
                    mono=true
                    disabled=true
                />
            </Field>
        </Stack>
    }
}
