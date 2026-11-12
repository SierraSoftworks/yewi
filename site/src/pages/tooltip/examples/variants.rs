use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster>
            <Tooltip tip="A card that follows the theme">
                <Button>{ "Surface" }</Button>
            </Tooltip>
            <Tooltip tip="A card with a pointer" arrow=true>
                <Button>{ "Surface, arrow" }</Button>
            </Tooltip>
            <Tooltip tip="Near-black in both themes" variant={TooltipVariant::Dark}>
                <Button>{ "Dark" }</Button>
            </Tooltip>
            <Tooltip tip="Near-black with a pointer" variant={TooltipVariant::Dark} arrow=true>
                <Button>{ "Dark, arrow" }</Button>
            </Tooltip>
        </Cluster>
    }
}
