use yew::prelude::*;
use yewi::prelude::*;

const THIS_WEEK: [f64; 14] = [412., 388., 451., 520., 498., 301., 276., 465., 540., 612., 587., 634., 352., 318.];
const LAST_WEEK: [f64; 14] = [380., 402., 419., 440., 462., 290., 248., 401., 455., 498., 530., 541., 322., 287.];

#[function_component]
pub fn Example() -> Html {
    let hovered = use_state(|| None::<usize>);
    let series = use_memo((), |_| {
        vec![
            Series::new("Visitors", THIS_WEEK),
            Series::new("Previous period", LAST_WEEK).color(SeriesColor::Muted).dash(SeriesDash::Dashed),
        ]
    });
    let x_label = use_callback((), |day: usize, _| AttrValue::from(format!("{} Jan", day + 1)));
    let on_hover = {
        let hovered = hovered.clone();
        Callback::from(move |day| hovered.set(day))
    };

    html! {
        <Stack>
            <Chart label="Visitors per day" series={(*series).clone()} {x_label} {on_hover} />
            <span role="status">
                { match *hovered {
                    Some(day) => format!("Reading {} Jan", day + 1),
                    None => "Not reading a day".to_owned(),
                } }
            </span>
        </Stack>
    }
}
