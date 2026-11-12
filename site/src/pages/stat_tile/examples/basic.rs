use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Grid min="11rem">
            <StatTile label="Visitors" value="12,480" delta="8%" trend={StatTrend::Up} detail="vs. previous 7 days" />
            <StatTile label="Pageviews" value="31,902" delta="3%" trend={StatTrend::Down} detail="vs. previous 7 days" />
            <StatTile
                label="Bounce rate"
                value="42"
                unit="%"
                delta="5%"
                trend={StatTrend::Down}
                lower_is_better=true
                detail="vs. previous 7 days"
            />
            <StatTile label="Median time" value="1m 24s" delta="0%" trend={StatTrend::Flat} detail="vs. previous 7 days" />
            <StatTile label="Events" value="–" detail="No events recorded" />
        </Grid>
    }
}
