use yew::prelude::*;
use yewi::prelude::*;

const FIELDS: [(&str, &str); 3] =
    [("status:", "passing, failing or unknown"), ("region:", "where the probe runs"), ("tag:", "a label on the probe")];

#[function_component]
pub fn Example() -> Html {
    let query = use_state(String::new);

    let content = html! {
        <span style="display: grid; padding: var(--yi-space-xs)">
            { for FIELDS.into_iter().map(|(field, hint)| {
                let query = query.clone();
                let onclick = Callback::from(move |_| query.set(field.to_string()));
                html! {
                    <button
                        type="button"
                        {onclick}
                        style="all: unset; display: flex; gap: var(--yi-space-md); padding: var(--yi-space-xs) var(--yi-space-sm); border-radius: var(--yi-radius-sm); cursor: pointer"
                    >
                        <code>{ field }</code>
                        <span style="color: var(--yi-fg-muted)">{ hint }</span>
                    </button>
                }
            }) }
        </span>
    };

    html! {
        <Stack align={Align::Start}>
            <Popover
                trigger={PopoverTrigger::Focus}
                placement={Placement::BottomStart}
                arrow=false
                flush=true
                head="Filter by"
                {content}
                label="Filters"
            >
                <input
                    type="search"
                    aria-label="Filter probes"
                    placeholder="Filter probes"
                    value={(*query).clone()}
                    style="font: inherit; padding: var(--yi-space-xs) var(--yi-space-sm); inline-size: 16rem"
                />
            </Popover>
            <span role="status">{ if query.is_empty() { "No filter".to_string() } else { format!("Filtering by {}", *query) } }</span>
        </Stack>
    }
}
