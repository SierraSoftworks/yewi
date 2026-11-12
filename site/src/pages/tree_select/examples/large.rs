use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let rack = use_state(|| None::<AttrValue>);
    let on_change = {
        let rack = rack.clone();
        Callback::from(move |value: Option<AttrValue>| rack.set(value))
    };
    // Six thousand racks: 5 regions, 4 sites, 6 halls, 50 racks.
    let options = use_memo((), |_| {
        let regions = ["eu-west", "eu-central", "us-east", "us-west", "ap-south"];
        (0..6000)
            .map(|n| {
                let (region, site, hall, rack) = (regions[n / 1200], n / 300 % 4 + 1, n / 50 % 6 + 1, n % 50 + 1);
                let code = format!("{region}-{site}-h{hall}-r{rack:02}");
                TreeOption::new(
                    [region.to_string(), format!("Site {site}"), format!("Hall {hall}"), format!("Rack {rack:02}")],
                    code.clone(),
                )
                .detail(code)
            })
            .collect::<TreeOptions>()
    });

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 22rem" }}>
            <Field label="Rack">
                <TreeSelect {options} value={(*rack).clone()} {on_change} placeholder="Search 6,000 racks" />
            </Field>
            <span role="status">
                { rack.as_ref().map_or_else(|| "No rack chosen".to_string(), |rack| format!("Rack {rack}")) }
            </span>
        </Stack>
    }
}
