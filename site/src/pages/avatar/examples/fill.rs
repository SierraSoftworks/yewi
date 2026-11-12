use yew::prelude::*;
use yewi::prelude::*;

const PEOPLE: [&str; 6] =
    ["Ada Lovelace", "Grace Hopper", "Alan Turing", "Barbara Liskov", "Edsger Dijkstra", "Margaret Hamilton"];

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <Cluster>
                { for PEOPLE.iter().map(|name| html! { <Avatar name={*name} /> }) }
                <Avatar />
            </Cluster>
            <Cluster>
                { for PEOPLE.iter().map(|name| html! { <Avatar name={*name} bold=true /> }) }
                <Avatar bold=true />
            </Cluster>
        </Stack>
    }
}
