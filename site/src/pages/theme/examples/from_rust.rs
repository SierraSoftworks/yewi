use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let theme = use_theme();
    html! {
        <Cluster>
            <Button on_click={|_| set_theme(Theme::Light)}>{ "Light" }</Button>
            <Button on_click={|_| set_theme(Theme::Dark)}>{ "Dark" }</Button>
            <Button on_click={|_| set_theme(Theme::System)}>{ "System" }</Button>
            <span role="status">
                { format!("Preference: {}. The page is {}.", theme.preference.label(), if theme.dark { "dark" } else { "light" }) }
            </span>
        </Cluster>
    }
}
