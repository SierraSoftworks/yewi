use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

#[derive(Clone, PartialEq)]
struct Page {
    path: &'static str,
    views: u32,
    visitors: u32,
}

/// Stands in for the server: returns the rows in the order asked for.
fn fetch(sort: &[Sort]) -> Rc<[Page]> {
    let mut pages = vec![
        Page { path: "/", views: 18_204, visitors: 9_310 },
        Page { path: "/docs/install", views: 6_120, visitors: 4_877 },
        Page { path: "/pricing", views: 3_480, visitors: 3_012 },
        Page { path: "/blog/yew-0-23", views: 9_951, visitors: 8_206 },
    ];
    if let Some(sort) = sort.first() {
        pages.sort_by(|a, b| {
            sort.direction.apply(match sort.column.as_str() {
                "views" => a.views.cmp(&b.views),
                "visitors" => a.visitors.cmp(&b.visitors),
                _ => a.path.cmp(b.path),
            })
        });
    }
    pages.into()
}

#[function_component]
pub fn Example() -> Html {
    let sort = use_state(|| vec![Sort::desc("views")]);
    let rows = use_memo((*sort).clone(), |sort| fetch(sort));
    let columns = use_columns(|| {
        vec![
            Column::new("path", "Page", |page: &Page| html! { <InlineCode>{ page.path }</InlineCode> }).sortable(),
            Column::new("views", "Views", |page: &Page| page.views).sortable().numeric(),
            Column::new("visitors", "Visitors", |page: &Page| page.visitors).sortable().numeric(),
        ]
    });
    let on_sort = {
        let sort = sort.clone();
        Callback::from(move |next: Vec<Sort>| sort.set(next))
    };

    let query = sort.first().map_or_else(String::new, |sort| {
        format!("?sort={}{}", if sort.direction == SortDirection::Descending { "-" } else { "" }, sort.column)
    });

    html! {
        <Stack>
            <Table<Page> label="Pages" {columns} rows={(*rows).clone()} sort={(*sort).clone()} {on_sort} />
            <span role="status">{ format!("Requested /pages{query}") }</span>
        </Stack>
    }
}
