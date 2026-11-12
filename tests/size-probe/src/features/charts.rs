use yew::prelude::*;
use yewi::prelude::*;

pub fn view() -> Html {
    let series = vec![
        Series::new("Visitors", [3.0, 5.0, 4.0, 8.0]).markers(true),
        Series::new("Previous", [2.0, 4.0, 5.0, 6.0]).color(SeriesColor::Muted).dash(SeriesDash::Dashed),
        Series::new("Errors", [0.0, 1.0, 0.0, 2.0]).kind(SeriesKind::Bars).color(SeriesColor::Tone(Tone::Danger)),
    ];
    html! {
        <>
            <Chart label="Visitors" {series} selectable=true />
            <Sparkline data={SeriesData::from([1.0, 3.0, 2.0])} />
        </>
    }
}
