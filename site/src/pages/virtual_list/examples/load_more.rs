use std::{ops::Range, rc::Rc};

use yew::prelude::*;
use yewi::prelude::*;

const TOTAL: u32 = 1_000;
const PAGE: u32 = 100;

#[derive(Clone, PartialEq)]
struct Commit {
    number: u32,
    subject: &'static str,
}

/// Stands in for a request to the server: the commits `from..from + PAGE`.
fn fetch_page(from: u32) -> impl Iterator<Item = Commit> {
    const SUBJECTS: [&str; 4] =
        ["Fix flaky rollout test", "Bump dependencies", "Add region to audit events", "Tidy probe timeouts"];
    (from..(from + PAGE).min(TOTAL)).map(|i| Commit { number: TOTAL - i, subject: SUBJECTS[(i * 3 % 4) as usize] })
}

#[function_component]
pub fn Example() -> Html {
    let items = use_state(|| fetch_page(0).collect::<Rc<[Commit]>>());
    let render = use_memo((), |()| {
        RowFn::new(|commit: &Commit| {
            html! {
                <Cluster gap={Space::Sm} nowrap=true>
                    <Text mono=true color={TextColor::Subtle}>{ format!("#{}", commit.number) }</Text>
                    <Text truncate=true>{ commit.subject }</Text>
                </Cluster>
            }
        })
    });

    let on_range = {
        let items = items.clone();
        Callback::from(move |drawn: Range<usize>| {
            let held = items.len();
            if drawn.end >= held && held < TOTAL as usize {
                items.set(items.iter().cloned().chain(fetch_page(held as u32)).collect());
            }
        })
    };

    html! {
        <Stack>
            <VirtualList<Commit>
                label="Commits"
                items={(*items).clone()}
                render={(*render).clone()}
                {on_range}
                max_height="18rem"
            />
            <span role="status">{ format!("Loaded {} of {} commits", items.len(), TOTAL) }</span>
        </Stack>
    }
}
