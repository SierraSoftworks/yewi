use yew::prelude::*;
use yewi::prelude::*;

const SERVICES: [&str; 6] = ["billing-api", "checkout-api", "checkout-web", "identity", "search-indexer", "webhooks"];

#[function_component]
pub fn Example() -> Html {
    let query = use_state(AttrValue::default);
    let on_input = {
        let query = query.clone();
        Callback::from(move |value: String| query.set(value.into()))
    };

    let needle = query.trim().to_lowercase();
    let matches: Vec<&str> = SERVICES.into_iter().filter(|name| name.contains(&needle)).collect();

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 22rem" }}>
            <SearchInput value={(*query).clone()} {on_input} label="Search services" placeholder="Search services" />
            <span role="status">{ format!("{} of {} services", matches.len(), SERVICES.len()) }</span>
            <ul style="margin: 0; padding-inline-start: 1.25rem">
                { for matches.iter().map(|name| html! { <li key={*name}>{ *name }</li> }) }
            </ul>
        </Stack>
    }
}
