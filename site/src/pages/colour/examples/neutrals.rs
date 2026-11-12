use yew::prelude::*;
use yewi::prelude::*;

const SURFACES: [&str; 7] =
    ["--yi-bg", "--yi-surface", "--yi-surface-raised", "--yi-tone-1", "--yi-tone-2", "--yi-line", "--yi-line-strong"];
const TEXT: [&str; 3] = ["--yi-fg", "--yi-fg-muted", "--yi-fg-subtle"];

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack gap={Space::Lg}>
            <Grid min="8rem" gap={Space::Sm}>
                { for SURFACES.into_iter().map(|token| html! {
                    <Stack gap={Space::Xs}>
                        <div class="demo-swatch" style={format!("background: var({token})")} />
                        <code>{ token }</code>
                    </Stack>
                }) }
            </Grid>
            <Stack gap={Space::Xs}>
                { for TEXT.into_iter().map(|token| html! {
                    <span style={format!("color: var({token})")}>
                        { "Clients go first, so the servers never schedule onto an older client. " }
                        <code>{ token }</code>
                    </span>
                }) }
            </Stack>
        </Stack>
    }
}
