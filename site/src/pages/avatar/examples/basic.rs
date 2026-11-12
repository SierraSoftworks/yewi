use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster>
            <Avatar name="Ada Lovelace" />
            <Avatar name="Grace Hopper" />
            <Avatar name="Alan Turing" />
            <Avatar name="Barbara Liskov" />
            <Avatar name="Edsger Dijkstra" />
            <Avatar name="deploy-bot" />
            <Avatar />
        </Cluster>
    }
}
