use yew::prelude::*;
use yewi::prelude::*;

/// An inventory of 5,000 packages: about 20,000 values.
fn inventory() -> Json {
    let packages: Vec<String> = (0..5000)
        .map(|i| format!(r#"{{"name": "pkg-{i:04}", "version": "{}.{}.{}", "auto": {}}}"#, i % 7, i % 13, i % 5, i % 3 == 0))
        .collect();
    let text = format!(r#"{{"host": "edge-fra-02", "collected": 5000, "packages": [{}]}}"#, packages.join(","));
    Json::parse(&text).unwrap_or(Json::Null)
}

#[function_component]
pub fn Example() -> Html {
    let inventory = use_memo((), |_| inventory());

    html! {
        <JsonView
            value={inventory}
            name="inventory"
            open=2
            page_size=50
            max_height="20rem"
            attrs={attrs! { "style" => "max-inline-size: 34rem" }}
        />
    }
}
