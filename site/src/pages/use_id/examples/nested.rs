use yew::prelude::*;
use yewi::prelude::*;

#[derive(Properties, PartialEq)]
struct LabelledProps {
    label: AttrValue,
}

#[function_component]
fn Labelled(props: &LabelledProps) -> Html {
    let id = use_id();
    html! {
        <Cluster>
            <label for={id.clone()}>{ props.label.clone() }</label>
            <input {id} class="yi-input" type="text" />
        </Cluster>
    }
}

#[derive(Properties, PartialEq)]
struct GroupProps {
    children: Html,
}

#[function_component]
fn Group(props: &GroupProps) -> Html {
    html! { <Stack>{ props.children.clone() }</Stack> }
}

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <Group>
                <Group><Labelled label="Street" /></Group>
                <Labelled label="City" />
            </Group>
            <Labelled label="Country" />
        </Stack>
    }
}
