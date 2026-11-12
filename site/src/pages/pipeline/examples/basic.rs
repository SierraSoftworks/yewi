use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Pipeline label="Release stages">
            <PipelineStage title="Build" status="done" meta={html! { <><span>{ "3 jobs" }</span><span>{ "2m 14s" }</span></> }}>
                <Steps orientation={StepsOrientation::Vertical} size={Size::Small} label="Build jobs">
                    <Step title="compile" status={StepStatus::Done} />
                    <Step title="package" status={StepStatus::Done} />
                    <Step title="sign" status={StepStatus::Done} />
                </Steps>
            </PipelineStage>
            <PipelineStage title="Test" status="running" current=true meta={html! { <><span>{ "2 jobs" }</span><span>{ "48s so far" }</span></> }}>
                <Steps orientation={StepsOrientation::Vertical} size={Size::Small} label="Test jobs">
                    <Step title="unit" status={StepStatus::Done} />
                    <Step title="integration" status={StepStatus::Running} />
                </Steps>
            </PipelineStage>
            <PipelineStage title="Publish" status="waiting" meta={html! { <><span>{ "1 job" }</span></> }}>
                <Steps orientation={StepsOrientation::Vertical} size={Size::Small} label="Publish jobs">
                    <Step title="upload" />
                </Steps>
            </PipelineStage>
        </Pipeline>
    }
}
