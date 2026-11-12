use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Breadcrumbs separator={BreadcrumbSeparator::Chevron}>
            <Crumb href="#home" icon={icons::HOUSE}>{ "Home" }</Crumb>
            <Crumb href="#missions">{ "Missions" }</Crumb>
            <Crumb href="#mission">
                { "Search and rescue exercise, northern sector, second rotation, night shift" }
            </Crumb>
            <Crumb href="#members">{ "Members" }</Crumb>
            <Crumb>{ "Ada Lovelace" }</Crumb>
        </Breadcrumbs>
    }
}
