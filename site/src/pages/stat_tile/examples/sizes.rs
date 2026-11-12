use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Grid min="11rem">
            <StatTile size={Size::Small} icon={icons::TIMER} label="p50 latency" value="38" unit="ms" />
            <StatTile size={Size::Medium} icon={icons::TIMER} label="p95 latency" value="184" unit="ms" />
            <StatTile size={Size::Large} icon={icons::TIMER} label="p99 latency" value="1.2" unit="s" tone={Tone::Warn} />
            <StatTile
                icon={icons::ACTIVITY}
                label="Probes"
                value="27"
                detail="View all probes"
                href="#sizes-units-and-links"
            />
        </Grid>
    }
}
