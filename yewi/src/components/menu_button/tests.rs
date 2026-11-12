use yew::prelude::*;
use yewi_test::render;

use super::*;
use crate::{MenuItem, attrs};

fn menu_button(icon_only: bool, disabled: bool) -> String {
    render(move || {
        html! {
            <MenuButton label="Actions" {icon_only} {disabled}>
                <MenuItem>{ "Rename" }</MenuItem>
            </MenuButton>
        }
    })
}

#[test]
fn renders_a_closed_button_that_says_it_opens_a_menu() {
    let html = menu_button(false, false);
    assert!(html.contains("<button"));
    assert!(html.contains("Actions"));
    assert!(html.contains(r#"aria-haspopup="menu""#));
    assert!(html.contains(r#"aria-expanded="false""#));
}

#[test]
fn the_menu_is_not_rendered_until_opened() {
    let html = menu_button(false, false);
    assert!(!html.contains("Rename"));
    assert!(!html.contains(r#"role="menu""#));
}

#[test]
fn icon_only_keeps_the_label_as_the_accessible_name() {
    let html = menu_button(true, false);
    assert!(html.contains(r#"aria-label="Actions""#), "{html}");
    assert!(html.contains(r#"aria-haspopup="menu""#));
}

#[test]
fn disabled_uses_the_native_attribute() {
    for icon_only in [false, true] {
        assert!(menu_button(icon_only, true).contains(" disabled"));
        assert!(!menu_button(icon_only, false).contains(" disabled"));
    }
}

#[test]
fn class_id_and_attrs_reach_the_root() {
    let html = render(|| html! { <MenuButton label="x" class="mine" id="m" attrs={attrs! { "data-test" => "x" }} /> });
    let root = &html[..html.find('>').unwrap()];
    for expected in ["mine", r#"id="m""#, r#"data-test="x""#] {
        assert!(root.contains(expected), "{expected} missing from {root}");
    }
}
