use yew::prelude::*;
use yewi::prelude::*;

const ACTIONS: [(&str, &str); 3] = [("Search", "/"), ("Command palette", "Ctrl+K"), ("Go to rollouts", "G R")];

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack gap={Space::Sm} attrs={attrs! { "style" => "max-inline-size: 18rem" }}>
            { for ACTIONS.into_iter().map(|(label, keys)| html! {
                <Cluster justify={Justify::Between} nowrap=true>
                    <Text>{ label }</Text>
                    <Kbd {keys} />
                </Cluster>
            }) }
        </Stack>
    }
}
