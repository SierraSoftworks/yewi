use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <Heading level={HeadingLevel::H1}>{ "Rollouts" }</Heading>
            <Heading level={HeadingLevel::H2}>{ "Latest rollout" }</Heading>
            <Heading level={HeadingLevel::H3}>{ "Stage: home" }</Heading>
            <Heading level={HeadingLevel::H4}>{ "Hosts in this stage" }</Heading>
            <Heading level={HeadingLevel::H5}>{ "Health checks" }</Heading>
            <Heading level={HeadingLevel::H6}>{ "Last probe" }</Heading>
        </Stack>
    }
}
