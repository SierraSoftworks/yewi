use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Steps orientation={StepsOrientation::Vertical} size={Size::Small} label="Steps of nginx">
            <Step
                title="package nginx"
                status={StepStatus::Done}
                description="Installed 1.27.2-1 from the distribution repository."
                meta={html! { "2.4s" }}
            />
            <Step
                title="file /etc/nginx/nginx.conf"
                status={StepStatus::Done}
                description="Rendered from the template; 4 lines changed."
                meta={html! { "38ms" }}
            />
            <Step title="service nginx reload" status={StepStatus::Failed} meta={html! { "1.1s" }}>
                <pre style="margin: 0; white-space: pre-wrap">
                    { "nginx: [emerg] unknown directive \"upstrem\" in /etc/nginx/nginx.conf:41" }
                </pre>
            </Step>
            <Step title="check http :443" status={StepStatus::Skipped} description="Not run: an earlier step failed." />
        </Steps>
    }
}
