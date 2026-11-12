use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yewi::prelude::*;

mod features {
    include!(concat!(env!("OUT_DIR"), "/features.rs"));
}

#[function_component]
fn App() -> Html {
    html! {
        <Stack>
            <Button variant={ButtonVariant::Primary} icon={icons::CHECK}>{ "Approve" }</Button>
            <ThemeToggle />
            { for features::views() }
        </Stack>
    }
}

#[wasm_bindgen(start)]
fn start() {
    install_styles();
    yew::Renderer::<App>::new().render();
}
