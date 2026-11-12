use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Text tag={TextTag::P}>
            { "See " }
            <Link href="#inside-a-link">{ "the " }<InlineCode>{ "Styles" }</InlineCode>{ " component" }</Link>
            { " for how the stylesheet is installed." }
        </Text>
    }
}
