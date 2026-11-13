//! Pages that only make sense without a client: a button that opens a dialog
//! by `command`, a confirmation that submits a native form. They are
//! prerendered under `/_static/server-only/<name>` for `server_only.spec.ts`
//! and are not part of the documentation, because hydrating them would put
//! two owners in charge of the same dialog.

use yew::prelude::*;
use yewi::prelude::*;

/// The page these are filed under: `/_static/server-only/<name>`.
pub const PAGE: &str = "server-only";

pub struct Fixture {
    pub name: &'static str,
    render: fn() -> Html,
}

pub const FIXTURES: &[Fixture] =
    &[Fixture { name: "confirm", render: confirm }, Fixture { name: "dialogs", render: dialogs }];

pub fn find(page: &str, name: &str) -> Option<Html> {
    FIXTURES.iter().find(|fixture| page == PAGE && fixture.name == name).map(|fixture| (fixture.render)())
}

fn confirm() -> Html {
    html! {
        <Stack>
            <form id="forget" method="get">
                <input type="hidden" name="forget" value="pi-1" />
            </form>
            <Button variant={ButtonVariant::Danger} command={Command::show_modal("confirm-forget")}>
                { "Forget pi-1" }
            </Button>
            <ConfirmDialog
                id="confirm-forget"
                form="forget"
                title="Forget pi-1?"
                confirm_label="Forget pi-1"
                danger=true
                acknowledge="I have drained this host"
                type_to_confirm="pi-1"
            >
                { "Its history is deleted and it has to enrol again." }
            </ConfirmDialog>
        </Stack>
    }
}

fn dialogs() -> Html {
    html! {
        <Cluster>
            <Button command={Command::show_modal("details")}>{ "Details" }</Button>
            <Dialog id="details" title="Host details">{ "pi-1 has been up for 12 days." }</Dialog>
            <Button command={Command::show_modal("filters")}>{ "Filters" }</Button>
            <Drawer id="filters" title="Filters">{ "Nothing to filter by yet." }</Drawer>
            <Dialog id="notice" title="Session expired" open=true>{ "Sign in again to continue." }</Dialog>
        </Cluster>
    }
}
