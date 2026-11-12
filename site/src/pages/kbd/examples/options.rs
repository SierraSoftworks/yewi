use yew::prelude::*;
use yewi::prelude::*;

const OUTLINED: &str = "--yi-kbd-bg: var(--yi-surface); --yi-kbd-border: var(--yi-line-strong)";
const RAISED: &str = "--yi-kbd-bg: var(--yi-surface); --yi-kbd-border: var(--yi-line-strong); --yi-kbd-depth: 2px";

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <Text tag={TextTag::P}>
                <Text weight={TextWeight::Medium}>{ "Tinted (default): " }</Text>
                { "open the palette with " }<Kbd keys="Ctrl+K" />{ "." }
            </Text>
            <Text tag={TextTag::P}>
                <Text weight={TextWeight::Medium}>{ "Outlined: " }</Text>
                { "open the palette with " }<Kbd keys="Ctrl+K" attrs={attrs! { "style" => OUTLINED }} />{ "." }
            </Text>
            <Text tag={TextTag::P}>
                <Text weight={TextWeight::Medium}>{ "Raised cap: " }</Text>
                { "open the palette with " }<Kbd keys="Ctrl+K" attrs={attrs! { "style" => RAISED }} />{ "." }
            </Text>
        </Stack>
    }
}
