use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster gap={Space::Lg}>
            // Decorative: the text already says it.
            <span><Icon icon={icons::LOCK} />{ " Private" }</span>
            // Meaningful: the icon is the only content.
            <Icon icon={icons::LOCK} label="Private" />
        </Cluster>
    }
}
