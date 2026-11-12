use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack gap={Space::None}>
            <EmptyState title="No audit events" size={Size::Small}>
                { "Nothing has changed in the last 7 days." }
            </EmptyState>
            <EmptyState title="No audit events" icon={icons::INBOX}>
                { "Nothing has changed in the last 7 days." }
            </EmptyState>
            <EmptyState title="No audit events" icon={icons::INBOX} size={Size::Large}>
                { "Nothing has changed in the last 7 days." }
            </EmptyState>
        </Stack>
    }
}
