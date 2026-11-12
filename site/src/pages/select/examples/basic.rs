use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let range = use_state(|| AttrValue::from("7d"));
    let on_change = {
        let range = range.clone();
        Callback::from(move |value: AttrValue| range.set(value))
    };
    let options = use_memo((), |_| {
        SelectOptions::from([
            SelectOption::new("24h", "Last 24 hours"),
            SelectOption::new("7d", "Last 7 days"),
            SelectOption::new("30d", "Last 30 days"),
            SelectOption::new("90d", "Last 90 days"),
            SelectOption::new("12m", "Last 12 months"),
        ])
    });

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 18rem" }}>
            <Field label="Date range">
                <Select {options} value={(*range).clone()} {on_change} />
            </Field>
            <span role="status">{ format!("Showing {}", *range) }</span>
        </Stack>
    }
}
