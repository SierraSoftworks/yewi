use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let status = use_state(|| AttrValue::from("all"));
    let range = use_state(|| AttrValue::from("7d"));
    let set = |state: &UseStateHandle<AttrValue>| {
        let state = state.clone();
        Callback::from(move |value: AttrValue| state.set(value))
    };
    let statuses = use_memo((), |_| {
        SelectOptions::from([
            SelectOption::new("all", "All statuses"),
            SelectOption::new("passing", "Passing"),
            SelectOption::new("failing", "Failing"),
        ])
    });
    let ranges = use_memo((), |_| {
        SelectOptions::from([
            SelectOption::new("24h", "Last 24 hours"),
            SelectOption::new("7d", "Last 7 days"),
            SelectOption::new("30d", "Last 30 days"),
        ])
    });

    html! {
        <Stack>
            <Cluster>
                <Select
                    fit=true
                    size={Size::Small}
                    label="Status"
                    options={statuses}
                    value={(*status).clone()}
                    on_change={set(&status)}
                />
                <Select
                    fit=true
                    size={Size::Small}
                    label="Date range"
                    options={ranges}
                    value={(*range).clone()}
                    on_change={set(&range)}
                />
                <Button size={Size::Small} icon={icons::REFRESH_CW}>{ "Refresh" }</Button>
            </Cluster>
            <span role="status">{ format!("Status {}, range {}", *status, *range) }</span>
        </Stack>
    }
}
