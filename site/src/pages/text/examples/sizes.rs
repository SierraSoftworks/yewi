use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack gap={Space::Xl}>
            <Stack gap={Space::Xs}>
                { for TextSize::ALL.into_iter().map(|size| html! {
                    <Text {size}>{ format!("Rollout #42 paused ({})", size.as_str()) }</Text>
                }) }
            </Stack>
            <Cluster gap={Space::Lg}>
                <Text weight={TextWeight::Regular}>{ "Regular" }</Text>
                <Text weight={TextWeight::Medium}>{ "Medium" }</Text>
                <Text weight={TextWeight::Semibold}>{ "Semibold" }</Text>
            </Cluster>
        </Stack>
    }
}
