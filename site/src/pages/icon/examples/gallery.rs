use yew::prelude::*;
use yewi::prelude::*;

/// How many matches to draw at once.
const LIMIT: usize = 96;

#[function_component]
pub fn Example() -> Html {
    let query = use_state(String::new);
    let on_input = {
        let query = query.clone();
        Callback::from(move |event: InputEvent| {
            let input: web_sys::HtmlInputElement = event.target_unchecked_into();
            query.set(input.value().to_lowercase());
        })
    };

    let matches = || icons::ALL.iter().filter(|(name, _)| name.contains(query.as_str()));
    let total = matches().count();

    html! {
        <Stack>
            <input type="search" placeholder="Search icons" aria-label="Search icons" oninput={on_input} />
            <span role="status">{ format!("Showing {} of {total} icons", total.min(LIMIT)) }</span>
            <Grid min="7rem" gap={Space::Sm}>
                { for matches().take(LIMIT).map(|(name, icon)| html! {
                    <Stack key={*name} gap={Space::Xs} align={Align::Center}>
                        <Icon icon={*icon} size="24px" />
                        <small>{ name }</small>
                    </Stack>
                }) }
            </Grid>
        </Stack>
    }
}
