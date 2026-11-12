use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster gap={Space::Xl} align={Align::Center}>
            <Skeleton width="12rem" />
            <Skeleton shape={SkeletonShape::Block} width="8rem" height="4.5rem" />
            <Skeleton shape={SkeletonShape::Circle} width="3rem" />
        </Cluster>
    }
}
