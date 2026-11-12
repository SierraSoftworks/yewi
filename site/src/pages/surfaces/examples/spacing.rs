use yew::prelude::*;
use yewi::prelude::*;

const STEPS: [&str; 9] = ["2xs", "xs", "sm", "md", "lg", "xl", "2xl", "3xl", "4xl"];

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack gap={Space::Xs}>
            { for STEPS.into_iter().map(|step| html! {
                <Cluster gap={Space::Lg} nowrap=true>
                    <code style="inline-size: 9rem; flex: none">{ format!("--yw-space-{step}") }</code>
                    <div class="demo-bar" style={format!("inline-size: var(--yw-space-{step})")} />
                </Cluster>
            }) }
        </Stack>
    }
}
