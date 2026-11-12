use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let segments: Rc<[BarSegment]> = Rc::from([
        BarSegment::new(212.5, Tone::Info, "GB volumes"),
        BarSegment::new(96.0, Tone::Warn, "GB snapshots"),
        BarSegment::new(41.5, Tone::Neutral, "GB logs"),
    ]);

    html! {
        <Stack gap={Space::Lg} attrs={attrs! { "style" => "max-inline-size: 26rem" }}>
            <SegmentBar label="Storage" segments={segments.clone()} total={500.0} rest_label="GB free" legend=true size={Size::Large} />
            <SegmentBar label="Storage" segments={segments.clone()} total={500.0} rest_label="GB free" />
            <SegmentBar label="Storage" {segments} total={500.0} rest_label="GB free" size={Size::Small} />
        </Stack>
    }
}
