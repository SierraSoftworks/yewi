use yew::prelude::*;
use yewi::prelude::*;

const P50: [f64; 12] = [42., 44., 41., 43., 47., 52., 49., 45., 44., 43., 46., 44.];
const P95: [f64; 12] = [88., 91., 86., 90., 104., 131., 118., 97., 92., 90., 95., 93.];
const P99: [f64; 12] = [140., 151., 139., 148., 190., 262., 221., 163., 150., 149., 158., 152.];
const TARGET: [f64; 12] = [200.; 12];

#[function_component]
pub fn Example() -> Html {
    let series = use_memo((), |_| {
        vec![
            Series::new("p50", P50).kind(SeriesKind::Line).markers(true),
            Series::new("p95", P95).markers(true),
            Series::new("p99", P99).dash(SeriesDash::Dotted),
            Series::new("Target", TARGET).color(SeriesColor::Muted).dash(SeriesDash::Dashed),
        ]
    });
    let x_label = use_callback((), |hour: usize, _| AttrValue::from(format!("{:02}:00", hour * 2)));
    let y_label = use_callback((), |ms: f64, _| AttrValue::from(format!("{ms} ms")));

    html! { <Chart label="Response time by percentile" series={(*series).clone()} {x_label} {y_label} /> }
}
