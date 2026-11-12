use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <Heading level={HeadingLevel::H4}>{ "Deploying with " }<InlineCode>{ "nomad" }</InlineCode></Heading>
            <Text tag={TextTag::P}>
                { "Run " }
                <InlineCode>{ "nomad job run caddy.hcl" }</InlineCode>
                { " to deploy, then check " }
                <InlineCode>{ "/v1/status" }</InlineCode>
                { " until it reports " }
                <InlineCode>{ "healthy" }</InlineCode>
                { "." }
            </Text>
            <Text tag={TextTag::P} size={TextSize::Sm} color={TextColor::Muted}>
                { "Set " }<InlineCode>{ "NOMAD_ADDR" }</InlineCode>{ " first." }
            </Text>
        </Stack>
    }
}
