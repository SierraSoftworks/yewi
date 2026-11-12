use yew::prelude::*;
use yewi::prelude::*;

const SHAPES: [(&str, &str, &str); 4] = [
    ("--yw-radius-sm", "var(--yw-radius-sm)", "none"),
    ("--yw-radius-control", "var(--yw-radius-control)", "none"),
    ("--yw-radius-card", "var(--yw-radius-card)", "var(--yw-shadow-card)"),
    ("--yw-shadow-overlay", "var(--yw-radius-card)", "var(--yw-shadow-overlay)"),
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
