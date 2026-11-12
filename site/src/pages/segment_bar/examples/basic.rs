use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let segments: Rc<[BarSegment]> =
        Rc::from([BarSegment::new(14.0, Tone::Ok, "done"), BarSegment::new(2.0, Tone::Danger, "failed")]);

    html! {
        <Stack gap={Space::Xs} attrs={attrs! { "style" => "max-inline-size: 22rem" }}>
            <Cluster justify={Justify::Between}>
                <span>{ "Rollout 2.14.0" }</span>
                <span style="color: var(--yi-fg-muted)">{ "16 of 24 hosts" }</span>
            </Cluster>
            <SegmentBar label="Hosts" {segments} total={24.0} rest_label="to go" />
        </Stack>
    }
}
