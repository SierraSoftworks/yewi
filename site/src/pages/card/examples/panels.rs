use yew::prelude::*;
use yewi::prelude::*;

const STAGES: [(&str, &str, &str); 3] = [
    ("Home", "4 hosts", "Done 12 minutes ago"),
    ("Staging", "9 hosts", "2 of 9 done"),
    ("Production", "18 hosts", "Waiting for approval"),
];

#[function_component]
pub fn Example() -> Html {
    html! {
        <Card
            title="Latest rollout"
            subtitle="2 of 31 hosts done"
            actions={html! { <Button variant={ButtonVariant::Primary}>{ "Approve" }</Button> }}
            attrs={attrs! { "style" => "max-inline-size: 28rem" }}
        >
            { for STAGES.into_iter().map(|(stage, hosts, progress)| html! {
                <Panel title={stage} subtitle={hosts}>{ progress }</Panel>
            }) }
        </Card>
    }
}
