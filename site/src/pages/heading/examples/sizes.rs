use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack gap={Space::Sm}>
            { for TextSize::ALL.into_iter().rev().map(|size| html! {
                <Heading level={HeadingLevel::H3} {size}>
                    { format!("Stage: home ({})", size.as_str()) }
                </Heading>
            }) }
        </Stack>
    }
}
