use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <Cluster gap={Space::Lg}>
                <Cluster gap={Space::Xs} align={Align::Center}>{ "Hosts" }<Badge count=31 /></Cluster>
                <Cluster gap={Space::Xs} align={Align::Center}>{ "Healthy" }<Badge count=28 tone={Tone::Ok} /></Cluster>
                <Cluster gap={Space::Xs} align={Align::Center}>{ "Held" }<Badge count=2 tone={Tone::Warn} /></Cluster>
                <Cluster gap={Space::Xs} align={Align::Center}>{ "Failing" }<Badge count=1 tone={Tone::Danger} /></Cluster>
            </Cluster>
            <Cluster gap={Space::Lg}>
                <Cluster gap={Space::Xs} align={Align::Center}>{ "Inbox" }<Badge count=4 tone={Tone::Info} bold=true /></Cluster>
                <Cluster gap={Space::Xs} align={Align::Center}>{ "Incidents" }<Badge count=1 tone={Tone::Danger} bold=true /></Cluster>
                <Cluster gap={Space::Xs} align={Align::Center}>{ "Drafts" }<Badge count=12 bold=true /></Cluster>
            </Cluster>
        </Stack>
    }
}
