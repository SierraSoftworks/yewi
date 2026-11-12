use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Grid min="14rem" gap={Space::Xl}>
            <p style="margin: 0">
                { "Rollouts move through hosts in waves. A wave that fails its health checks pauses the rollout and leaves the remaining hosts untouched." }
            </p>
            <Skeleton lines=4 />
        </Grid>
    }
}
