use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <DescriptionList layout={DescriptionLayout::Stacked} attrs={attrs! { "aria-label" => "Rollout" }}>
            <DescriptionItem term="Started by">{ "Ben" }</DescriptionItem>
            <DescriptionItem term="Strategy">{ "Canary, then 25% waves" }</DescriptionItem>
            <DescriptionItem term="Progress">{ "2 of 31 hosts" }</DescriptionItem>
            <DescriptionItem term="Commit" mono=true>{ "9f9107c" }</DescriptionItem>
            <DescriptionItem term="Reason" wide=true>
                { "Picks up the upstream fix for connection resets under load, ahead of the weekend traffic peak." }
            </DescriptionItem>
        </DescriptionList>
    }
}
