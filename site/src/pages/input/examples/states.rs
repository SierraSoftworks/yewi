use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Grid min="14rem">
            <Field label="Empty">
                <TextInput placeholder="Placeholder" />
            </Field>
            <Field label="Filled">
                <TextInput value="eu-west-1" />
            </Field>
            <Field label="Read-only">
                <TextInput value="eu-west-1" readonly=true />
            </Field>
            <Field label="Disabled">
                <TextInput value="eu-west-1" disabled=true />
            </Field>
            <Field label="Invalid" error="Unknown region.">
                <TextInput value="eu-west-9" />
            </Field>
            <Field label="Invalid number" error="Must be at least 1.">
                <NumberInput value={0.0} min={1.0} />
            </Field>
        </Grid>
    }
}
