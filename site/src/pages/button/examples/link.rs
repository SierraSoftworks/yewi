use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster>
            <Button href="https://github.com/SierraSoftworks/yewi" icon={icons::EXTERNAL_LINK} target="_blank">
                { "Source" }
            </Button>
            <Button variant={ButtonVariant::Subtle} href="#as-a-link">{ "Jump to this section" }</Button>
        </Cluster>
    }
}
