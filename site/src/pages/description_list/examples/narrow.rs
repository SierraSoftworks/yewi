use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
fn Host() -> Html {
    html! {
        <DescriptionList attrs={attrs! { "aria-label" => "Host" }}>
            <DescriptionItem term="Host">{ "nomad-client-07" }</DescriptionItem>
            <DescriptionItem term="Address" mono=true>{ "10.20.4.17" }</DescriptionItem>
            <DescriptionItem term="Last heartbeat">{ "12 seconds ago" }</DescriptionItem>
        </DescriptionList>
    }
}

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster gap={Space::Lg} align={Align::Start}>
            <div class="demo-card" style="inline-size: 26rem; max-inline-size: 100%">
                <Host />
            </div>
            <div class="demo-card" style="inline-size: 14rem">
                <Host />
            </div>
        </Cluster>
    }
}
