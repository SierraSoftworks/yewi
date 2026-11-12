use yew::prelude::*;
use yewi::prelude::*;

const FLAT: &str = "--yi-kbd-depth: 1px";
const FILLED: &str = "--yi-kbd-depth: 1px; --yi-kbd-bg: var(--yi-neutral-soft); --yi-kbd-border: transparent";

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <Text tag={TextTag::P}>
                <Text weight={TextWeight::Medium}>{ "A. Key cap (default): " }</Text>
                { "open the palette with " }<Kbd keys="Ctrl+K" />{ "." }
            </Text>
            <Text tag={TextTag::P}>
                <Text weight={TextWeight::Medium}>{ "B. Flat outline: " }</Text>
                { "open the palette with " }<Kbd keys="Ctrl+K" attrs={attrs! { "style" => FLAT }} />{ "." }
            </Text>
            <Text tag={TextTag::P}>
                <Text weight={TextWeight::Medium}>{ "C. Tinted, no border: " }</Text>
                { "open the palette with " }<Kbd keys="Ctrl+K" attrs={attrs! { "style" => FILLED }} />{ "." }
            </Text>
        </Stack>
    }
}
