use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster>
            <Button variant={ButtonVariant::Primary}>{ "Approve" }</Button>
            <Button>{ "Cancel" }</Button>
            <Button variant={ButtonVariant::Subtle}>{ "Details" }</Button>
            <Button variant={ButtonVariant::Danger}>{ "Roll back" }</Button>
        </Cluster>
    }
}
