use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster gap={Space::Xs}>
            <Tag name="env">{ "production" }</Tag>
            <Tag name="region">{ "eu-west" }</Tag>
            <Tag name="tier">{ "edge" }</Tag>
            <Tag name="commit" mono=true>{ "9f9107c" }</Tag>
        </Cluster>
    }
}
