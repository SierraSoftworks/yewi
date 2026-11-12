use yew::prelude::*;
use yewi::prelude::*;

pub fn view() -> Html {
    html! { <Markdown source={"# Title\n\nText with a [link](/a) and `code`.\n\n| a | b |\n|---|---|\n| 1 | 2 |\n\n```rust\nfn main() {}\n```\n"} /> }
}
