use yew::prelude::*;
use yewi::prelude::*;


#[function_component]
pub fn Example() -> Html {
    let brand = html! {
        <AppBarBrand href="#" label="Buckle overview" logo={html! { <Icon icon={icons::BOXES} /> }}>
            { "Buckle" }
        </AppBarBrand>
    };
    let actions = html! {
        <>
            <ThemeToggle />
            <UserChip name="Ada Lovelace" detail="admin" size={Size::Small} />
        </>
    };

    html! { <AppBar {brand} {actions} /> }
}
