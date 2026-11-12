use yew::prelude::*;
use yewi::prelude::*;

pub fn view() -> Html {
    html! { <CodeBlock code={"fn main() {}\n"} language="rust" filename="main.rs" line_numbers=true marked="1" /> }
}
