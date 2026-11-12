use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster>
            <Tooltip tip="A card that follows the theme">
                <Button>{ "Surface" }</Button>
            </Tooltip>
            <Tooltip tip="A card without the arrow" arrow=false>
                <Button>{ "Surface, no arrow" }</Button>
            </Tooltip>
            <Tooltip tip="Near-black in both themes" variant={TooltipVariant::Dark}>
                <Button>{ "Dark" }</Button>
            </Tooltip>
            <Tooltip tip="Near-black without the arrow" variant={TooltipVariant::Dark} arrow=false>
                <Button>{ "Dark, no arrow" }</Button>
            </Tooltip>
        </Cluster>
    }
}
