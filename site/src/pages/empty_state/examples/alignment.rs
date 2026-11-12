use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let state = |align: EmptyStateAlign| {
        html! {
            <EmptyState
                title="No workflows yet"
                icon={icons::INBOX}
                {align}
                actions={html! { <Button icon={icons::PLUS}>{ "New workflow" }</Button> }}
            >
                { "Workflows run a series of steps when something happens in a connected service." }
            </EmptyState>
        }
    };

    html! {
        <Grid min="16rem">
            { state(EmptyStateAlign::Center) }
            { state(EmptyStateAlign::Start) }
        </Grid>
    }
}
