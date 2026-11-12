use yew::prelude::*;
use yewi::prelude::*;

const EMAIL: &str = "ada@example.com";

#[function_component]
pub fn Example() -> Html {
    // Looking a picture up is a request to a third party, so this page waits to be asked.
    let lookup = use_state(|| false);
    let email = lookup.then_some(EMAIL);

    let on_click = {
        let lookup = lookup.clone();
        Callback::from(move |_| lookup.set(true))
    };

    html! {
        <Stack>
            <Cluster>
                <Avatar name="Ada Lovelace" {email} seed={EMAIL} size={Size::Large} />
                <UserChip name="Ada Lovelace" detail={EMAIL} {email} seed={EMAIL} />
                <Button {on_click} disabled={*lookup}>{ "Look up the picture" }</Button>
            </Cluster>
            <code style="overflow-wrap: anywhere">{ avatar_url(EMAIL, 96) }</code>
        </Stack>
    }
}
