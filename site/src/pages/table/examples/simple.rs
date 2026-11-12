use yew::prelude::*;
use yewi::prelude::*;

/// A host, its state and how long its update took in seconds.
type Host = (&'static str, &'static str, u32);

const STAGES: [(&str, [Host; 2]); 2] = [
    ("Canary", [("web-01", "Updated", 42), ("web-02", "Updated", 38)]),
    ("Fleet", [("db-01", "Waiting", 0), ("edge-ams", "Waiting", 0)]),
];

#[function_component]
pub fn Example() -> Html {
    let direction = use_state(|| SortDirection::Ascending);
    let on_sort = {
        let direction = direction.clone();
        Callback::from(move |click: SortClick| direction.set(click.direction))
    };

    html! {
        <SimpleTable label="Rollout by stage" variant={TableVariant::Ruled}>
            <thead>
                <tr>
                    <SortHeader direction={*direction} {on_sort}>{ "Host" }</SortHeader>
                    <th scope="col">{ "State" }</th>
                    <th scope="col" class="yi-table__cell--end">{ "Took" }</th>
                </tr>
            </thead>
            <tbody>
                { for STAGES.iter().map(|(stage, hosts)| {
                    let mut hosts = hosts.to_vec();
                    if *direction == SortDirection::Descending {
                        hosts.reverse();
                    }
                    html! {
                        <>
                            <tr class="yi-table__group">
                                <th scope="colgroup" colspan="3">{ *stage }</th>
                            </tr>
                            { for hosts.into_iter().map(|(host, state, seconds)| html! {
                                <tr class="yi-table__row">
                                    <th scope="row">{ host }</th>
                                    <td>{ state }</td>
                                    <td class="yi-table__cell--end">
                                        { if seconds > 0 { format!("{seconds} s") } else { "–".into() } }
                                    </td>
                                </tr>
                            }) }
                        </>
                    }
                }) }
            </tbody>
        </SimpleTable>
    }
}
