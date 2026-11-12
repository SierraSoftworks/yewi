use yew::prelude::*;
use yewi::prelude::*;

const OUTLINED: &str = "--yi-inline-code-bg: var(--yi-surface); --yi-inline-code-border: var(--yi-line-strong)";
const PLAIN: &str = "--yi-inline-code-bg: transparent; --yi-inline-code-fg: var(--yi-accent-text)";

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <Text tag={TextTag::P}>
                <Text weight={TextWeight::Medium}>{ "A. Tinted (default): " }</Text>
                { "run " }<InlineCode>{ "nomad job run caddy.hcl" }</InlineCode>{ " to deploy." }
            </Text>
            <Text tag={TextTag::P}>
                <Text weight={TextWeight::Medium}>{ "B. Outlined: " }</Text>
                { "run " }<InlineCode attrs={attrs! { "style" => OUTLINED }}>{ "nomad job run caddy.hcl" }</InlineCode>{ " to deploy." }
            </Text>
            <Text tag={TextTag::P}>
                <Text weight={TextWeight::Medium}>{ "C. Plain accent: " }</Text>
                { "run " }<InlineCode attrs={attrs! { "style" => PLAIN }}>{ "nomad job run caddy.hcl" }</InlineCode>{ " to deploy." }
            </Text>
        </Stack>
    }
}
