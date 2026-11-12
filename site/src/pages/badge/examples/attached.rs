use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster gap={Space::Xl}>
            <Badge count=3 tone={Tone::Danger} bold=true label="3 unread notifications">
                <Button class="yi-button--icon-only" icon={icons::BELL} attrs={attrs! { "aria-label" => "Notifications" }} />
            </Badge>
            <Badge count=128 tone={Tone::Info} bold=true label="128 unread messages">
                <Button class="yi-button--icon-only" icon={icons::MAIL} attrs={attrs! { "aria-label" => "Messages" }} />
            </Badge>
            <Badge count=5 label="5 queued changes">
                <Button icon={icons::INBOX}>{ "Queue" }</Button>
            </Badge>
        </Cluster>
    }
}
