use yew::prelude::*;
use yewi::prelude::*;

const ROWS: [(&str, [f64; 12], &str, SeriesColor); 3] = [
    ("Page views", [31., 34., 30., 38., 44., 41., 47., 52., 49., 58., 61., 66.], "66", SeriesColor::Auto),
    ("Sign-ups", [9., 8., 9., 7., 8., 9., 8., 8., 7., 9., 8., 8.], "8", SeriesColor::Muted),
    ("Errors", [1., 0., 0., 2., 1., 0., 3., 2., 6., 9., 14., 21.], "21", SeriesColor::Tone(Tone::Danger)),
];

#[function_component]
pub fn Example() -> Html {
    let rows = use_memo((), |_| ROWS.map(|(name, data, latest, color)| (name, SeriesData::from(data), latest, color)));

    html! {
        <Stack gap={Space::Sm}>
            { for rows.iter().map(|(name, data, latest, color)| html! {
                <Cluster nowrap=true>
                    <span style="inline-size: 6rem">{ *name }</span>
                    <span style="inline-size: 8rem">
                        <Sparkline data={data.clone()} color={*color} label={format!("{name}, last 12 hours")} />
                    </span>
                    <strong>{ *latest }</strong>
                </Cluster>
            }) }
        </Stack>
    }
}
