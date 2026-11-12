use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <DescriptionList ruled=true attrs={attrs! { "aria-label" => "Connection settings" }}>
            <DescriptionItem term="Protocol">{ "TLS 1.3" }</DescriptionItem>
            <DescriptionItem term="Port" mono=true>{ "8089" }</DescriptionItem>
            <DescriptionItem term="Client certificates">{ "Required" }</DescriptionItem>
            <DescriptionItem term="Idle timeout">{ "5 minutes" }</DescriptionItem>
            <DescriptionItem term="Maximum message size">{ "1 MB" }</DescriptionItem>
        </DescriptionList>
    }
}
