use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

#[derive(Clone, PartialEq)]
struct Run {
    task: &'static str,
    ok: bool,
    duration: &'static str,
    host: &'static str,
    output: &'static str,
}

const RUNS: [Run; 3] = [
    Run { task: "Rotate certificates", ok: true, duration: "12 s", host: "web-01", output: "3 certificates renewed" },
    Run { task: "Prune images", ok: true, duration: "48 s", host: "web-02", output: "Freed 2.1 GB" },
    Run { task: "Back up database", ok: false, duration: "4 min", host: "db-01", output: "Upload timed out after 240 s" },
];

#[function_component]
pub fn Example() -> Html {
    let rows = use_memo((), |()| Rc::<[Run]>::from(RUNS));
    let columns = use_columns(|| {
        vec![
            Column::new("task", "Task", |run: &Run| run.task).row_header(),
            Column::new("result", "Result", |run: &Run| {
                if run.ok {
                    html! { <Pill tone={Tone::Ok}>{ "Succeeded" }</Pill> }
                } else {
                    html! { <Pill tone={Tone::Danger}>{ "Failed" }</Pill> }
                }
            }),
            Column::new("duration", "Duration", |run: &Run| run.duration).numeric(),
        ]
    });
    let task = use_memo((), |()| RowFn::new(|run: &Run| AttrValue::from(run.task)));
    let detail = use_memo((), |()| {
        RowFn::new(|run: &Run| {
            html! {
                <DescriptionList>
                    <DescriptionItem term="Host">{ run.host }</DescriptionItem>
                    <DescriptionItem term="Output">{ run.output }</DescriptionItem>
                </DescriptionList>
            }
        })
    });

    html! {
        <Table<Run>
            label="Task runs"
            {columns}
            rows={(*rows).clone()}
            row_key={(*task).clone()}
            row_label={(*task).clone()}
            detail={(*detail).clone()}
        />
    }
}
