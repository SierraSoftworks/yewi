use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let approved = use_state(|| false);
    let approve = {
        let approved = approved.clone();
        Callback::from(move |_| approved.set(true))
    };

    let production_gate = if *approved {
        html! { <PipelineGate label="approved" state={GateState::Open} /> }
    } else {
        html! { <PipelineGate label="needs approval" state={GateState::Closed} /> }
    };

    html! {
        <Stack gap={Space::Lg}>
            <Pipeline label="Deployment stages">
                <PipelineStage title="Staging" status="done" meta={html! { "deployed 14:02" }} />
                <PipelineStage
                    title="Canary"
                    status="done"
                    meta={html! { "5% of traffic" }}
                    gate={html! { <PipelineGate label="checks passed" state={GateState::Open} /> }}
                />
                <PipelineStage
                    title="Production"
                    status={if *approved { "rolling out" } else { "held" }}
                    current=true
                    meta={html! { "100% of traffic" }}
                    gate={production_gate}
                >
                    <Button variant={ButtonVariant::Primary} size={Size::Small} disabled={*approved} on_click={approve}>
                        { "Approve" }
                    </Button>
                </PipelineStage>
                <PipelineStage
                    title="Cleanup"
                    status="waiting"
                    meta={html! { "removes the old release" }}
                    gate={html! { <PipelineGate label="bake 30m" /> }}
                />
                <PipelineStage
                    title="Announce"
                    status="paused"
                    meta={html! { "posts the changelog" }}
                    gate={html! {
                        <PipelineGate label="paused" state={GateState::Closed} tone={Tone::Warn} icon={icons::PAUSE} />
                    }}
                />
            </Pipeline>
            <span role="status">
                { if *approved { "Production approved" } else { "Production is waiting for approval" } }
            </span>
        </Stack>
    }
}
