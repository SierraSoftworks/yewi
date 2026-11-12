use yew::prelude::*;
use yewi::prelude::*;

const ACCENTS: [(&str, &str); 5] = [
    ("Default", "light-dark(#409eff, #69b4ff)"),
    ("Amber", "#b9852f"),
    ("Green", "#156a4f"),
    ("Red", "#c4372e"),
    ("Violet", "#6d4aff"),
];

#[function_component]
pub fn Example() -> Html {
    html! {
        <Grid min="15rem" gap={Space::Sm}>
            { for ACCENTS.into_iter().map(|(name, accent)| html! {
                // `data-yi-scope` re-derives every token from this element's accent.
                <div class="demo-panel" data-yi-scope="" style={format!("--yi-accent: {accent}")}>
                    <Stack align={Align::Start}>
                        <b>{ name }</b>
                        <Cluster>
                            <Button variant={ButtonVariant::Primary}>{ "Approve" }</Button>
                            <Button>{ "Cancel" }</Button>
                        </Cluster>
                        <a href="#accent">{ "A link in the accent" }</a>
                        <span style="color: var(--yi-fg-muted)">{ "Supporting text on a tinted neutral" }</span>
                        <span class="demo-chip" style="background: var(--yi-accent-soft); color: var(--yi-accent-text)">
                            { "Soft accent" }
                        </span>
                    </Stack>
                </div>
            }) }
        </Grid>
    }
}
