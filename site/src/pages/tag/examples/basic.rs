use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster gap={Space::Xs}>
            <Tag>{ "linux" }</Tag>
            <Tag>{ "arm64" }</Tag>
            <Tag>{ "raspberry-pi" }</Tag>
            <Tag mono=true>{ "v1.42.0" }</Tag>
            <Tag mono=true>{ "9f9107c" }</Tag>
        </Cluster>
    }
}
