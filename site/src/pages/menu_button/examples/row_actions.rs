use yew::prelude::*;
use yewi::prelude::*;

const HOSTS: [&str; 3] = ["web-1", "web-2", "worker-1"];

#[function_component]
pub fn Example() -> Html {
    let done = use_state(|| String::from("Nothing done yet"));
    let act = |verb: &'static str, host: &'static str| {
        let done = done.clone();
        Callback::from(move |_| done.set(format!("{verb} {host}")))
    };

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 22rem" }}>
            { for HOSTS.into_iter().map(|host| html! {
                <Cluster justify={Justify::Between}>
                    <code>{ host }</code>
                    <MenuButton
                        label={format!("Actions for {host}")}
                        icon_only=true
                        variant={ButtonVariant::Subtle}
                        size={Size::Small}
                        placement={Placement::BottomEnd}
                    >
                        <MenuItem on_select={act("Restarted", host)}>{ "Restart" }</MenuItem>
                        <MenuItem on_select={act("Drained", host)}>{ "Drain" }</MenuItem>
                        <MenuItem tone={Tone::Danger} on_select={act("Forgot", host)}>{ "Forget" }</MenuItem>
                    </MenuButton>
                </Cluster>
            }) }
            <span role="status">{ (*done).clone() }</span>
        </Stack>
    }
}
