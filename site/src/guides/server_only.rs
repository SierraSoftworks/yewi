use yew::prelude::*;
use yewi::SimpleTable;

use super::rust;

/// What a component does on a page that loads no wasm.
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
        "Layout on a wide screen",
        "Hiding the sidebar; all navigation on a narrow screen",
        "Repeat the destinations in a TopMenu with TopMenuCollapse::Never",
    ),
    row("TopMenu", "The row of links", "Groups; the collapsed dropdown", "TopMenuCollapse::Never and no groups"),
    row("Tabs", "The selected panel", "Switching panels", "TabNav, one page per tab"),
    row(
        "Table, SimpleTable",
        "Rows and links",
        "Sorting, selection, expandable rows",
        "Sort on the server from a link",
    ),
    row(
        "JsonView",
        "Source; nodes open at render",
        "Opening a folded node",
        "JsonViewVariant::Source, or a larger open depth",
    ),
    row("Diff", "The diff", "Revealing folded lines; the view switch", "A context large enough to fold nothing"),
    row("CodeBlock", "Code and highlighting", "Copy; the wrap toggle", "copy=false"),
    row(
        "NumberInput",
        "Typing and submitting",
        "Stepper, arrow keys, clamping",
        "stepper=false; validate on the server",
    ),
    row("InputGroup, SearchInput", "Typing and submitting", "Clear button; attached dropdown", "NativeSelect"),
    row("SecretInput", "Typing and submitting", "Show toggle; generator", "None"),
    row("FileDrop", "Opening the picker", "Reporting files; it has no form name", "A native file input in a Field"),
    row("Alert, Banner, Note, Tag", "Display and links", "Dismiss and remove buttons", "Omit on_dismiss and on_remove"),
    row("Avatar", "Initials or the picture", "Falling back when the picture fails", "Omit src and email"),
    row("BarList", "Bars and links", "Expanding, paging, filtering", "Pass every row to show"),
    row(
        "Timestamp, Countdown",
        "The text as rendered, in the server's zone",
        "Ticking, local time, the tooltip",
        "TimestampDisplay::DateTime",
    ),
    row("HistoryStrip, Timeline", "Cells, events and links", "Detail on hover", "Link each cell or event to its page"),
    row("Menu", "Items and links, placed in the page", "Arrow keys and type-ahead", "None"),
    row("Chart, Sparkline", "The whole drawing", "Hover values; range selection", "A table of the values beside it"),
    row("Map", "The frame and its slots", "The map", "None"),
    row("VirtualList, VirtualTable", "The first window of rows", "Everything after it", "Table with Pagination"),
];

const INERT: &[Row] = &[
    row(
        "CopyButton, Copyable, SecretValue, SecretReveal",
        "",
        "Copying, revealing",
        "Show the value; the reader selects it",
    ),
    row("Dialog, ConfirmDialog, Drawer", "", "Opening and closing", "A confirmation page"),
    row("MenuButton, SplitButton, UserMenu, NotificationPanel, Popover", "", "Opening", "Links in the page"),
    row("Tooltip", "", "Showing", "Visible text, or a native title"),
    row("ThemeToggle", "", "Changing the theme", "ThemeScript alone follows the system"),
    row("Select", "", "Opening", "NativeSelect, or a RadioGroup with name"),
    row(
        "Combobox, TagPicker, TreeSelect, QueryInput",
        "",
        "Suggestions and choosing",
        "TextInput, or Checkboxes with name",
    ),
    row("SchemaForm", "", "Editing", "Fields written by hand"),
    row("Toast", "", "Everything", "An Alert or Banner rendered by the server"),
];

fn table(label: &'static str, rows: &'static [Row], with_works: bool) -> Html {
    html! {
        <SimpleTable {label} wrap=true>
            <thead>
                <tr>
                    <th>{ "Component" }</th>
                    if with_works {
                        <th>{ "Works" }</th>
                    }
                    <th>{ "Needs a client" }</th>
                    <th>{ "Until then" }</th>
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
                { "A server-only app renders with the " }<code>{ "ssr" }</code>
                { " feature and ships no wasm. Every component draws correctly that way; what differs is \
                   which behaviour survives. This page lists it per component." }
            </p>

            <h2>{ "The rules" }</h2>
            <ul>
                <li>{ "State lives in the browser. A prop such as " }<code>{ "checked" }</code>{ " or " }
                    <code>{ "value" }</code>{ " is the starting value, and no " }<code>{ "on_*" }</code>
                    { " callback ever runs." }</li>
                <li>{ "Data leaves through forms. Give every control a " }<code>{ "name" }</code>
                    { " and put it in a " }<code>{ "<form>" }</code>{ "." }</li>
                <li>{ "Navigation is links. Give Button, Pagination, TabNav and the like an " }
                    <code>{ "href" }</code>{ "." }</li>
            </ul>
            { rust(r#"
<form method="post" action="/projects">
    <Field label="Name"><TextInput name="name" /></Field>
    <Checkbox name="private" label="Private" />
    <Button variant={ButtonVariant::Primary} kind={ButtonKind::Submit}>{ "Create" }</Button>
</form>
"#) }

            <h2>{ "Works as it is" }</h2>
            <p>{ COMPLETE }{ "." }</p>

            <h2>{ "Works in part" }</h2>
            { table("Components that work in part without a client", PARTIAL, true) }

            <h2>{ "Needs a client" }</h2>
            { table("Components that do nothing without a client", INERT, false) }
        </>
    }
}
