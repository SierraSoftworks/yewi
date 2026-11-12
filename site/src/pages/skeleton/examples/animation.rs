use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let column = |name: &'static str, animation: SkeletonAnimation| {
        html! {
            <Stack gap={Space::Sm}>
                <b>{ name }</b>
                <Skeleton shape={SkeletonShape::Block} height="4rem" {animation} />
                <Skeleton lines=3 {animation} />
            </Stack>
        }
    };

    html! {
        <Grid min="11rem" gap={Space::Xl}>
            { column("Shimmer", SkeletonAnimation::Shimmer) }
            { column("Pulse", SkeletonAnimation::Pulse) }
            { column("None", SkeletonAnimation::None) }
        </Grid>
    }
}
