use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let filters = use_state(|| vec![("env", "production"), ("region", "eu-west"), ("os", "linux")]);
    let removed = use_state(|| None::<&'static str>);

    let status = match *removed {
        Some(value) => format!("Removed {value}, {} filters left", filters.len()),
        None => format!("{} filters", filters.len()),
    };
    let reset = {
        let (filters, removed) = (filters.clone(), removed.clone());
        Callback::from(move |_| {
            filters.set(vec![("env", "production"), ("region", "eu-west"), ("os", "linux")]);
            removed.set(None);
        })
    };

    html! {
        <Stack>
            <Cluster gap={Space::Xs} align={Align::Center}>
                { for filters.iter().map(|&(name, value)| {
                    let on_remove = {
                        let (filters, removed) = (filters.clone(), removed.clone());
                        Callback::from(move |_| {
                            filters.set(filters.iter().copied().filter(|filter| filter.1 != value).collect());
                            removed.set(Some(value));
                        })
                    };
                    html! { <Tag key={value} {name} {on_remove}>{ value }</Tag> }
                }) }
                <Tag name="owner" disabled=true on_remove={Callback::noop()}>{ "platform" }</Tag>
                <Button size={Size::Small} variant={ButtonVariant::Subtle} on_click={reset}>{ "Reset" }</Button>
            </Cluster>
            <span role="status">{ status }</span>
        </Stack>
    }
}
