use yew::prelude::*;
use yewi::prelude::*;

// An app would set these once in its stylesheet, on `.yi-stat-tile`.
const UPPERCASE: &str = "--yi-stat-tile-label-size: var(--yi-text-xs); \
                         --yi-stat-tile-label-transform: uppercase; \
                         --yi-stat-tile-label-tracking: 0.06em";

#[function_component]
pub fn Example() -> Html {
    let tiles = |style: &'static str| {
        let attrs = attrs! { "style" => style };
        html! {
            <Grid min="10rem">
                <StatTile attrs={attrs.clone()} label="Accounts" value="4" />
                <StatTile attrs={attrs.clone()} label="Administrators" value="1" />
                <StatTile {attrs} label="Suspended" value="1" detail="Cannot sign in or connect." />
            </Grid>
        }
    };

    html! {
        <Stack gap={Space::Lg}>
            { tiles("") }
            { tiles(UPPERCASE) }
        </Stack>
    }
}
