use yew::prelude::*;
use yewi::prelude::*;

const SIZES: [(&str, &str); 8] = [
    ("xs", "12"),
    ("sm", "13"),
    ("md", "14"),
    ("lg", "15"),
    ("xl", "17"),
    ("2xl", "20"),
    ("3xl", "24"),
    ("4xl", "30"),
];

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack gap={Space::Sm}>
            { for SIZES.into_iter().map(|(name, px)| html! {
                <Cluster align={Align::Baseline} gap={Space::Lg} nowrap=true>
                    <code style="inline-size: 9rem; flex: none">{ format!("--yi-text-{name}") }</code>
                    <span style={format!("font-size: var(--yi-text-{name})")}>{ format!("Rollout #42 paused ({px}px)") }</span>
                </Cluster>
            }) }
        </Stack>
    }
}
