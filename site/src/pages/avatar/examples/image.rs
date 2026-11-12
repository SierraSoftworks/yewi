use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let broken = use_state(|| false);
    let src = if *broken { "/people/missing.png" } else { PORTRAIT };

    let on_click = {
        let broken = broken.clone();
        Callback::from(move |_| broken.set(!*broken))
    };

    html! {
        <Stack>
            <Cluster>
                <Avatar name="Ada Lovelace" {src} size={Size::Small} />
                <Avatar name="Ada Lovelace" {src} />
                <Avatar name="Ada Lovelace" {src} size={Size::Large} />
            </Cluster>
            <Cluster>
                <Button {on_click}>{ if *broken { "Restore the picture" } else { "Break the picture" } }</Button>
                <span role="status">{ if *broken { "Showing initials" } else { "Showing the picture" } }</span>
            </Cluster>
        </Stack>
    }
}

/// Any image URL works; this one is inline so the example needs no server.
const PORTRAIT: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 64 64'%3E%3Crect width='64' height='64' fill='%23c9d6e8'/%3E%3Ccircle cx='32' cy='25' r='11' fill='%235b7394'/%3E%3Cpath d='M10 64c0-13 10-22 22-22s22 9 22 22z' fill='%235b7394'/%3E%3C/svg%3E";
