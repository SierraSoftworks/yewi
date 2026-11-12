use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <span>
            <Icon icon={icons::BELL} />
            { " 3" }
            <VisuallyHidden>{ " unread notifications" }</VisuallyHidden>
        </span>
    }
}
