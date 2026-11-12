use yew::prelude::*;
use yewi::prelude::*;

const VISITORS: [f64; 14] = [412., 388., 451., 520., 498., 301., 276., 465., 540., 612., 587., 634., 352., 318.];

#[function_component]
pub fn Example() -> Html {
    // Build the series once: the chart only recomputes when the data changes.
    let series = use_memo((), |_| vec![Series::new("Visitors", VISITORS)]);
    let x_label = use_callback((), |day: usize, _| AttrValue::from(format!("{} Jan", day + 1)));

    html! { <Chart label="Visitors per day" series={(*series).clone()} {x_label} /> }
}
