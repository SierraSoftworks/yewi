use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <Cluster>
                <Kbd>{ "Esc" }</Kbd>
                <Kbd>{ "Tab" }</Kbd>
                <Kbd>{ "Enter" }</Kbd>
                <Kbd>{ "Space" }</Kbd>
                <Kbd>{ "?" }</Kbd>
                <Kbd>{ "/" }</Kbd>
            </Cluster>
            <Text tag={TextTag::P}>{ "Press " }<Kbd>{ "/" }</Kbd>{ " to search, or " }<Kbd>{ "Esc" }</Kbd>{ " to close." }</Text>
        </Stack>
    }
}
