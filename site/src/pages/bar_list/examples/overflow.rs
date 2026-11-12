use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

const SECTIONS: [&str; 6] = ["docs", "blog", "guides", "api", "changelog", "help"];

/// 24 paths with a long tail, as an API would return the top of a breakdown.
fn paths() -> Rc<[BarItem]> {
    (0..24_u32)
        .map(|rank| {
            let path = format!("/{}/page-{:02}", SECTIONS[rank as usize % SECTIONS.len()], rank + 1);
            BarItem::labelled(path, (5200.0 / f64::from(rank + 1)).round())
        })
        .collect()
}

#[function_component]
pub fn Example() -> Html {
    let items = use_memo((), |_| paths());

    html! {
        <Grid min="16rem" gap={Space::Xl}>
            <Stack gap={Space::Sm}>
                <strong>{ "Expand" }</strong>
                <BarList
                    items={(*items).clone()}
                    label="Pages by views"
                    overflow={BarListOverflow::Expand(5)}
                    show_share=true
                    total={31_400.0}
                />
            </Stack>
            <Stack gap={Space::Sm}>
                <strong>{ "Pages" }</strong>
                <BarList
                    items={(*items).clone()}
                    label="Pages by visitors"
                    overflow={BarListOverflow::Pages(5)}
                    show_share=true
                    total={31_400.0}
                />
            </Stack>
        </Grid>
    }
}
