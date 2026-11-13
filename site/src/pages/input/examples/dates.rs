use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let starts = use_state(|| AttrValue::from("2026-01-15T13:30"));
    let on_change = {
        let starts = starts.clone();
        Callback::from(move |value: String| starts.set(value.into()))
    };

    html! {
        <Stack>
            <Grid min="14rem">
                <Field label="Starts" hint="In your own time zone.">
                    <TextInput kind={TextInputKind::DateTime} value={(*starts).clone()} {on_change} />
                </Field>
                <Field label="Day">
                    <TextInput kind={TextInputKind::Date} value="2026-01-15" />
                </Field>
                <Field label="Daily at">
                    <TextInput kind={TextInputKind::Time} value="09:00" />
                </Field>
            </Grid>
            <span role="status">{ format!("Starts {}", *starts) }</span>
        </Stack>
    }
}
