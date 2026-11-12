use yew::prelude::*;
use yewi::prelude::*;

/// Every built-in lexer is reachable, as it is from a Markdown code fence.
pub fn view() -> Html {
    html! {
        { for Language::ALL.into_iter().map(|language| html! {
            <pre><Highlight code={"let x = \"1\"; // one\n"} language={language.name()} /></pre>
        }) }
    }
}
