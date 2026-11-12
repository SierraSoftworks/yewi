use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            { for LinkVariant::ALL.into_iter().map(|variant| html! {
                <Cluster gap={Space::Lg}>
                    <Link href="#variants-and-underline" {variant}>{ format!("{variant:?}") }</Link>
                    <Link href="#variants-and-underline" {variant} underline={LinkUnderline::Always}>
                        { "Always underlined" }
                    </Link>
                    <Link href="#variants-and-underline" {variant} underline={LinkUnderline::Hover}>
                        { "Underlined on hover" }
                    </Link>
                </Cluster>
            }) }
            <Text tag={TextTag::P} color={TextColor::Muted}>
                { "Deployed by " }
                <Link href="#variants-and-underline" variant={LinkVariant::Subtle}>{ "ben" }</Link>
                { " from " }
                <Link href="#variants-and-underline" variant={LinkVariant::Subtle}>{ "main" }</Link>
                { "." }
            </Text>
        </Stack>
    }
}
