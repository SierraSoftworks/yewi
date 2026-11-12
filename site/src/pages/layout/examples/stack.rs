use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack gap={Space::Sm}>
            <div class="demo-box">{ "First" }</div>
            <div class="demo-box">{ "Second" }</div>
            <div class="demo-box">{ "Third" }</div>
        </Stack>
    }
}
