use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

#[derive(Clone, PartialEq)]
struct Request {
    id: u32,
    route: &'static str,
    status: u16,
    millis: u32,
}

/// 500 deterministic rows.
fn requests() -> Rc<[Request]> {
    const ROUTES: [&str; 5] = ["/api/hosts", "/api/rollouts", "/api/audit", "/healthz", "/api/workloads/caddy"];
    const STATUSES: [u16; 7] = [200, 200, 200, 204, 304, 404, 502];
    (0..500_u32)
        .map(|i| Request {
            id: 48_000 + i,
            route: ROUTES[(i * 7 % 5) as usize],
            status: STATUSES[(i * 11 % 7) as usize],
            millis: 3 + (i * 37) % 240,
        })
        .collect()
}

#[function_component]
pub fn Example() -> Html {
    let rows = use_memo((), |()| requests());
    let columns = use_columns(|| {
        vec![
            Column::new("id", "Request", |request: &Request| request.id).sort_by_key(|r| r.id).width("6rem"),
            Column::new("route", "Route", |request: &Request| request.route).sort_text(|r| r.route),
            Column::new("status", "Status", |request: &Request| request.status)
                .sort_by_key(|r| r.status)
                .width("6rem"),
            Column::new("time", "Time", |request: &Request| format!("{} ms", request.millis))
                .sort_by_key(|r| r.millis)
                .numeric()
                .width("6rem"),
        ]
    });

    html! {
        <Table<Request>
            label="Recent requests"
            variant={TableVariant::Ruled}
            compact=true
            fixed=true
            sticky_header=true
            max_height="18rem"
            multi_sort=true
            {columns}
            rows={(*rows).clone()}
            row_key={RowFn::new(|request: &Request| request.id.to_string().into())}
            footer={html! { <><span>{ "500 requests" }</span><span>{ "Shift-click a header to add a tie-break" }</span></> }}
        />
    }
}
