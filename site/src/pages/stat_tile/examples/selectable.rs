use yew::prelude::*;
use yewi::prelude::*;

const METRICS: [(&str, &str, &str, StatTrend); 3] = [
    ("Visitors", "12,480", "8%", StatTrend::Up),
    ("Pageviews", "31,902", "3%", StatTrend::Down),
    ("Events", "1,204", "21%", StatTrend::Up),
];

#[function_component]
pub fn Example() -> Html {
    let chosen = use_state(|| 0_usize);

    html! {
        <Stack>
            <Grid min="10rem">
                { for METRICS.into_iter().enumerate().map(|(index, (label, value, delta, trend))| {
                    let chosen = chosen.clone();
                    html! {
                        <StatTile
                            {label}
                            {value}
                            {delta}
                            {trend}
                            selected={*chosen == index}
                            on_click={Callback::from(move |_| chosen.set(index))}
                        />
                    }
                }) }
            </Grid>
            <span role="status">{ format!("Charting {}", METRICS[*chosen].0.to_lowercase()) }</span>
        </Stack>
    }
}
