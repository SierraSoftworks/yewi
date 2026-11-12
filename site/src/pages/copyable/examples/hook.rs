use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let clipboard = use_clipboard();
    let on_click = {
        let copy = clipboard.copy.clone();
        Callback::from(move |_| copy.emit("https://buckle.example.com/rollouts/1842".into()))
    };

    html! {
        <Stack align={Align::Start}>
            <Button icon={icons::SHARE_2} {on_click}>{ "Share this rollout" }</Button>
            <span role="status">
                { match clipboard.state {
                    CopyState::Idle => "",
                    CopyState::Copied => "Link copied to the clipboard",
                    CopyState::Failed => "The browser refused the clipboard",
                } }
            </span>
        </Stack>
    }
}
