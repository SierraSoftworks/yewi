use yew::prelude::*;
use yewi_test::render;

use super::*;
use crate::{attrs, icons};

#[test]
fn a_menu_is_a_labelled_list_of_menu_items() {
    let html = render(|| {
        html! {
            <Menu label="Dashboard">
                <MenuItem>{ "Rename" }</MenuItem>
                <MenuItem icon={icons::COPY}>{ "Duplicate" }</MenuItem>
            </Menu>
        }
    });
    assert!(html.contains(r#"role="menu""#));
    assert!(html.contains(r#"aria-label="Dashboard""#));
    assert_eq!(html.matches(r#"role="menuitem""#).count(), 2);
    assert!(html.find("Rename").unwrap() < html.find("Duplicate").unwrap());
}

#[test]
fn an_item_is_a_button_unless_it_has_somewhere_to_go() {
    let button = render(|| html! { <MenuItem>{ "Rename" }</MenuItem> });
    assert!(button.starts_with("<button"), "{button}");
    assert!(button.contains(r#"type="button""#));

    let link = render(|| html! { <MenuItem href="/docs" target="_blank">{ "Docs" }</MenuItem> });
    assert!(link.starts_with("<a"), "{link}");
    assert!(link.contains(r#"href="/docs""#));
    assert!(link.contains("noopener"));
    assert!(link.contains(r#"role="menuitem""#));
}

#[test]
fn a_checked_item_is_an_option_that_reports_its_state() {
    for (checked, state) in [(true, r#"aria-checked="true""#), (false, r#"aria-checked="false""#)] {
        let html = render(move || html! { <MenuItem {checked}>{ "Wrap lines" }</MenuItem> });
        assert!(html.contains(r#"role="menuitemcheckbox""#), "{html}");
        assert!(html.contains(state), "{html}");
    }
    let plain = render(|| html! { <MenuItem>{ "Wrap lines" }</MenuItem> });
    assert!(!plain.contains("aria-checked"));
}

#[test]
fn a_disabled_item_is_announced_and_cannot_be_followed() {
    let cases = [
        render(|| html! { <MenuItem disabled=true>{ "Export" }</MenuItem> }),
        render(|| html! { <MenuItem disabled=true href="/export">{ "Export" }</MenuItem> }),
    ];
    for html in cases {
        assert!(html.contains(r#"aria-disabled="true""#), "{html}");
        assert!(!html.contains("href="), "{html}");
    }
}

#[test]
fn tones_that_carry_a_warning_are_distinguishable() {
    let tone = |tone| render(move || html! { <MenuItem {tone}>{ "Forget" }</MenuItem> });
    let (neutral, warn, danger) = (tone(Tone::Neutral), tone(Tone::Warn), tone(Tone::Danger));
    assert_ne!(neutral, warn);
    assert_ne!(neutral, danger);
    assert_ne!(warn, danger);
}

#[test]
fn aside_follows_the_label() {
    let html = render(|| html! { <MenuItem aside="systemd">{ "Restart" }</MenuItem> });
    assert!(html.find("Restart").unwrap() < html.find("systemd").unwrap());
}

#[test]
fn a_group_is_named_by_its_heading() {
    let html = render(|| {
        html! {
            <Menu>
                <MenuGroup label="Actions"><MenuItem>{ "Restart" }</MenuItem></MenuGroup>
                <MenuSeparator />
                <MenuGroup><MenuItem>{ "Forget" }</MenuItem></MenuGroup>
            </Menu>
        }
    });
    assert_eq!(html.matches(r#"role="group""#).count(), 2);
    assert_eq!(html.matches(r#"aria-label="Actions""#).count(), 1);
    assert!(html.contains(r#"role="separator""#));
}

#[test]
fn class_id_and_attrs_reach_the_root() {
    let cases = [
        render(|| html! { <Menu class="mine" id="m" attrs={attrs! { "data-test" => "x" }} /> }),
        render(|| html! { <MenuItem class="mine" id="m" attrs={attrs! { "data-test" => "x" }} /> }),
        render(|| html! { <MenuGroup class="mine" id="m" attrs={attrs! { "data-test" => "x" }} /> }),
        render(|| html! { <MenuSeparator class="mine" id="m" attrs={attrs! { "data-test" => "x" }} /> }),
    ];
    for html in cases {
        let root = &html[..html.find('>').unwrap()];
        for expected in ["mine", r#"id="m""#, r#"data-test="x""#] {
            assert!(root.contains(expected), "{expected} missing from {root}");
        }
    }
}
