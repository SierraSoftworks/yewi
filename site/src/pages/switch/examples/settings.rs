use yew::prelude::*;
use yewi::prelude::*;

const SETTINGS: [(&str, &str); 3] = [
    ("Automatic rollback", "Roll back when health checks fail after a deploy."),
    ("Require approval", "Production deploys wait for a second person."),
    ("Weekend freeze", "Block deploys from Friday 18:00 to Monday 08:00."),
];

#[function_component]
pub fn Example() -> Html {
    let enabled = use_state(|| [true, true, false]);

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 32rem" }}>
            { for SETTINGS.into_iter().enumerate().map(|(index, (label, hint))| {
                let on_change = {
                    let enabled = enabled.clone();
                    Callback::from(move |checked| {
                        let mut next = *enabled;
                        next[index] = checked;
                        enabled.set(next);
                    })
                };
                html! {
                    <Field {label} {hint} layout={FieldLayout::Inline}>
                        <ToggleSwitch checked={enabled[index]} {on_change} />
                    </Field>
                }
            }) }
            <span role="status">{ format!("{} of 3 enabled", enabled.iter().filter(|on| **on).count()) }</span>
        </Stack>
    }
}
