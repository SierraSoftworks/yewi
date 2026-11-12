use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let topic = use_state(|| Some(AttrValue::from("sensors/roof/wind")));
    let on_change = {
        let topic = topic.clone();
        Callback::from(move |value: Option<AttrValue>| topic.set(value))
    };
    let options = use_memo((), |_| {
        TreeOptions::from([
            TreeOption::new(["Sensors", "Kitchen", "Temperature"], "sensors/kitchen/temperature"),
            TreeOption::new(["Sensors", "Kitchen", "Humidity"], "sensors/kitchen/humidity"),
            TreeOption::new(["Sensors", "Garage", "Door"], "sensors/garage/door"),
            TreeOption::new(["Alerts", "Smoke"], "alerts/smoke"),
            TreeOption::new(["Alerts", "Flood"], "alerts/flood"),
        ])
    });
    // Any topic of two or more lowercase segments is well formed.
    let custom = Callback::from(|typed: String| {
        let well_formed = typed.split('/').count() >= 2
            && typed.split('/').all(|part| !part.is_empty() && part.chars().all(|c| c.is_ascii_lowercase() || c == '-'));
        well_formed.then(|| AttrValue::from(typed))
    });

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 20rem" }}>
            <Field label="Topic" hint="Pick a known topic or type one, such as sensors/attic/light">
                <TreeSelect
                    {options}
                    value={(*topic).clone()}
                    {on_change}
                    {custom}
                    clear_label="Every topic"
                    placeholder="Every topic"
                />
            </Field>
            <span role="status">
                { topic.as_ref().map_or_else(|| "Subscribed to everything".to_string(), |topic| format!("Subscribed to {topic}")) }
            </span>
        </Stack>
    }
}
