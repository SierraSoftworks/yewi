use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let sides = [
        ("Top", Placement::Top),
        ("Right", Placement::Right),
        ("Bottom", Placement::Bottom),
        ("Left", Placement::Left),
        ("Bottom start", Placement::BottomStart),
        ("Bottom end", Placement::BottomEnd),
    ];

    html! {
        <Cluster>
            { for sides.into_iter().map(|(label, placement)| html! {
                <Popover content={format!("Placed with Placement::{placement:?}")} {placement}>
                    <Button>{ label }</Button>
                </Popover>
            }) }
        </Cluster>
    }
}
