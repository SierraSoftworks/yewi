use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let steps = |size: Size, label: &'static str| {
        html! {
            <Steps {size} {label}>
                <Step title="Source" status={StepStatus::Done} icon={icons::GIT_BRANCH} />
                <Step title="Build" status={StepStatus::Running} icon={icons::HAMMER} />
                <Step title="Release" icon={icons::ROCKET} />
            </Steps>
        }
    };

    html! {
        <Stack gap={Space::Xl}>
            { steps(Size::Small, "Small steps") }
            { steps(Size::Medium, "Medium steps") }
            { steps(Size::Large, "Large steps") }
        </Stack>
    }
}
