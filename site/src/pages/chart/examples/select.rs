use yew::prelude::*;
use yewi::prelude::*;

const VISITORS: [f64; 14] = [412., 388., 451., 520., 498., 301., 276., 465., 540., 612., 587., 634., 352., 318.];

#[function_component]
pub fn Example() -> Html {
    let range = use_state(|| None::<(usize, usize)>);
    let series = use_memo((), |_| vec![Series::new("Visitors", VISITORS)]);
    let x_label = use_callback((), |day: usize, _| AttrValue::from(format!("{} Jan", day + 1)));
    let on_select = {
        let range = range.clone();
        Callback::from(move |selected| range.set(Some(selected)))
    };

    html! {
        <Stack>
            <Chart label="Visitors per day" series={(*series).clone()} {x_label} selectable=true {on_select} />
            <span role="status">
                { match *range {
                    Some((from, to)) => format!("Selected {} Jan to {} Jan", from + 1, to + 1),
                    None => "Drag across the chart to select days".to_owned(),
                } }
            </span>
        </Stack>
    }
}
