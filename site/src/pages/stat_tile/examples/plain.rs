use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let variant = StatTileVariant::Plain;

    html! {
        <div class="demo-card" style="max-inline-size: 34rem">
            <Stack gap={Space::Lg}>
                <b class="demo-title">{ "Services" }</b>
                <Cluster gap={Space::Xxl} align={Align::Start}>
                    <StatTile {variant} label="Registered" value="14" />
                    <StatTile {variant} label="Reporting" value="11" tone={Tone::Ok} />
                    <StatTile
                        {variant}
                        label="Need attention"
                        value="3"
                        tone={Tone::Danger}
                        detail="Degraded, stopped, or not reporting."
                    />
                </Cluster>
            </Stack>
        </div>
    }
}
