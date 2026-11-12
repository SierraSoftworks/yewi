use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let unread = use_state(|| 0_u32);

    let receive = {
        let unread = unread.clone();
        Callback::from(move |_| unread.set(*unread + 4))
    };
    let read_all = {
        let unread = unread.clone();
        Callback::from(move |_| unread.set(0))
    };

    html! {
        <Stack>
            <Cluster gap={Space::Lg} align={Align::Center}>
                <Badge count={*unread} max=9 tone={Tone::Danger} bold=true>
                    <IconButton icon={icons::BELL} label="Notifications" />
                </Badge>
                <Button on_click={receive}>{ "Receive 4" }</Button>
                <Button variant={ButtonVariant::Subtle} disabled={*unread == 0} on_click={read_all}>{ "Mark all read" }</Button>
            </Cluster>
            <span role="status">{ format!("{} unread notifications", *unread) }</span>
        </Stack>
    }
}
