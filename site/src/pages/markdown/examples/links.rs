use yew::prelude::*;
use yewi::prelude::*;

const SOURCE: &str = r#"Paste the address into the webhook settings of
[your Miniflux instance](https://miniflux.app/docs/), then check the
[delivery log](/workflows/deliveries) for the first event.
"#;

#[function_component]
pub fn Example() -> Html {
    let followed = use_state(|| None::<AttrValue>);
    let on_link_click = {
        let followed = followed.clone();
        Callback::from(move |click: MarkdownLinkClick| {
            // A real app would hand links within the site to its router.
            click.event.prevent_default();
            followed.set(Some(click.href));
        })
    };

    html! {
        <Stack>
            <Markdown source={SOURCE} external_links=true {on_link_click} />
            <span role="status">
                { match &*followed {
                    Some(href) => format!("Followed {href}"),
                    None => "No link followed yet".to_owned(),
                } }
            </span>
        </Stack>
    }
}
