use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

#[derive(Clone, PartialEq)]
struct Request {
    id: u32,
    route: &'static str,
    region: &'static str,
    status: u16,
    millis: u32,
}

/// 100,000 deterministic rows.
fn requests() -> Rc<[Request]> {
    const ROUTES: [&str; 6] =
        ["/api/hosts", "/api/rollouts", "/api/audit", "/healthz", "/api/workloads/caddy", "/api/sessions"];
    const REGIONS: [&str; 4] = ["ams", "fra", "iad", "syd"];
    const STATUSES: [u16; 7] = [200, 200, 200, 204, 304, 404, 502];
    (0..100_000_u32)
        .map(|i| Request {
            id: 4_800_000 + i,
            route: ROUTES[(i * 7 % 6) as usize],
            region: REGIONS[(i * 13 % 4) as usize],
            status: STATUSES[(i * 11 % 7) as usize],
            millis: 3 + (i * 37) % 2_411,
        })
        .collect()
}

#[function_component]
pub fn Example() -> Html {
    let rows = use_memo((), |()| requests());
    let columns = use_columns(|| {
        vec![
            Column::new("id", "Request", |request: &Request| request.id)
                .row_header()
                .sort_by_key(|r| r.id)
                .width("7rem"),
            Column::new("route", "Route", |request: &Request| request.route).sort_text(|r| r.route).width("14rem"),
            Column::new("region", "Region", |request: &Request| request.region)
                .sort_text(|r| r.region)
                .width("6rem"),
            Column::new("status", "Status", |request: &Request| request.status)
                .sort_by_key(|r| r.status)
                .width("6rem"),
            Column::new("time", "Time", |request: &Request| format!("{} ms", request.millis))
                .sort_by_key(|r| r.millis)
                .numeric()
                .width("7rem"),
        ]
    });

    let sorted_by = use_state(|| String::from("id, ascending"));
    let on_sort = {
        let sorted_by = sorted_by.clone();
        Callback::from(move |sort: Vec<Sort>| {
            sorted_by.set(sort.iter().map(|s| format!("{}, {}", s.column, s.direction.as_str())).collect());
        })
    };

    html! {
        <Stack>
            <VirtualTable<Request>
                label="Requests"
                {columns}
                rows={(*rows).clone()}
                max_height="24rem"
                default_sort={Sort::asc("id")}
                {on_sort}
                footer={html! { <span>{ "100,000 requests" }</span> }}
            />
            <span role="status">{ format!("Sorted by {}", *sorted_by) }</span>
        </Stack>
    }
}
