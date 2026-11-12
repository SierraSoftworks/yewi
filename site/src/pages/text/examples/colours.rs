use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack gap={Space::Xl}>
            <Stack gap={Space::Xs}>
                <Text color={TextColor::Primary}>{ "caddy-ingress" }</Text>
                <Text color={TextColor::Muted}>{ "Reverse proxy for the home cluster." }</Text>
                <Text color={TextColor::Subtle} size={TextSize::Sm}>{ "Updated 4 minutes ago" }</Text>
                <Text color={TextColor::Accent}>{ "3 new commits" }</Text>
            </Stack>
            <Cluster gap={Space::Lg}>
                <Text tone={Tone::Ok}>{ "Healthy" }</Text>
                <Text tone={Tone::Warn}>{ "Degraded" }</Text>
                <Text tone={Tone::Danger}>{ "Failed" }</Text>
                <Text tone={Tone::Info}>{ "Scheduled" }</Text>
                <Text tone={Tone::Active}>{ "Rolling out" }</Text>
                <Text tone={Tone::Neutral}>{ "Paused" }</Text>
            </Cluster>
        </Stack>
    }
}
