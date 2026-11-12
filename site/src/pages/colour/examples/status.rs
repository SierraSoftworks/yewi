use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster>
            { for Tone::ALL.into_iter().map(|tone| {
                let name = tone.as_str();
                html! {
                    <span class="demo-chip" style={format!("background: var(--yi-{name}-soft); color: var(--yi-{name})")}>
                        { name }
                    </span>
                }
            }) }
        </Cluster>
    }
}
