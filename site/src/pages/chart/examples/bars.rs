use yew::prelude::*;
use yewi::prelude::*;

const REQUESTS: [f64; 24] = [
    820., 790., 760., 742., 731., 768., 905., 1180., 1420., 1510., 1488., 1532., 1601., 1575., 1540., 1498., 1466.,
    1390., 1275., 1160., 1048., 960., 902., 861.,
];
const ERRORS: [f64; 24] =
    [0., 0., 1., 0., 0., 0., 2., 0., 3., 14., 9., 2., 0., 0., 1., 0., 0., 4., 1., 0., 0., 0., 0., 0.];

#[function_component]
pub fn Example() -> Html {
    let requests = use_memo((), |_| vec![Series::new("Requests", REQUESTS)]);
    let errors = use_memo((), |_| {
        vec![Series::new("Errors", ERRORS).kind(SeriesKind::Bars).color(SeriesColor::Tone(Tone::Danger))]
    });
    let x_label = use_callback((), |hour: usize, _| AttrValue::from(format!("{hour:02}:00")));
    // Each chart shows its crosshair at the hour the other is pointing at.
    let hour = use_state(|| None::<usize>);
    let on_hover = {
        let hour = hour.clone();
        Callback::from(move |at: Option<usize>| hour.set(at))
    };

    // Two measures on different scales: two charts over the same hours, with
    // the same axis width and banding so the hours line up. Only the lower
    // one labels them.
    html! {
        <Stack attrs={attrs! { "style" => "--yi-chart-axis-width: 2.5rem" }}>
            <Chart
                label="Requests per hour"
                series={(*requests).clone()}
                x_label={x_label.clone()}
                x_ticks=0
                banded=true
                cursor={*hour}
                on_hover={on_hover.clone()}
            />
            <Chart
                label="Errors per hour"
                series={(*errors).clone()}
                {x_label}
                integer_ticks=true
                cursor={*hour}
                {on_hover}
                attrs={attrs! { "style" => "--yi-chart-height: 6rem" }}
            />
        </Stack>
    }
}
