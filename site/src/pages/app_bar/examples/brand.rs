use yew::prelude::*;
use yewi::prelude::*;


#[function_component]
pub fn Example() -> Html {
    let custom = html! {
        <svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
            <path d="M12 2 22 20H2Z" />
        </svg>
    };

    html! {
        <Stack>
            <AppBar brand={html! {
                <AppBarBrand href="#" logo={html! { <Icon icon={icons::ROCKET} /> }}>{ "Automate" }</AppBarBrand>
            }} />
            <AppBar brand={html! { <AppBarBrand href="#">{ "rustak" }</AppBarBrand> }} />
            <AppBar
                attrs={attrs! { "style" => "--yi-app-bar-logo-color: var(--yi-danger)" }}
                brand={html! { <AppBarBrand logo={custom}>{ "Sierra" }</AppBarBrand> }}
            />
        </Stack>
    }
}
