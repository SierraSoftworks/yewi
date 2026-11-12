use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let filter = use_state(|| Some(AttrValue::from("docs")));
    let opened = use_state(|| None::<AttrValue>);
    let items = use_memo((), |_| {
        Rc::<[BarItem]>::from(vec![
            BarItem::new("website", "Website", 8421.0),
            BarItem::new("docs", "Documentation", 3977.0),
            BarItem::new("status", "Status page", 1250.0),
            BarItem::new("", "No project", 310.0).muted(),
        ])
    });

    let on_select = {
        let filter = filter.clone();
        Callback::from(move |value: AttrValue| {
            // Choosing the active row again clears the filter.
            filter.set(Some(value).filter(|value| filter.as_ref() != Some(value)));
        })
    };
    let action = {
        let opened = opened.clone();
        BarListAction::new(icons::SETTINGS, "Manage", Callback::from(move |value| opened.set(Some(value))))
    };

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 26rem" }}>
            <BarList
                items={(*items).clone()}
                label="Projects"
                heading="Project"
                value_heading="Events"
                selected={(*filter).clone()}
                {on_select}
                {action}
            />
            <span role="status">
                { match (filter.as_deref(), opened.as_deref()) {
                    (_, Some(project)) => format!("Managing {project}"),
                    (Some(""), None) => "Showing events with no project".to_string(),
                    (Some(project), None) => format!("Showing events for {project}"),
                    (None, None) => "Showing all events".to_string(),
                } }
            </span>
        </Stack>
    }
}
