use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster gap={Space::Xs}>
            <Tag>{ "package" }</Tag>
            <Tag tone={Tone::Danger} icon={icons::TRIANGLE_ALERT}>{ "destructive" }</Tag>
            <Tag tone={Tone::Warn} icon={icons::TRIANGLE_ALERT}>{ "caution" }</Tag>
            <Tag tone={Tone::Info} icon={icons::FLASK_CONICAL}>{ "demo" }</Tag>
            <Tag tone={Tone::Ok}>{ "verified" }</Tag>
            <Tag tone={Tone::Active}>{ "syncing" }</Tag>
            <Tag icon={icons::GIT_BRANCH} mono=true>{ "main" }</Tag>
            <Tag><StatusDot tone={Tone::Ok} />{ "online" }</Tag>
        </Cluster>
    }
}
