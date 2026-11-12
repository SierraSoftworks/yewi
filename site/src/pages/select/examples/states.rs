use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let options = use_memo((), |_| {
        SelectOptions::from([
            SelectOption::new("eu-west-1", "Ireland"),
            SelectOption::new("eu-central-1", "Frankfurt"),
            SelectOption::new("us-east-1", "Virginia"),
        ])
    });

    html! {
        <Grid min="14rem">
            <Field label="Small">
                <Select options={options.clone()} value="eu-west-1" size={Size::Small} />
            </Field>
            <Field label="Medium">
                <Select options={options.clone()} value="eu-west-1" />
            </Field>
            <Field label="Large">
                <Select options={options.clone()} value="eu-west-1" size={Size::Large} />
            </Field>
            <Field label="Empty">
                <Select options={options.clone()} placeholder="Choose a region" />
            </Field>
            <Field label="Disabled">
                <Select options={options.clone()} value="eu-west-1" disabled=true />
            </Field>
            <Field label="Invalid" error="This region is being retired.">
                <Select options={options.clone()} value="us-east-1" />
            </Field>
            <Field label="Combobox">
                <Combobox options={options.clone()} value="eu-central-1" />
            </Field>
            <Field label="Combobox, disabled">
                <Combobox options={options.clone()} value="eu-central-1" disabled=true />
            </Field>
            <Field label="Combobox, invalid" error="Choose a region.">
                <Combobox {options} placeholder="Search regions" />
            </Field>
        </Grid>
    }
}
