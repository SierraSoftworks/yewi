use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster gap={Space::Xxxl} align={Align::Start}>
            <Timeline label="Marker shapes" run_off={TimelineRunOff::None}>
                <TimelineItem title="Dot" meta={html! { "No tone: an outline" }} />
                <TimelineItem title="Dot with a tone" tone={Tone::Ok} meta={html! { "Filled" }} />
                <TimelineItem title="Ring" marker={TimelineMarker::Ring} tone={Tone::Warn} meta={html! { "Not settled yet" }} />
                <TimelineItem
                    title="Diamond"
                    marker={TimelineMarker::Diamond}
                    tone={Tone::Info}
                    meta={html! { "Another kind of event" }}
                />
                <TimelineItem title="Active" tone={Tone::Active} active=true meta={html! { "Still going on" }} />
            </Timeline>
            <Timeline label="Icon markers" run_off={TimelineRunOff::None}>
                <TimelineItem title="Released 2.4.0" icon={icons::ROCKET} tone={Tone::Ok} meta={html! { "Today, 11:25" }} />
                <TimelineItem title="Build failed" icon={icons::X} tone={Tone::Danger} meta={html! { "Today, 10:02" }} />
                <TimelineItem title="Merged #218" icon={icons::GIT_MERGE} meta={html! { "Yesterday" }} />
                <TimelineItem title="Branch created" icon={icons::GIT_BRANCH} meta={html! { "Monday" }} />
            </Timeline>
        </Cluster>
    }
}
