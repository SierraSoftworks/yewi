use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let checked = use_state(|| false);
    let on_change = {
        let checked = checked.clone();
        Callback::from(move |value| checked.set(value))
    };

    html! {
        <Stack>
            <Checkbox checked={*checked} {on_change}>{ "Notify me when the deploy finishes" }</Checkbox>
            <span role="status">{ if *checked { "Notifications on" } else { "Notifications off" } }</span>
        </Stack>
    }
}
