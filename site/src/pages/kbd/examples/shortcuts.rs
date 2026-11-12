use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <Text tag={TextTag::P}>{ "Open the command palette with " }<Kbd keys="Ctrl+Shift+P" />{ "." }</Text>
            <Text tag={TextTag::P}>{ "Go to hosts with " }<Kbd keys="G H" />{ "." }</Text>
            <Text tag={TextTag::P}>{ "Save everything with " }<Kbd keys="Ctrl+K Ctrl+S" />{ "." }</Text>
            <Text tag={TextTag::P}>{ "Zoom in with " }<Kbd keys="Ctrl++" />{ "." }</Text>
        </Stack>
    }
}
