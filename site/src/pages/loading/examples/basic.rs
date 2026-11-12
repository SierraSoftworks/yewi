use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster gap={Space::Xl}>
            <Loading size={Size::Small} label="Loading hosts" />
            <Loading />
            <Loading size={Size::Large} label="Planning rollout" />
        </Cluster>
    }
}
