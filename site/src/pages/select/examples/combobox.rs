use yew::prelude::*;
use yewi::prelude::*;

const ZONES: [(&str, &str); 10] = [
    ("Africa/Johannesburg", "UTC+02:00"),
    ("America/Los_Angeles", "UTC-08:00"),
    ("America/New_York", "UTC-05:00"),
    ("America/Sao_Paulo", "UTC-03:00"),
    ("Asia/Kolkata", "UTC+05:30"),
    ("Asia/Tokyo", "UTC+09:00"),
    ("Australia/Sydney", "UTC+11:00"),
    ("Europe/Dublin", "UTC+00:00"),
    ("Europe/London", "UTC+00:00"),
    ("Europe/Berlin", "UTC+01:00"),
];

#[function_component]
pub fn Example() -> Html {
    let zone = use_state(|| AttrValue::from("Europe/Dublin"));
    let on_change = {
        let zone = zone.clone();
        Callback::from(move |value: AttrValue| zone.set(value))
    };
    let options = use_memo((), |_| {
        ZONES
            .into_iter()
            .map(|(zone, offset)| SelectOption::new(zone, zone.replace('_', " ")).description(offset))
            .collect::<SelectOptions>()
    });

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 18rem" }}>
            <Field label="Time zone">
                <Combobox {options} value={(*zone).clone()} {on_change} placeholder="Search time zones" />
            </Field>
            <span role="status">{ format!("Times shown in {}", *zone) }</span>
        </Stack>
    }
}
