use yew::prelude::*;
use yewi::prelude::*;

/// A caption pointing at the control with this id.
fn labelled(id: &'static str, caption: &'static str, control: Html) -> Html {
    html! {
        <Stack gap={Space::Xs}>
            <label for={id} style="font-size: var(--yi-text-sm); font-weight: 500">{ caption }</label>
            { control }
        </Stack>
    }
}

#[function_component]
pub fn Example() -> Html {
    let options = use_memo((), |_| {
        SelectOptions::from([
            SelectOption::new("eu-west-1", "Ireland"),
            SelectOption::new("eu-central-1", "Frankfurt"),
            SelectOption::new("us-east-1", "Virginia"),
        ])
    });
    let select = |id: &'static str, caption: &'static str, size: Size, disabled: bool, invalid: bool| {
        let control = html! { <Select {id} options={options.clone()} value="eu-west-1" {size} {disabled} {invalid} /> };
        labelled(id, caption, control)
    };
    let combobox = |id: &'static str, caption: &'static str, disabled: bool, invalid: bool| {
        let control = html! { <Combobox {id} options={options.clone()} value="eu-central-1" {disabled} {invalid} /> };
        labelled(id, caption, control)
    };

    html! {
        <Grid min="14rem">
            { select("states-small", "Small", Size::Small, false, false) }
            { select("states-medium", "Medium", Size::Medium, false, false) }
            { select("states-large", "Large", Size::Large, false, false) }
            { labelled("states-empty", "Empty", html! {
                <Select id="states-empty" options={options.clone()} placeholder="Choose a region" />
            }) }
            { select("states-disabled", "Disabled", Size::Medium, true, false) }
            { select("states-invalid", "Invalid", Size::Medium, false, true) }
            { combobox("states-combobox", "Combobox", false, false) }
            { combobox("states-combobox-disabled", "Combobox, disabled", true, false) }
            { combobox("states-combobox-invalid", "Combobox, invalid", false, true) }
        </Grid>
    }
}
