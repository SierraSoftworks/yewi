use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <Alert tone={Tone::Danger} title="The agent could not reach the cluster">
                <p>{ "The last successful refresh was 4 minutes ago." }</p>
                <ul>
                    <li>{ "Check that the agent can resolve the cluster's address." }</li>
                    <li>{ "Confirm its token has not expired in " }<a href="#settings">{ "Settings" }</a>{ "." }</li>
                </ul>
            </Alert>
            <Alert tone={Tone::Warn} icon={icons::KEY_ROUND} title="This key is shown once">
                { "Copy it now. It cannot be recovered after you leave this page." }
            </Alert>
            <Alert hide_icon=true>{ "Changes to routing take up to a minute to apply." }</Alert>
        </Stack>
    }
}
