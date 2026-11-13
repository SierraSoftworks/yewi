use yewi_test::render;

use super::*;
use crate::{AppBar, Sidebar, SidebarItem, attrs};

fn shell(embedded: bool) -> String {
    render(move || {
        html! {
            <AppShell
                {embedded}
                bar={html! { <AppBar /> }}
                sidebar={html! { <Sidebar><SidebarItem href="/hosts">{ "Hosts" }</SidebarItem></Sidebar> }}
            >
                { "Page body" }
            </AppShell>
        }
    })
}

fn attr<'a>(html: &'a str, after: &str, name: &str) -> &'a str {
    let tag = &html[html.find(after).unwrap_or_else(|| panic!("no {after} in {html}"))..];
    tag.split(&format!("{name}=\"")).nth(1).unwrap().split('"').next().unwrap()
}

#[test]
fn renders_the_bar_the_navigation_and_the_page() {
    let html = shell(false);
    assert!(html.contains("<header"));
    assert!(html.contains("<nav"));
    assert!(html.contains("Hosts"));
    assert!(html.contains("Page body"));
}

#[test]
fn the_skip_link_targets_the_main_landmark() {
    let html = shell(false);
    let target = attr(&html, "<a", "href");
    assert!(html.contains("Skip to content"));
    let main = &html[html.find("<main").expect("a main landmark")..];
    assert_eq!(attr(main, "<main", "id"), &target[1..]);
}

#[test]
fn the_menu_button_controls_the_sidebar_and_reports_it_shown() {
    let html = shell(false);
    let controls = attr(&html, "<button", "aria-controls");
    assert_eq!(attr(&html, "<button", "aria-expanded"), "true");
    assert!(html.contains(&format!(r#"id="{controls}""#)));
}

#[test]
fn a_sidebar_that_starts_collapsed_is_reported_hidden() {
    let html = render(|| {
        html! {
            <AppShell collapsed=true bar={html! { <AppBar /> }} sidebar={html! { <Sidebar /> }} />
        }
    });
    assert_eq!(attr(&html, "<button", "aria-expanded"), "false");
}

#[test]
fn the_navigation_is_rendered_once_while_the_drawer_is_closed() {
    let html = shell(false);
    assert_eq!(html.matches("Hosts").count(), 1);
    assert!(html.contains("<dialog"));
}

#[test]
fn without_a_sidebar_there_is_no_menu_button_or_drawer() {
    let html = render(|| html! { <AppShell bar={html! { <AppBar /> }}>{ "Page body" }</AppShell> });
    assert!(!html.contains("<button"));
    assert!(!html.contains("<dialog"));
}

#[test]
fn an_embedded_shell_adds_no_page_landmarks() {
    let html = shell(true);
    assert!(!html.contains("<main"));
    assert!(!html.contains("Skip to content"));
    assert!(html.contains("Page body"));
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html = render(|| html! { <AppShell id="app" class="mine" attrs={attrs! { "data-app" => "buckle" }} /> });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="app""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"data-app="buckle""#));
}

#[test]
fn without_a_client_the_menu_button_opens_the_navigation_drawer() {
    let html = render(|| {
        html! {
            <crate::ServerOnly script="/yewi.js">
                <AppShell
                    storage_key="shell"
                    bar={html! { <AppBar /> }}
                    sidebar={html! { <Sidebar><SidebarItem href="/hosts">{ "Hosts" }</SidebarItem></Sidebar> }}
                />
            </crate::ServerOnly>
        }
    });
    let drawer = attr(&html, "<button", "commandfor");
    assert_eq!(attr(&html, "<button", "command"), "show-modal");
    assert!(html.contains("data-yi-shell-toggle"));
    assert_eq!(attr(&html, "<dialog", "id"), drawer);
    assert!(html.contains(r#"data-yi-collapse-key="shell""#));
    // The drawer holds the navigation before anything opens it.
    assert_eq!(html.matches("/hosts").count(), 2);
}

#[test]
fn a_hydrating_shell_declares_nothing_for_the_script() {
    let html = shell(false);
    assert!(!html.contains("commandfor"));
    assert!(!html.contains("data-yi-"));
}
