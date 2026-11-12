use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let advanced = use_state(|| false);
    let on_toggle = {
        let advanced = advanced.clone();
        Callback::from(move |open| advanced.set(open))
    };

    html! {
        <Stack>
            <Card title="Rollout policy" attrs={attrs! { "style" => "max-inline-size: 30rem" }}>
                <Panel title="Schedule" subtitle="Weekdays, 22:00 to 04:00" collapsible=true>
                    { "Rollouts start inside the maintenance window and finish the stage they are on." }
                </Panel>
                <Panel title="Advanced" subtitle="Health checks and timeouts" collapsible=true collapsed=true {on_toggle}>
                    { "A host must report healthy for 5 minutes before the next one starts." }
                </Panel>
            </Card>
            <span role="status">
                { if *advanced { "Advanced settings are shown" } else { "Advanced settings are hidden" } }
            </span>
        </Stack>
    }
}
