use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <DescriptionList attrs={attrs! { "aria-label" => "Workload" }}>
            <DescriptionItem term="Name">{ "caddy" }</DescriptionItem>
            <DescriptionItem term="Cluster">{ "eu-west-1" }</DescriptionItem>
            <DescriptionItem term="Running version">{ "2.8.4" }</DescriptionItem>
            <DescriptionItem term="On removal">{ "Drain connections, then stop" }</DescriptionItem>
        </DescriptionList>
    }
}
