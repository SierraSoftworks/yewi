use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let bar = html! {
        <AppBar
            brand={html! { <AppBarBrand logo={html! { <Icon icon={icons::BOXES} /> }}>{ "Buckle" }</AppBarBrand> }}
            actions={html! { <Button variant={ButtonVariant::Primary} size={Size::Small}>{ "Sign in" }</Button> }}
        />
    };

    html! {
        <AppShell embedded=true {bar} attrs={attrs! { "style" => "--yi-app-shell-height: 16rem" }}>
            <PageHeader title="Welcome" subtitle={html! { "Sign in to see the fleet." }} />
        </AppShell>
    }
}
