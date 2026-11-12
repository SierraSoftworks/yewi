use yew::prelude::*;
use yewi::prelude::*;

const SHAPES: [(&str, &str, &str); 4] = [
    ("--yi-radius-sm", "var(--yi-radius-sm)", "none"),
    ("--yi-radius-control", "var(--yi-radius-control)", "none"),
    ("--yi-radius-card", "var(--yi-radius-card)", "var(--yi-shadow-card)"),
    ("--yi-shadow-overlay", "var(--yi-radius-card)", "var(--yi-shadow-overlay)"),
];

#[function_component]
pub fn Example() -> Html {
    html! {
        <Grid min="10rem">
            { for SHAPES.into_iter().map(|(label, radius, shadow)| html! {
                <Stack gap={Space::Sm}>
                    <div class="demo-shape" style={format!("border-radius: {radius}; box-shadow: {shadow}")} />
                    <code>{ label }</code>
                </Stack>
            }) }
        </Grid>
    }
}
