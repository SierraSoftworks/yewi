use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster gap={Space::Xs}>
            <Tag href="#os-linux">{ "linux" }</Tag>
            <Tag name="env" href="#env-production">{ "production" }</Tag>
            <Tag icon={icons::GIT_BRANCH} mono=true href="#branch-main" on_remove={Callback::noop()}>{ "main" }</Tag>
        </Cluster>
    }
}
