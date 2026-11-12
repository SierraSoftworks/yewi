use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster gap={Space::Lg}>
            <Icon icon={icons::SEARCH} />
            <Icon icon={icons::BELL} />
            <Icon icon={icons::REFRESH_CW} />
            <Icon icon={icons::CHECK} />
            <Icon icon={icons::TRIANGLE_ALERT} />
            <Icon icon={icons::COPY} />
            <Icon icon={icons::USER} />
            <Icon icon={icons::LAYERS} />
        </Cluster>
    }
}
