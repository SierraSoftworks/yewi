use yew::prelude::*;
use yewi::prelude::*;

pub fn view() -> Html {
    html! { <pre><Highlight code={"int main(void) { return 0; }\n"} language="c" /></pre> }
}
