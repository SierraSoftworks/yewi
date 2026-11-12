use yew::prelude::*;
use yewi::prelude::*;

const SOURCE: &str = r#"// Greets whoever is signed in.
fn greet(user: &User) -> String {
    format!("Hello {}, you have {} alerts", user.name, 3)
}"#;

const BOX: &str = "margin: 0; padding: var(--yi-space-md); border-radius: var(--yi-radius-control); \
                   background: var(--yi-code-bg); overflow-x: auto";

#[function_component]
pub fn Example() -> Html {
    let tokens = tokenize(SOURCE, "rust");
    let count = |kind| tokens.iter().filter(|token| token.kind == Some(kind)).count();

    html! {
        <Stack>
            <pre style={BOX}><Highlight code={SOURCE} language="rust" /></pre>
            // The class colours the text; `yi-highlight` supplies the palette.
            <code class="yi-highlight">
                <Cluster>
                    { for TokenKind::ALL.into_iter().map(|kind| html! {
                        <span class={kind.class()}>{ format!("{} × {}", kind.as_str(), count(kind)) }</span>
                    }) }
                </Cluster>
            </code>
            // Recolour one kind for this subtree only.
            <pre style={format!("{BOX}; --yi-highlight-comment: var(--yi-accent-text)")}>
                <Highlight code={SOURCE} language="rust" />
            </pre>
        </Stack>
    }
}
