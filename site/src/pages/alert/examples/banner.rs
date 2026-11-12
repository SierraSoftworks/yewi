use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let dismissed = use_state(|| false);
    let on_dismiss = {
        let dismissed = dismissed.clone();
        Callback::from(move |()| dismissed.set(true))
    };

    html! {
        <Stack>
            if !*dismissed {
                <Banner {on_dismiss} dismiss_label="Dismiss announcement" actions={html! { <a href="#changes">{ "What changed" }</a> }}>
                    { "Version 2.4 is out, with faster mission sync." }
                </Banner>
            }
            <Banner tone={Tone::Warn} actions={html! { <Button size={Size::Small}>{ "Reconnect" }</Button> }}>
                { "You are offline. Changes are kept on this device until the connection returns." }
            </Banner>
            <Banner tone={Tone::Danger}>{ "The server is in read-only mode while a restore is running." }</Banner>
            <Note attrs={attrs! { "role" => "status" }}>
                { if *dismissed { "Announcement dismissed" } else { "Announcement shown" } }
            </Note>
        </Stack>
    }
}
