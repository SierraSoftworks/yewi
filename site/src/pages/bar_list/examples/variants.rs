use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let items = use_memo((), |_| {
        Rc::<[BarItem]>::from(vec![
            BarItem::labelled("Google", 3120.0),
            BarItem::labelled("Direct", 1874.0).muted(),
            BarItem::labelled("github.com", 1203.0),
            BarItem::labelled("news.ycombinator.com", 655.0),
            BarItem::labelled("duckduckgo.com", 214.0),
        ])
    });

    html! {
        <Grid min="16rem" gap={Space::Xl}>
            { for BarListVariant::ALL.into_iter().map(|variant| html! {
                <Stack gap={Space::Sm}>
                    <strong>{ format!("{variant:?}") }</strong>
                    <BarList
                        items={(*items).clone()}
                        label={format!("Referrers, bar {variant:?}")}
                        {variant}
                        show_share=true
                    />
                </Stack>
            }) }
        </Grid>
    }
}
