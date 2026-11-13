use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Steps orientation={Orientation::Vertical} label="Release checklist">
            <Step title="Build" status={StepStatus::Done} description="Finished successfully." />
            <Step title="Unit tests" status={StepStatus::Failed} description="Finished with an error." />
            <Step title="Lint" status={StepStatus::Skipped} description="Deliberately not run." />
            <Step title="Review" status={StepStatus::Current} description="Where the user is now." />
            <Step title="Deploy to staging" status={StepStatus::Running} description="Work is in flight." />
            <Step title="Approval" status={StepStatus::Blocked} description="Held until something else happens." />
            <Step title="Deploy to production" status={StepStatus::Pending} description="Not reached yet." />
        </Steps>
    }
}
