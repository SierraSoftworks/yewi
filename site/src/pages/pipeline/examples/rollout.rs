use yew::prelude::*;
use yewi::prelude::*;

/// A host's state in the rollout: its label, tone and fill.
#[derive(Clone, Copy, PartialEq)]
enum Host {
    Healthy,
    Baking,
    Applying,
    Failed,
    Planned,
    Pending,
}

impl Host {
    const LEGEND: [Host; 6] =
        [Host::Pending, Host::Planned, Host::Applying, Host::Baking, Host::Healthy, Host::Failed];

    fn look(self) -> (&'static str, Tone, UnitFill) {
        match self {
            Host::Healthy => ("healthy", Tone::Ok, UnitFill::Solid),
            Host::Baking => ("baking", Tone::Active, UnitFill::Soft),
            Host::Applying => ("applying", Tone::Active, UnitFill::Solid),
            Host::Failed => ("failed", Tone::Danger, UnitFill::Solid),
            Host::Planned => ("planned", Tone::Neutral, UnitFill::Soft),
            Host::Pending => ("pending", Tone::Neutral, UnitFill::Hollow),
        }
    }
}

fn hosts(prefix: &str, states: &[Host]) -> Html {
    let units = states.iter().enumerate().map(|(i, state)| {
        let (label, tone, fill) = state.look();
        let name = format!("{prefix}-{:02}", i + 1);
        html! {
            <PipelineUnit
                label={format!("{name}: {label}")}
                {tone}
                {fill}
                pulse={*state == Host::Baking}
                href={format!("#{name}")}
            />
        }
    });
    html! { <PipelineUnits label={format!("Hosts in {prefix}")} per_row=8>{ for units }</PipelineUnits> }
}

#[function_component]
pub fn Example() -> Html {
    use Host::*;

    html! {
        <Stack>
            <Pipeline label="Rollout stages">
                <PipelineStage
                    title="canary"
                    status="done"
                    meta={html! { <><span>{ "2/2 done" }</span><span>{ "parallel 1" }</span></> }}
                >
                    { hosts("canary", &[Healthy, Healthy]) }
                </PipelineStage>
                <PipelineStage
                    title="edge"
                    status="rolling"
                    current=true
                    meta={html! { <><span>{ "5/12 done" }</span><span>{ "1 failed" }</span><span>{ "bake 4m 10s" }</span></> }}
                    gate={html! { <PipelineGate label="approved" state={GateState::Open} /> }}
                >
                    { hosts("edge", &[
                        Healthy, Healthy, Healthy, Healthy, Healthy, Failed, Baking, Baking,
                        Applying, Planned, Planned, Planned,
                    ]) }
                </PipelineStage>
                <PipelineStage
                    title="core"
                    status="waiting"
                    meta={html! { <><span>{ "0/20 done" }</span><span>{ "parallel 4" }</span></> }}
                    gate={html! { <PipelineGate label="approval required" /> }}
                >
                    { hosts("core", &[Pending; 20]) }
                </PipelineStage>
                <PipelineStage title="Not in this rollout" placeholder=true meta={html! { "registered since" }}>
                    <PipelineUnits label="Hosts not in this rollout">
                        <PipelineUnit label="lab-01: healthy" tone={Tone::Ok} round=true />
                        <PipelineUnit label="lab-02: offline" fill={UnitFill::Dashed} round=true />
                    </PipelineUnits>
                </PipelineStage>
            </Pipeline>
            <Cluster gap={Space::Lg}>
                { for Host::LEGEND.into_iter().map(|state| {
                    let (label, tone, fill) = state.look();
                    html! {
                        <Cluster gap={Space::Xs} align={Align::Center}>
                            <PipelineUnit label={format!("Legend: {label}")} {tone} {fill} />
                            <small>{ label }</small>
                        </Cluster>
                    }
                }) }
            </Cluster>
        </Stack>
    }
}
