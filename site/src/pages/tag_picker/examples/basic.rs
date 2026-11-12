use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let labels = use_state(|| vec![AttrValue::from("bug"), AttrValue::from("ui")]);
    let on_change = {
        let labels = labels.clone();
        Callback::from(move |values: Vec<AttrValue>| labels.set(values))
    };
    let options = use_memo((), |_| {
        SelectOptions::from([
            SelectOption::new("bug", "Bug").description("Something is not working"),
            SelectOption::new("docs", "Documentation").description("Missing or unclear docs"),
            SelectOption::new("feature", "Feature").description("A new capability"),
            SelectOption::new("perf", "Performance").description("Slow or heavy"),
            SelectOption::new("security", "Security").description("Needs a private report").disabled(true),
            SelectOption::new("ui", "Interface").description("Looks or behaves oddly"),
        ])
    });

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 24rem" }}>
            <Field label="Labels">
                <TagPicker {options} values={(*labels).clone()} {on_change} placeholder="Add labels" />
            </Field>
            <span role="status">{ format!("Labels: {}", labels.join(", ")) }</span>
        </Stack>
    }
}
