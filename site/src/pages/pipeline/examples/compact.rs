use yew::prelude::*;
use yewi::prelude::*;

fn units(done: usize, total: usize, stage: &str) -> Html {
    let units = (0..total).map(|i| {
        let (state, tone, fill) =
            if i < done { ("done", Tone::Ok, UnitFill::Solid) } else { ("pending", Tone::Neutral, UnitFill::Hollow) };
        html! { <PipelineUnit label={format!("{stage} {}: {state}", i + 1)} {tone} {fill} /> }
    });
    html! { <PipelineUnits label={format!("Hosts in {stage}")}>{ for units }</PipelineUnits> }
}

#[function_component]
pub fn Example() -> Html {
    html! {
        <Pipeline label="Rollout overview" compact=true fill=true>
            <PipelineStage title="canary" status="2/2" meta={html! { "hidden when compact" }}>
                { units(2, 2, "canary") }
            </PipelineStage>
            <PipelineStage
                title="edge"
                status="7/12"
                current=true
                gate={html! { <PipelineGate label="approved" state={GateState::Open} /> }}
            >
                { units(7, 12, "edge") }
            </PipelineStage>
            <PipelineStage title="core" status="0/20" gate={html! { <PipelineGate label="approval" /> }}>
                { units(0, 20, "core") }
            </PipelineStage>
        </Pipeline>
    }
}
