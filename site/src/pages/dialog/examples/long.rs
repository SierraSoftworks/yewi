use yew::prelude::*;
use yewi::prelude::*;

const TERMS: [&str; 8] = [
    "Agents report host inventory, package versions and service health every 30 seconds.",
    "Reports are kept for 90 days, then reduced to daily summaries.",
    "Commands run on a host are recorded with the operator, the time and the exit status.",
    "Secrets are never included in a report. Values matching a known secret format are removed on the host.",
    "An agent that has not reported for 10 minutes is shown as unreachable.",
    "Removing a host deletes its reports and its command history.",
    "Agents update themselves from the controller, never from the internet.",
    "Access to a cluster is granted per team and can be withdrawn at any time.",
];

#[function_component]
pub fn Example() -> Html {
    let open = use_state(|| false);
    let outcome = use_state(|| "Not answered yet");

    let show = {
        let open = open.clone();
        Callback::from(move |_| open.set(true))
    };
    let answer = |text: &'static str| {
        let (open, outcome) = (open.clone(), outcome.clone());
        Callback::from(move |_| {
            outcome.set(text);
            open.set(false);
        })
    };

    let footer = html! {
        <>
            <Button on_click={answer("Declined")}>{ "Decline" }</Button>
            <Button variant={ButtonVariant::Primary} on_click={answer("Accepted")}>{ "Accept and enrol" }</Button>
        </>
    };

    html! {
        <Stack>
            <Cluster>
                <Button on_click={show}>{ "Enrol this cluster" }</Button>
            </Cluster>
            <span role="status">{ *outcome }</span>
            <Dialog open={*open} title="What enrolling a cluster means" dismissible=false {footer}>
                { for TERMS.iter().cycle().take(16).map(|term| html! { <p>{ *term }</p> }) }
            </Dialog>
        </Stack>
    }
}
