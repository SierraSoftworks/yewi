use yew::prelude::*;
use yewi::prelude::*;

const POINTS: usize = 200_000;

/// A deterministic noisy signal with a few spikes, standing in for a metric.
fn signal() -> SeriesData {
    let mut seed = 42_u32;
    (0..POINTS)
        .map(|i| {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let noise = f64::from(seed >> 16) / 65_536.0;
            let wave = (i as f64 / 9_000.0).sin() * 120.0 + (i as f64 / 1_300.0).sin() * 30.0;
            let spike = if i % 37_717 == 0 && i > 0 { 400.0 } else { 0.0 };
            300.0 + wave + noise * 60.0 + spike
        })
        .collect()
}

#[function_component]
pub fn Example() -> Html {
    let series = use_memo((), |_| vec![Series::new("Latency", signal())]);
    let x_label = use_callback((), |sample: usize, _| AttrValue::from(format!("#{sample}")));
    let y_label = use_callback((), |ms: f64, _| AttrValue::from(format!("{} ms", ms.round())));

    html! {
        <Stack>
            <Chart label="Latency per request" series={(*series).clone()} {x_label} {y_label} x_ticks=5 />
            <span>{ format!("{POINTS} points, drawn as 1,000 buckets") }</span>
        </Stack>
    }
}
