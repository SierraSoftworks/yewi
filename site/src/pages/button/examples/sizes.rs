use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster>
            <Button size={Size::Small}>{ "Small" }</Button>
            <Button size={Size::Medium}>{ "Medium" }</Button>
            <Button size={Size::Large}>{ "Large" }</Button>
        </Cluster>
    }
}
