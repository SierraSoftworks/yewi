use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <Cluster gap={Space::Lg}>
                <Loading label="Loading avatar" label_hidden=true size={Size::Small} />
                <Loading label="Loading chart" label_hidden=true />
                <Loading label="Loading map" label_hidden=true size={Size::Large} />
            </Cluster>
            <span>
                <Spinner />{ " Syncing 3 of 12 repositories " }
                <span style="color: var(--yi-ok)"><Spinner size="1.5em" /></span>
            </span>
        </Stack>
    }
}
