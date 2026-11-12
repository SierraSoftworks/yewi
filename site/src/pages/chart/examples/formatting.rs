use yew::prelude::*;
use yewi::prelude::*;

const REVENUE: [f64; 12] =
    [18_400., 21_250., 19_800., 24_100., 26_900., 25_300., 31_200., 29_750., 33_400., 38_100., 36_600., 41_900.];
const MONTHS: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

#[function_component]
pub fn Example() -> Html {
    let series = use_memo((), |_| vec![Series::new("Revenue", REVENUE)]);
    let x_label = use_callback((), |month: usize, _| AttrValue::from(MONTHS[month % 12]));
    let y_label = use_callback((), |value: f64, _| AttrValue::from(format!("€{}k", value / 1000.0)));

    html! {
        <Stack gap={Space::Xl}>
            <Chart label="Revenue per month" series={(*series).clone()} {x_label} {y_label} x_ticks=12 />
            <Chart label="Refunds per month" series={Vec::new()} empty="No refunds in this period" />
        </Stack>
    }
}
