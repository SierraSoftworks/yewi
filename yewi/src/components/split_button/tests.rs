use yewi_test::render;

use super::*;
use crate::{MenuItem, attrs};

fn split(disabled: bool, loading: bool) -> String {
    render(move || {
        html! {
            <SplitButton label="Deploy" menu_label="More ways to deploy" {disabled} {loading}>
                <MenuItem>{ "Deploy to staging" }</MenuItem>
            </SplitButton>
        }
    })
}

#[test]
fn renders_the_action_and_a_named_menu_button_as_one_group() {
    let html = split(false, false);
    assert!(html.contains(r#"role="group""#));
    assert_eq!(html.matches("<button").count(), 2);
    assert!(html.contains("Deploy"));
    assert!(html.contains(r#"aria-label="More ways to deploy""#));
    assert!(html.contains(r#"aria-haspopup="menu""#));
    assert!(!html.contains("Deploy to staging"));
}

#[test]
fn without_other_actions_it_is_only_a_button() {
    let html = render(|| html! { <SplitButton label="Deploy" /> });
    assert_eq!(html.matches("<button").count(), 1);
    assert!(!html.contains("aria-haspopup"));
}

#[test]
fn face_disabled_leaves_the_menu_usable() {
    let html = render(|| {
        html! {
            <SplitButton label="Demote" face_disabled=true>
                <MenuItem>{ "Remove" }</MenuItem>
            </SplitButton>
        }
    });
    assert_eq!(html.matches(" disabled").count(), 1);
    let face = &html[html.find("<button").unwrap()..];
    assert!(face[..face.find('>').unwrap()].contains(" disabled"));
}

#[test]
fn disabled_turns_off_both_halves() {
    assert_eq!(split(true, false).matches(" disabled").count(), 2);
}

#[test]
fn loading_marks_only_the_action_busy() {
    let html = split(false, true);
    assert_eq!(html.matches(r#"aria-busy="true""#).count(), 1);
    assert!(!html.contains(" disabled"));
}

#[test]
fn class_id_and_attrs_reach_the_root() {
    let html = render(|| html! { <SplitButton label="x" class="mine" id="m" attrs={attrs! { "data-test" => "x" }} /> });
    let root = &html[..html.find('>').unwrap()];
    for expected in ["mine", r#"id="m""#, r#"data-test="x""#] {
        assert!(root.contains(expected), "{expected} missing from {root}");
    }
}
