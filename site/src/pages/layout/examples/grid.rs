use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack gap={Space::Lg}>
            <Grid min="9rem" gap={Space::Sm}>
                { for (1..=5).map(|n| html! { <div class="demo-box">{ format!("Fits {n}") }</div> }) }
            </Grid>
            <Grid columns=3 gap={Space::Sm}>
                { for (1..=3).map(|n| html! { <div class="demo-box">{ format!("Fixed {n}") }</div> }) }
            </Grid>
        </Stack>
    }
}
