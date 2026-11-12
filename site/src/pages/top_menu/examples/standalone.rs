use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <TopMenu label="Status pages">
                <TopMenuItem href="#services" icon={icons::ACTIVITY} current=true>{ "Services" }</TopMenuItem>
                <TopMenuItem href="#incidents" icon={icons::TRIANGLE_ALERT}>{ "Incidents" }</TopMenuItem>
                <TopMenuItem href="#maintenance" icon={icons::WRENCH}>{ "Maintenance" }</TopMenuItem>
            </TopMenu>
            <Text>{ "All 14 services are operating normally." }</Text>
        </Stack>
    }
}
