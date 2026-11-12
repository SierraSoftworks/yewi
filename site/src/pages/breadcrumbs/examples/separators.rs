use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let separators = [("Slash", BreadcrumbSeparator::Slash), ("Chevron", BreadcrumbSeparator::Chevron)];

    html! {
        <Stack>
            { for separators.into_iter().map(|(name, separator)| html! {
                <Breadcrumbs label={format!("{name} breadcrumb")} {separator}>
                    <Crumb href="#fleet">{ "Fleet" }</Crumb>
                    <Crumb href="#eu-west">{ "eu-west" }</Crumb>
                    <Crumb href="#web">{ "web" }</Crumb>
                    <Crumb>{ name }</Crumb>
                </Breadcrumbs>
            }) }
        </Stack>
    }
}
