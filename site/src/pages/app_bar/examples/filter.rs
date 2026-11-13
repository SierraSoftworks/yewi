use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let query = use_state(AttrValue::default);
    let on_input = {
        let query = query.clone();
        Callback::from(move |value: String| query.set(value.into()))
    };

    let brand = html! { <AppBarBrand logo={html! { <Icon icon={icons::ACTIVITY} /> }}>{ "Grey" }</AppBarBrand> };
    let actions = html! {
        <>
            <Pill tone={Tone::Ok} active=true>{ "live" }</Pill>
            <ThemeToggle />
            <Avatar name="Ada Lovelace" size={Size::Small} />
        </>
    };

    html! {
        <Stack>
            <AppBar {brand} {actions} wrap=true>
                <TextInput
                    kind={TextInputKind::Search}
                    value={(*query).clone()}
                    {on_input}
                    placeholder="Filter by name, service or tag"
                    attrs={attrs! { "aria-label" => "Filter" }}
                />
            </AppBar>
            <span role="status">
                { if query.is_empty() { "Showing everything".to_owned() } else { format!("Filtering by {}", *query) } }
            </span>
        </Stack>
    }
}
