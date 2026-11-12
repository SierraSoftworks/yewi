use yew::prelude::*;
use yewi::prelude::*;

/// Draws a map symbol from its key: a frame shape and a status colour.
fn symbol(key: AttrValue) -> Html {
    let (shape, tone) = key.split_once(':').unwrap_or(("unit", "info"));
    let style = format!("color: var(--yi-{tone})");
    let frame = match shape {
        "air" => html! { <path d="M4 19V11a8 8 0 0 1 16 0v8z" /> },
        "sea" => html! { <circle cx="12" cy="12" r="8" /> },
        "point" => html! { <path d="M12 3l9 9-9 9-9-9z" /> },
        _ => html! { <rect x="3" y="6" width="18" height="12" /> },
    };
    html! {
        <svg viewBox="0 0 24 24" width="24" height="24" fill="currentColor" fill-opacity="0.2"
            stroke="currentColor" stroke-width="1.5" stroke-linejoin="round" aria-hidden="true" {style}>
            { frame }
        </svg>
    }
}

#[function_component]
pub fn Example() -> Html {
    let kind = use_state(|| Some(AttrValue::from("a-f-G-U-C-I")));
    let on_change = {
        let kind = kind.clone();
        Callback::from(move |value: Option<AttrValue>| kind.set(value))
    };
    let options = use_memo((), |_| {
        let entry = |path: &[&'static str], code: &'static str, media: &'static str| {
            TreeOption::new(path.iter().copied(), code).detail(code).media(media)
        };
        TreeOptions::from([
            TreeOption::branch(["Ground"]).icon(icons::MOUNTAIN),
            entry(&["Ground", "Unit"], "a-f-G-U", "unit:info"),
            entry(&["Ground", "Unit", "Infantry"], "a-f-G-U-C-I", "unit:info"),
            entry(&["Ground", "Unit", "Armour"], "a-f-G-U-C-A", "unit:info"),
            entry(&["Ground", "Unit", "Medical"], "a-f-G-U-S-M", "unit:ok"),
            entry(&["Ground", "Hostile unit"], "a-h-G-U", "point:danger"),
            TreeOption::branch(["Air"]).icon(icons::PLANE),
            entry(&["Air", "Fixed wing"], "a-f-A-M-F", "air:info"),
            entry(&["Air", "Rotary wing"], "a-f-A-M-H", "air:info"),
            entry(&["Air", "Unknown aircraft"], "a-u-A", "air:warn"),
            TreeOption::branch(["Sea"]).icon(icons::WAVES),
            entry(&["Sea", "Surface vessel"], "a-f-S", "sea:info"),
            entry(&["Sea", "Neutral vessel"], "a-n-S", "sea:ok"),
        ])
    });

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 22rem" }}>
            <Field label="Marker type">
                <TreeSelect
                    {options}
                    value={(*kind).clone()}
                    {on_change}
                    render_media={Callback::from(symbol)}
                    search_label="Search by name or code"
                />
            </Field>
            <span role="status">
                { kind.as_ref().map_or_else(|| "No type".to_string(), |kind| format!("Marker type {kind}")) }
            </span>
        </Stack>
    }
}
