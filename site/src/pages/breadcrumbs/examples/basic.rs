use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Breadcrumbs>
            <Crumb href="#dashboard">{ "Dashboard" }</Crumb>
            <Crumb href="#exceptions">{ "Exceptions" }</Crumb>
            <Crumb>{ "TimeoutError" }</Crumb>
        </Breadcrumbs>
    }
}
