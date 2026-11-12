use yewi_test::render;

use super::*;
use crate::attrs;

fn user_menu(show_name: bool) -> String {
    render(move || {
        html! {
            <UserMenu name="Ada Lovelace" detail="admin" {show_name} on_sign_out={Callback::noop()}>
                <MenuItem>{ "Settings" }</MenuItem>
            </UserMenu>
        }
    })
}

/// The opening tag of the trigger button.
fn trigger(html: &str) -> &str {
    let start = html.find("<button").expect("a trigger button");
    &html[start..start + html[start..].find('>').unwrap()]
}

#[test]
fn the_trigger_is_a_closed_menu_button() {
    for show_name in [false, true] {
        let html = user_menu(show_name);
        let trigger = trigger(&html);
        assert!(trigger.contains(r#"aria-haspopup="menu""#), "{trigger}");
        assert!(trigger.contains(r#"aria-expanded="false""#), "{trigger}");
    }
}

#[test]
fn the_menu_is_not_rendered_until_opened() {
    let html = user_menu(false);
    assert!(!html.contains(r#"role="menu""#));
    assert!(!html.contains("Settings"));
    assert!(!html.contains("Sign out"));
}

#[test]
fn the_trigger_is_named_by_the_person_in_every_form() {
    for show_name in [false, true] {
        let html = user_menu(show_name);
        let button = &html[html.find("<button").unwrap()..html.find("</button>").unwrap()];
        assert!(button.contains("Ada Lovelace") && button.contains("admin"), "{button}");
    }
}

#[test]
fn class_id_and_attrs_reach_the_root() {
    let html = render(|| html! { <UserMenu name="x" class="mine" id="m" attrs={attrs! { "data-test" => "x" }} /> });
    let root = &html[..html.find('>').unwrap()];
    for expected in ["mine", r#"id="m""#, r#"data-test="x""#] {
        assert!(root.contains(expected), "{expected} missing from {root}");
    }
}
