use yewi_test::render;

use super::*;
use crate::attrs;

/// The opening tag of each `<tag ...>` in `html`.
fn tags<'a>(html: &'a str, tag: &str) -> Vec<&'a str> {
    let open = format!("<{tag} ");
    html.match_indices(&open).map(|(at, _)| &html[at..at + html[at..].find('>').unwrap()]).collect()
}

fn menu(collapse: TopMenuCollapse) -> String {
    render(move || {
        html! {
            <TopMenu label="Sections" {collapse}>
                <TopMenuItem href="/workflows" current=true>{ "Workflows" }</TopMenuItem>
                <TopMenuItem href="/activity">{ "Activity" }</TopMenuItem>
                <TopMenuItem href="https://example.com" external=true>{ "Docs" }</TopMenuItem>
                <TopMenuGroup label="Admin">
                    <MenuItem href="/accounts">{ "Accounts" }</MenuItem>
                </TopMenuGroup>
            </TopMenu>
        }
    })
}

#[test]
fn renders_a_named_landmark_of_links() {
    let html = menu(TopMenuCollapse::Never);
    assert!(tags(&html, "nav")[0].contains(r#"aria-label="Sections""#));
    let links = tags(&html, "a");
    assert_eq!(links.len(), 3);
    assert!(links[0].contains(r#"href="/workflows""#));
    assert!(html.contains("Workflows") && html.contains("Activity"));
}

#[test]
fn only_the_current_item_is_marked() {
    let html = menu(TopMenuCollapse::Never);
    let links = tags(&html, "a");
    assert!(links[0].contains(r#"aria-current="page""#));
    assert!(!links[1].contains("aria-current"));
}

#[test]
fn an_external_item_opens_a_new_tab_and_says_so() {
    let html = menu(TopMenuCollapse::Never);
    let docs = tags(&html, "a")[2];
    assert!(docs.contains(r#"target="_blank""#) && docs.contains("noopener"));
    assert!(html.contains("opens in a new tab"));
}

#[test]
fn a_group_is_a_closed_menu_button() {
    let html = menu(TopMenuCollapse::Never);
    let buttons = tags(&html, "button");
    assert_eq!(buttons.len(), 1);
    assert!(buttons[0].contains(r#"aria-haspopup="menu""#));
    assert!(buttons[0].contains(r#"aria-expanded="false""#));
    assert!(html.contains("Admin"));
    assert!(!html.contains("Accounts"), "the group's items are rendered only while open");
}

#[test]
fn collapse_decides_which_forms_are_rendered() {
    for collapse in TopMenuCollapse::ALL {
        let html = menu(collapse);
        let row = html.contains("Workflows");
        let button = html.contains("Menu");
        assert_eq!(row, collapse != TopMenuCollapse::Always, "{collapse:?} row");
        assert_eq!(button, collapse != TopMenuCollapse::Never, "{collapse:?} button");
    }
}

#[test]
fn actions_follow_the_row_and_wait_in_the_dropdown_until_it_opens() {
    let html = render(|| {
        html! {
            <TopMenu actions={html! { <span>{ "Signed in" }</span> }}>
                <TopMenuItem href="/a">{ "A" }</TopMenuItem>
            </TopMenu>
        }
    });
    assert_eq!(html.matches("Signed in").count(), 1);
    assert!(html.find("Signed in") > html.find(r#"href="/a""#));
}

#[test]
fn an_item_without_href_is_a_button() {
    let html = render(|| html! { <TopMenuItem current=true>{ "Overview" }</TopMenuItem> });
    assert!(html.contains("<button") && !html.contains("<a "));
    assert!(html.contains(r#"aria-current="page""#));
}

#[test]
fn class_id_and_attrs_reach_the_root() {
    let html = render(|| html! { <TopMenu class="mine" id="m" attrs={attrs! { "data-test" => "x" }} /> });
    let root = &html[..html.find('>').unwrap()];
    for expected in ["mine", r#"id="m""#, r#"data-test="x""#] {
        assert!(root.contains(expected), "{expected} missing from {root}");
    }
}
