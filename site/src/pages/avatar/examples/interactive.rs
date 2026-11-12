use yew::prelude::*;
use yewi::prelude::*;

const PEOPLE: [&str; 3] = ["Ada Lovelace", "Grace Hopper", "Alan Turing"];

#[function_component]
pub fn Example() -> Html {
    let assignee = use_state(|| None::<&'static str>);

    html! {
        <Stack>
            <Cluster>
                { for PEOPLE.iter().map(|name| {
                    let assignee = assignee.clone();
                    let on_click = Callback::from(move |_| assignee.set(Some(*name)));
                    html! { <UserChip name={*name} variant={UserChipVariant::Soft} size={Size::Small} {on_click} /> }
                }) }
            </Cluster>
            <UserChip name="Barbara Liskov" detail="View profile" href="#barbara-liskov" />
            <span role="status">
                { match *assignee {
                    Some(name) => format!("Assigned to {name}"),
                    None => "Unassigned".to_owned(),
                } }
            </span>
        </Stack>
    }
}
