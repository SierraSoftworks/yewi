use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Grid min="16rem">
            <EmptyState title="No incidents reported" icon={icons::CHECK} tone={Tone::Ok}>
                { "Every probe has been healthy for the last 90 days." }
            </EmptyState>
            <EmptyState
                title="No hosts match"
                icon={icons::SEARCH_X}
                actions={html! { <Button>{ "Clear filters" }</Button> }}
            >
                { "Nothing matches \"region:apac status:draining\"." }
            </EmptyState>
        </Grid>
    }
}
