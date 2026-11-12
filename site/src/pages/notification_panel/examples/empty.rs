use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster justify={Justify::End}>
            <NotificationPanel
                title="Needs attention"
                label="Nothing needs attention"
                empty="Every host is healthy, connected and running the current agent."
            />
        </Cluster>
    }
}
