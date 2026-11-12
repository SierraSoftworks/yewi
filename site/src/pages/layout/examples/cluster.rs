use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster justify={Justify::Between}>
            <Cluster>
                <Button variant={ButtonVariant::Primary}>{ "Approve" }</Button>
                <Button>{ "Pause" }</Button>
            </Cluster>
            <Button variant={ButtonVariant::Subtle} icon={icons::REFRESH_CW}>{ "Refresh" }</Button>
        </Cluster>
    }
}
