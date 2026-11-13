use yew::prelude::*;
use yewi::SimpleTable;

use super::rust;

/// What a component can do on a page that loads no wasm.
struct Row {
    component: &'static str,
    works: &'static str,
    needs_client: &'static str,
    instead: &'static str,
}

const fn row(component: &'static str, works: &'static str, needs_client: &'static str, instead: &'static str) -> Row {
    Row { component, works, needs_client, instead }
}

/// Components with nothing to run, or whose behaviour is the browser's own.
const COMPLETE: &str = "AppBar, Badge, Breadcrumbs, Button, Card, Checkbox, DescriptionList, Divider, Duration, EmptyState, \
                        Field, Heading, Highlight, Icon, InlineCode, Kbd, Stack, Cluster, Grid, Link, Spinner, Loading, \
                        Markdown, PageHeader, Pagination, Panel, Pill, Pipeline, QrCode, Radio, SegmentBar, \
                        SegmentedControl, Sidebar, Skeleton, StatTile, StatusDot, Steps, TabNav, Terminal, Text, \
                        TextInput, TextArea, ThemeScript, ToggleSwitch, VisuallyHidden";

const PARTIAL: &[Row] = &[
    row(
        "AppShell",
        "The layout on a wide screen",
        "Hiding the sidebar, and all navigation on a narrow screen",
        "Repeat the destinations in a TopMenu with TopMenuCollapse::Never",
    ),
    row("TopMenu", "The row of links", "Groups and the collapsed dropdown", "Use TopMenuCollapse::Never and no groups"),
    row("Tabs", "The selected panel", "Switching panels", "TabNav, with one page per tab"),
    row(
        "Table, SimpleTable",
        "Rows and links",
        "Sorting, selection and expandable rows",
        "Sort on the server, using a link for each order",
    ),
    row(
        "JsonView",
        "The source, and nodes that start open",
        "Opening a folded node",
        "Use JsonViewVariant::Source, or open more levels up front",
    ),
    row(
        "Diff",
        "The diff",
        "Revealing folded lines and switching views",
        "Set the context large enough that nothing folds",
    ),
    row("CodeBlock", "The code and its highlighting", "Copying and the wrap toggle", "Set copy=false"),
    row(
        "NumberInput",
        "Typing and submitting",
        "The stepper, arrow keys and clamping",
        "Set stepper=false and validate on the server",
    ),
    row(
        "InputGroup, SearchInput",
        "Typing and submitting",
        "The clear button and attached dropdowns",
        "Use InputGroupSelect",
    ),
    row("SecretInput", "Typing and submitting", "The show toggle and the generator", "No alternative"),
    row(
        "FileDrop",
        "Opening the picker",
        "Reporting files, since it has no form name",
        "Use a native file input inside a Field",
    ),
    row(
        "Alert, Banner, Note, Tag",
        "Display and links",
        "Dismiss and remove buttons",
        "Leave out on_dismiss and on_remove",
    ),
    row(
        "Avatar",
        "Initials or the picture",
        "Falling back to initials when the picture fails to load",
        "Leave out src and email",
    ),
    row("BarList", "Bars and links", "Expanding, paging, filtering", "Pass every row you want to show"),
    row(
        "Timestamp, Countdown",
        "The text as rendered, in the server's time zone",
        "Ticking, local time and the tooltip",
        "Use TimestampDisplay::DateTime",
    ),
    row(
        "HistoryStrip, Timeline",
        "Cells, events and links",
        "Detail on hover",
        "Link each cell or event to its own page",
    ),
    row("Menu", "Items and links shown in the page", "Arrow keys and type-ahead", "No alternative"),
    row(
        "Chart, Sparkline",
        "The whole drawing",
        "Values on hover and range selection",
        "Put a table of the values beside it",
    ),
    row("Map", "The frame and its slots", "The map itself", "No alternative"),
    row("VirtualList, VirtualTable", "The first window of rows", "Everything after it", "Use a Table with Pagination"),
];

const INERT: &[Row] = &[
    row(
        "CopyButton, Copyable, SecretValue, SecretReveal",
        "",
        "Copying, revealing",
        "Show the value so the reader can select it",
    ),
    row("Dialog, ConfirmDialog, Drawer", "", "Opening and closing", "Use a separate confirmation page"),
    row(
        "MenuButton, SplitButton, UserMenu, NotificationPanel, Popover",
        "",
        "Opening",
        "Put links directly in the page",
    ),
    row("Tooltip", "", "Showing", "Use visible text, or a native title"),
    row("ThemeToggle", "", "Changing the theme", "ThemeScript on its own follows the system setting"),
    row("Select", "", "Opening", "Use InputGroupSelect, or a RadioGroup with a name"),
    row(
        "Combobox, TagPicker, TreeSelect, QueryInput",
        "",
        "Suggestions and choosing an option",
        "Use a TextInput, or Checkboxes with a name",
    ),
    row("SchemaForm", "", "Editing", "Write the fields by hand"),
    row("Toast", "", "Everything", "Render an Alert or Banner on the server"),
];

fn table(label: &'static str, rows: &'static [Row], with_works: bool) -> Html {
    html! {
        <SimpleTable {label} wrap=true>
            <thead>
                <tr>
                    <th>{ "Component" }</th>
                    if with_works {
                        <th>{ "What works" }</th>
                    }
                    <th>{ "What needs wasm" }</th>
                    <th>{ "What to use instead" }</th>
                </tr>
            </thead>
            <tbody>
                { for rows.iter().map(|row| html! {
                    <tr>
                        <th scope="row">{ row.component }</th>
                        if with_works {
                            <td>{ row.works }</td>
                        }
                        <td>{ row.needs_client }</td>
                        <td>{ row.instead }</td>
                    </tr>
                }) }
            </tbody>
        </SimpleTable>
    }
}

pub fn render() -> Html {
    html! {
        <>
            <p>
                { "A server-only app renders its pages with the " }<code>{ "ssr" }</code>
                { " feature and doesn't send any wasm to the browser. Every component still draws correctly, \
                   but anything interactive that relies on wasm won't work. This page goes through each \
                   component, explains what still works, and suggests what to use instead." }
            </p>

            <h2>{ "How server-only pages behave" }</h2>
            <ul>
                <li>{ "The browser holds the state. A prop such as " }<code>{ "checked" }</code>{ " or " }
                    <code>{ "value" }</code>{ " only sets the starting value, and " }<code>{ "on_*" }</code>
                    { " callbacks never run." }</li>
                <li>{ "Data reaches the server through forms, so give every control a " }<code>{ "name" }</code>
                    { " and put it inside a " }<code>{ "<form>" }</code>{ "." }</li>
                <li>{ "Navigation happens through links, so give Button, Pagination, TabNav and similar components an " }
                    <code>{ "href" }</code>{ "." }</li>
            </ul>
            { rust(r#"
<form method="post" action="/projects">
    <Field label="Name"><TextInput name="name" /></Field>
    <Checkbox name="private" label="Private" />
    <Button variant={ButtonVariant::Primary} kind={ButtonKind::Submit}>{ "Create" }</Button>
</form>
"#) }

            <h2>{ "Components that work fully" }</h2>
            <p>{ "These components either have nothing to run or rely only on the browser's own behaviour, so they work the same with or without wasm: " }{ COMPLETE }{ "." }</p>

            <h2>{ "Components that partly work" }</h2>
            <p>{ "These components render fully, but some of their behaviour needs wasm." }</p>
            { table("Components that partly work without wasm", PARTIAL, true) }

            <h2>{ "Components that need wasm" }</h2>
            <p>{ "These components render, but do nothing until wasm is running on the page." }</p>
            { table("Components that need wasm", INERT, false) }
        </>
    }
}
