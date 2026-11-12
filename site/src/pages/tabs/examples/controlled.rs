use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let selected = use_state(|| AttrValue::from("summary"));
    let select = |value: &'static str| {
        let selected = selected.clone();
        Callback::from(move |_| selected.set(value.into()))
    };
    let on_change = {
        let selected = selected.clone();
        Callback::from(move |value: AttrValue| selected.set(value))
    };

    html! {
        <Stack>
            <Cluster>
                <Button on_click={select("summary")}>{ "Open summary" }</Button>
                <Button on_click={select("notes")}>{ "Open notes" }</Button>
            </Cluster>
            <Tabs label="Incident" selected={(*selected).clone()} manual=true keep_mounted=true {on_change}>
                <Tab value="summary" label="Summary">{ "What happened and who is on it." }</Tab>
                <Tab value="timeline" label="Timeline">{ "Events in the order they were seen." }</Tab>
                <Tab value="notes" label="Notes">
                    <label>
                        { "Notes " }
                        <input type="text" placeholder="Kept while you look at other tabs" />
                    </label>
                </Tab>
            </Tabs>
            <span role="status">{ format!("Selected: {}", *selected) }</span>
        </Stack>
    }
}
