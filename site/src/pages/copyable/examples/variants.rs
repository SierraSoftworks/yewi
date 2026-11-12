use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack gap={Space::Lg} attrs={attrs! { "style" => "max-inline-size: 26rem" }}>
            <span>
                { "Trace " }
                <Copyable value="7c3e0b52a9d14f68" />
            </span>
            <span>
                { "Trace " }
                <Copyable variant={CopyableVariant::Chip} value="7c3e0b52a9d14f68" />
            </span>
            <Copyable variant={CopyableVariant::Field} value="7c3e0b52a9d14f68" />
        </Stack>
    }
}
