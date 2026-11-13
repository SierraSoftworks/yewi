use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let members = html! {
        <Stack gap={Space::Xs}>
            <span>{ "fra-1 · online" }</span>
            <span>{ "ams-2 · online" }</span>
            <span>{ "lhr-1 · unreachable" }</span>
        </Stack>
    };

    html! {
        <Popover keep_mounted=true label="Cluster members" content={members}>
            <Button>{ "3 members" }</Button>
        </Popover>
    }
}
