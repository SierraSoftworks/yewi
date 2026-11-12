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
        TreeOptions::from([
            TreeOption::new(["Europe", "Ireland"], "ie").detail("IE"),
            TreeOption::new(["Europe", "Germany"], "de").detail("DE"),
            TreeOption::new(["Americas", "Brazil"], "br").detail("BR"),
        ])
    });
    let select = |id: &'static str, caption: &'static str, size: Size, disabled: bool, invalid: bool| {
        let control = html! { <TreeSelect {id} options={options.clone()} value="ie" {size} {disabled} {invalid} /> };
        labelled(id, caption, control)
    };

    html! {
        <Grid min="14rem">
            { select("tree-small", "Small", Size::Small, false, false) }
            { select("tree-medium", "Medium", Size::Medium, false, false) }
            { select("tree-large", "Large", Size::Large, false, false) }
            { labelled("tree-empty", "Empty", html! {
                <TreeSelect id="tree-empty" options={options.clone()} placeholder="Choose a country" />
            }) }
            { select("tree-disabled", "Disabled", Size::Medium, true, false) }
            { select("tree-invalid", "Invalid", Size::Medium, false, true) }
            { labelled("tree-unlisted", "A value the tree does not list", html! {
                <TreeSelect id="tree-unlisted" options={options.clone()} value="xk" />
            }) }
            { labelled("tree-fit", "Fit, submitted as country", html! {
                <div><TreeSelect id="tree-fit" options={options.clone()} value="br" fit=true name="country" /></div>
            }) }
        </Grid>
    }
}
