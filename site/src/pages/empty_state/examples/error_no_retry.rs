use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Grid min="16rem">
            <ErrorState
                title="Not permitted"
                icon={icons::LOCK}
                actions={html! { <Button href="#access">{ "Request access" }</Button> }}
            >
                { "Your role cannot view rollouts in the production cluster." }
            </ErrorState>
            <ErrorState title="Could not load the audit log" size={Size::Small} align={EmptyStateAlign::Start}>
                <p>{ "The request failed with status 502." }</p>
                <p><code>{ "request 7f3a9c1e" }</code></p>
            </ErrorState>
        </Grid>
    }
}
