use yewi_test::render;

use super::*;
use crate::{NavTarget, attrs, icons};

#[test]
fn is_a_named_navigation_landmark() {
    let html = render(|| html! { <Sidebar label="Sections"><SidebarItem href="/hosts">{ "Hosts" }</SidebarItem></Sidebar> });
    assert!(html.starts_with("<nav"));
    assert!(html.contains(r#"aria-label="Sections""#));
    assert!(html.contains("Hosts"));
}

#[test]
fn a_group_is_named_by_its_label() {
    let html = render(|| {
        html! { <SidebarGroup label="Clients"><SidebarItem href="/users">{ "Users" }</SidebarItem></SidebarGroup> }
    });
    assert!(html.contains(r#"role="group""#));
    assert!(html.contains(r#"aria-label="Clients""#));
    assert!(html.contains(">Clients<"));
}

#[test]
fn the_current_item_is_announced() {
    for (current, expected) in [(true, true), (false, false)] {
        let link = render(move || html! { <SidebarItem href="/hosts" {current}>{ "Hosts" }</SidebarItem> });
        assert_eq!(link.contains(r#"aria-current="page""#), expected, "{link}");
        let button = render(move || html! { <SidebarItem {current}>{ "Hosts" }</SidebarItem> });
        assert_eq!(button.contains("aria-current="), expected, "{button}");
    }
}

#[test]
fn an_item_is_a_link_with_an_href_and_a_button_without() {
    let link = render(|| html! { <SidebarItem href="/hosts" icon={icons::SERVER}>{ "Hosts" }</SidebarItem> });
    assert!(link.starts_with("<a"));
    assert!(link.contains(r#"href="/hosts""#));
    assert!(!link.contains("target="));

    let button = render(|| html! { <SidebarItem>{ "Hosts" }</SidebarItem> });
    assert!(button.starts_with("<button"));
    assert!(button.contains(r#"type="button""#));
}

#[test]
fn an_external_item_opens_a_new_tab_and_says_so() {
    let html = render(|| {
        html! { <SidebarItem href={NavTarget::href("https://example.com").external()}>{ "Documentation" }</SidebarItem> }
    });
    assert!(html.contains(r#"target="_blank""#));
    assert!(html.contains("noopener"));
    assert!(html.contains("new tab"));
}

#[test]
fn trailing_content_and_footer_are_rendered() {
    let html = render(|| {
        html! {
            <Sidebar footer={html! { "v1.8.2" }}>
                <SidebarItem href="/rollouts" trailing={html! { "3" }}>{ "Rollouts" }</SidebarItem>
            </Sidebar>
        }
    });
    assert!(html.find("Rollouts").unwrap() < html.rfind(">3<").expect("the trailing count"));
    assert!(html.contains("v1.8.2"));
}

#[test]
fn id_class_and_attrs_reach_each_root_element() {
    let rendered = [
        render(|| html! { <Sidebar id="x" class="mine" attrs={attrs! { "data-k" => "v" }} /> }),
        render(|| html! { <SidebarGroup label="G" id="x" class="mine" attrs={attrs! { "data-k" => "v" }} /> }),
        render(|| html! { <SidebarItem id="x" class="mine" attrs={attrs! { "data-k" => "v" }} /> }),
    ];
    for html in rendered {
        let root = &html[..html.find('>').unwrap()];
        assert!(root.contains(r#"id="x""#) && root.contains("mine") && root.contains(r#"data-k="v""#), "{root}");
    }
}

#[cfg(feature = "router")]
mod router {
    use yew::prelude::*;
    use yew_router::{
        history::{AnyHistory, History, MemoryHistory},
        prelude::*,
    };
    use yewi_test::render;

    use crate::{NavTarget, SidebarItem};

    #[derive(Clone, PartialEq, Routable)]
    enum Route {
        #[at("/")]
        Home,
        #[at("/hosts")]
        Hosts,
    }

    fn render_at(path: &'static str, hosts_current: Option<bool>) -> String {
        render(move || {
            let history = AnyHistory::from(MemoryHistory::new());
            history.push(path);
            html! {
                <Router {history}>
                    <SidebarItem href={NavTarget::route(Route::Home)}>{ "Home" }</SidebarItem>
                    <SidebarItem href={NavTarget::route(Route::Hosts)} current={hosts_current}>{ "Hosts" }</SidebarItem>
                </Router>
            }
        })
    }

    #[test]
    fn a_route_item_is_a_real_link_marked_current_on_its_route() {
        let html = render_at("/hosts", None);
        assert!(html.contains(r#"href="/hosts""#));
        assert_eq!(html.matches(r#"aria-current="page""#).count(), 1);
        let hosts = &html[html.find(r#"href="/hosts""#).unwrap()..];
        assert!(hosts[..hosts.find('>').unwrap()].contains("aria-current"));
    }

    #[test]
    fn current_overrides_the_router() {
        assert_eq!(render_at("/", Some(true)).matches(r#"aria-current="page""#).count(), 2);
        assert!(!render_at("/hosts", Some(false)).contains("aria-current"));
    }
}
