use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let site = use_state(|| Some(AttrValue::from("dub-1")));
    let on_change = {
        let site = site.clone();
        Callback::from(move |value: Option<AttrValue>| site.set(value))
    };
    let options = use_memo((), |_| {
        TreeOptions::from([
            TreeOption::new(["Europe", "Ireland", "Dublin 1"], "dub-1"),
            TreeOption::new(["Europe", "Ireland", "Dublin 2"], "dub-2"),
            TreeOption::new(["Europe", "Germany", "Frankfurt"], "fra-1"),
            TreeOption::new(["Europe", "Germany", "Berlin"], "ber-1"),
            TreeOption::new(["Americas", "United States", "Virginia"], "iad-1"),
            TreeOption::new(["Americas", "United States", "Oregon"], "pdx-1"),
            TreeOption::new(["Americas", "Brazil", "Sao Paulo"], "gru-1"),
            TreeOption::new(["Asia Pacific", "Japan", "Tokyo"], "nrt-1"),
            TreeOption::new(["Asia Pacific", "Australia", "Sydney"], "syd-1"),
        ])
    });

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 18rem" }}>
            <Field label="Data centre">
                <TreeSelect {options} value={(*site).clone()} {on_change} placeholder="Choose a data centre" />
            </Field>
            <span role="status">
                { site.as_ref().map_or_else(|| "Nothing chosen".to_string(), |site| format!("Deploying to {site}")) }
            </span>
        </Stack>
    }
}
