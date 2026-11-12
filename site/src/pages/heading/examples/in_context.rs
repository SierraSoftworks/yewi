use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack gap={Space::Xl} attrs={attrs! { "style" => "max-inline-size: 28rem" }}>
            <Stack gap={Space::Xs}>
                <Heading level={HeadingLevel::H1} truncate=true>
                    { "caddy-ingress-eu-west-1-production-2026-01-15" }
                </Heading>
                <Text color={TextColor::Muted}>{ "Rollout #42, started by ben 4 minutes ago." }</Text>
            </Stack>
            <Stack gap={Space::Sm}>
                <Heading level={HeadingLevel::H2} size={TextSize::Xl}>{ "Stages" }</Heading>
                <Text tag={TextTag::P}>{ "Each stage must report healthy before the next one starts." }</Text>
            </Stack>
            <Stack gap={Space::Sm}>
                <Heading level={HeadingLevel::H2} size={TextSize::Xl}>{ "History" }</Heading>
                <Text tag={TextTag::P}>{ "Commits and operator actions, newest first." }</Text>
            </Stack>
        </Stack>
    }
}
