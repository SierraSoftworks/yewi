use yewi_test::render;

use yew::html::IntoPropValue;

use super::*;
use crate::attrs;

/// The opening tag of the root element.
fn root(html: &str) -> &str {
    &html[..html.find('>').unwrap()]
}

#[test]
fn a_closed_dialog_renders_no_content() {
    let html = render(|| html! { <Dialog title="Edit project">{ "Body text" }</Dialog> });
    assert!(html.starts_with("<dialog"));
    assert!(!html.contains("Edit project"));
    assert!(!html.contains("Body text"));
}

#[test]
fn an_open_dialog_is_named_by_its_title() {
    let html = render(|| {
        html! {
            <Dialog open=true title="Edit project" footer={html! { <button>{ "Save" }</button> }}>
                { "Body text" }
            </Dialog>
        }
    });
    let labelled_by = root(&html).split("aria-labelledby=\"").nth(1).unwrap().split('"').next().unwrap();
    assert!(html.contains(&format!(r#"<h2 id="{labelled_by}""#)));
    assert!(html.contains("Edit project</h2>"));
    assert!(html.contains("Body text"));
    assert!(html.contains("Save"));
    assert!(html.contains(r#"aria-label="Close""#));
}

#[test]
fn an_empty_footer_draws_no_footer_row() {
    let empty: [(&str, Html); 3] =
        [("default", Html::default()), ("fragment", html! { <></> }), ("none", None::<Html>.into_prop_value())];
    for (case, footer) in empty {
        let html = render(move || html! { <Dialog open=true title="Edit project" footer={footer.clone()} /> });
        assert!(!html.contains("yi-dialog__foot"), "{case}");
    }
}

#[test]
fn the_open_state_is_left_to_the_browser() {
    // `showModal()` sets `open`; rendering it would make a non-modal dialog.
    let html = render(|| html! { <Dialog open=true title="Edit project" /> });
    assert!(!root(&html).contains(" open"));
}

#[test]
fn a_dialog_that_cannot_be_dismissed_has_no_close_button() {
    let html = render(|| html! { <Dialog open=true dismissible=false title="Saving" /> });
    assert!(!html.contains(r#"aria-label="Close""#));
}

#[test]
fn an_alert_dialog_is_described_by_its_body() {
    let html = render(|| html! { <Dialog open=true alert=true title="Discard changes?">{ "They are lost." }</Dialog> });
    let root = root(&html);
    assert!(root.contains(r#"role="alertdialog""#));
    let described_by = root.split("aria-describedby=\"").nth(1).unwrap().split('"').next().unwrap();
    assert!(html.contains(&format!(r#"<div id="{described_by}""#)));
    assert!(html.contains("They are lost."));
}

#[test]
fn every_size_is_distinguishable() {
    let rendered: Vec<String> = [Size::Small, Size::Medium, Size::Large]
        .into_iter()
        .map(|size| render(move || html! { <Dialog {size} title="Sized" /> }))
        .collect();
    assert_ne!(rendered[0], rendered[1]);
    assert_ne!(rendered[1], rendered[2]);
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    for html in [
        render(|| html! { <Dialog id="edit" class="mine" attrs={attrs! { "data-kind" => "form" }} title="Edit" /> }),
        render(|| {
            html! {
                <ConfirmDialog id="edit" class="mine" attrs={attrs! { "data-kind" => "form" }} title="Sure?"
                    confirm_label="Do it" />
            }
        }),
    ] {
        let root = root(&html);
        assert!(root.contains(r#"id="edit""#));
        assert!(root.contains("mine"));
        assert!(root.contains(r#"data-kind="form""#));
    }
}

#[test]
fn confirm_labels_its_buttons_with_the_action() {
    let html = render(|| {
        html! {
            <ConfirmDialog open=true title="Forget pi-1?" confirm_label="Forget pi-1" cancel_label="Keep it">
                { "Its history is removed." }
            </ConfirmDialog>
        }
    });
    assert!(html.contains(r#"role="alertdialog""#));
    assert!(html.contains("Its history is removed."));
    assert!(html.contains("Forget pi-1</button>"));
    assert!(html.contains("Keep it</button>"));
}

#[test]
fn confirm_shows_its_steps_and_error() {
    let html = render(|| {
        html! {
            <ConfirmDialog open=true title="Delete?" confirm_label="Delete" acknowledge="I have a backup"
                type_to_confirm="prod-db" error="The server refused." />
        }
    });
    // The steps are the shared controls, so they follow the theme.
    assert!(html.contains("yi-checkbox"));
    assert!(html.contains("yi-input"));
    assert!(html.contains("I have a backup"));
    assert!(html.contains("prod-db"));
    assert!(html.contains(r#"role="alert""#));
    assert!(html.contains("The server refused."));
}

#[test]
fn confirm_cannot_be_dismissed_while_busy() {
    let html = render(|| html! { <ConfirmDialog open=true busy=true title="Delete?" confirm_label="Delete" /> });
    assert!(!html.contains(r#"aria-label="Close""#));
    assert!(html.contains(r#"aria-busy="true""#));
}

#[test]
fn confirm_is_ready_only_when_every_condition_is_met() {
    // (needs acknowledge, acknowledged, word, typed, disabled) => ready
    let cases = [
        ((false, false, None, "", false), true),
        ((false, false, None, "", true), false),
        ((true, false, None, "", false), false),
        ((true, true, None, "", false), true),
        ((false, false, Some("prod-db"), "", false), false),
        ((false, false, Some("prod-db"), "prod", false), false),
        ((false, false, Some("prod-db"), " prod-db ", false), true),
        ((true, false, Some("prod-db"), "prod-db", false), false),
        ((true, true, Some("prod-db"), "prod-db", false), true),
        ((true, true, Some("prod-db"), "prod-db", true), false),
    ];
    for ((needs, acknowledged, word, typed, disabled), expected) in cases {
        assert_eq!(
            confirm_ready(needs, acknowledged, word, typed, disabled),
            expected,
            "{needs} {acknowledged} {word:?} {typed:?} {disabled}"
        );
    }
}
