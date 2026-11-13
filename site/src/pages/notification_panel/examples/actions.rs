use yew::prelude::*;
use yewi::prelude::*;

const BUILDS: [(&str, &str, Tone, &str); 3] = [
    ("api #1482", "Tests failed on main", Tone::Danger, "4 min"),
    ("site #731", "Deployed to production", Tone::Ok, "26 min"),
    ("worker #96", "Waiting for approval", Tone::Info, "1 h"),
];

#[function_component]
pub fn Example() -> Html {
    let open = use_state(|| false);
    let unread = use_state(|| true);

    let on_open_change = {
        let open = open.clone();
        Callback::from(move |next| open.set(next))
    };
    let mark_read = {
        let (open, unread) = (open.clone(), unread.clone());
        Callback::from(move |_| {
            unread.set(false);
            open.set(false);
        })
    };

    let count = if *unread { BUILDS.len() as u32 } else { 0 };
    let label = if *unread { format!("{count} unread notifications") } else { "No unread notifications".to_string() };
    let footer = html! {
        <Button size={Size::Small} variant={ButtonVariant::Subtle} icon={icons::CHECK_CHECK} on_click={mark_read}>
            { "Mark all as read" }
        </Button>
    };

    html! {
        <Stack align={Align::End}>
            <NotificationPanel
                {count}
                label={label.clone()}
                tone={Tone::Danger}
                open={*open}
                {on_open_change}
                footer={if *unread { footer } else { Html::default() }}
                empty="You are all caught up."
            >
                if *unread {
                    { for BUILDS.into_iter().map(|(title, why, tone, age)| html! {
                        <NotificationItem {title} {tone} meta={age} href={format!("#{}", title.replace([' ', '#'], "-"))}>
                            { why }
                        </NotificationItem>
                    }) }
                }
            </NotificationPanel>
            <span role="status">{ label }</span>
        </Stack>
    }
}
