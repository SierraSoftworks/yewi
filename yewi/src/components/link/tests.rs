use yewi_test::render;

use super::*;
use crate::attrs;

#[test]
fn href_renders_an_anchor() {
    let html = render(|| html! { <Link href="/hosts">{ "Hosts" }</Link> });
    assert!(html.starts_with("<a"));
    assert!(html.contains(r#"href="/hosts""#));
    assert!(html.contains("Hosts"));
    assert!(!html.contains("target="));
}

#[test]
fn without_href_it_is_a_native_button() {
    let html = render(|| html! { <Link>{ "Retry" }</Link> });
    assert!(html.starts_with("<button"));
    assert!(html.contains(r#"type="button""#));
}

#[test]
fn external_opens_a_new_tab_safely_and_says_so() {
    let html = render(|| html! { <Link href="https://yew.rs" external=true>{ "Yew" }</Link> });
    assert!(html.contains(r#"target="_blank""#));
    assert!(html.contains("noopener"));
    assert!(html.contains("noreferrer"));
    assert!(html.contains("opens in a new tab"));
}

#[test]
fn a_new_tab_target_keeps_the_callers_rel() {
    let html = render(|| html! { <Link href="/doc.pdf" target="_blank" rel="author">{ "Doc" }</Link> });
    assert!(html.contains("author"));
    assert!(html.contains("noopener"));
}

#[test]
fn a_disabled_link_cannot_be_followed() {
    let html = render(|| html! { <Link href="/hosts" disabled=true>{ "Hosts" }</Link> });
    assert!(!html.contains("href="));
    assert!(html.contains(r#"aria-disabled="true""#));
    assert!(html.contains(r#"role="link""#));

    let button = render(|| html! { <Link disabled=true>{ "Retry" }</Link> });
    assert!(button.contains(" disabled"));
}

#[test]
fn every_variant_and_underline_is_distinguishable() {
    let mut rendered = Vec::new();
    for variant in LinkVariant::ALL {
        for underline in LinkUnderline::ALL {
            rendered.push(render(move || html! { <Link href="/" {variant} {underline}>{ "x" }</Link> }));
        }
    }
    for (i, a) in rendered.iter().enumerate() {
        for b in &rendered[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html = render(|| html! { <Link href="/" id="l" class="mine" attrs={attrs! { "download" => "x" }}>{ "x" }</Link> });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="l""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"download="x""#));
}

#[cfg(feature = "router")]
mod router {
    use yew::prelude::*;
    use yew_router::{
        history::{AnyHistory, History, MemoryHistory},
        prelude::*,
    };
    use yewi_test::render;

    use crate::RouteLink;

    #[derive(Clone, PartialEq, Routable)]
    enum Route {
        #[at("/")]
        Home,
        #[at("/hosts/:id")]
        Host { id: u32 },
    }

    fn render_at(path: &'static str, basename: Option<&'static str>) -> String {
        render(move || {
            let history = AnyHistory::from(MemoryHistory::new());
            history.push(format!("{}{path}", basename.unwrap_or_default()));
            html! {
                <Router {history} {basename}>
                    <RouteLink<Route> to={Route::Home}>{ "Home" }</RouteLink<Route>>
                    <RouteLink<Route> to={Route::Host { id: 7 }}>{ "Host 7" }</RouteLink<Route>>
                </Router>
            }
        })
    }

    #[test]
    fn renders_the_route_as_a_real_href() {
        let html = render_at("/", None);
        assert!(html.contains(r#"href="/""#));
        assert!(html.contains(r#"href="/hosts/7""#));
    }

    #[test]
    fn only_the_current_route_is_marked_current() {
        let html = render_at("/hosts/7", None);
        assert_eq!(html.matches(r#"aria-current="page""#).count(), 1);
        let host = &html[html.find(r#"href="/hosts/7""#).unwrap()..];
        assert!(host[..host.find('>').unwrap()].contains("aria-current"));
    }

    #[test]
    fn hrefs_include_the_router_basename() {
        let html = render_at("/", Some("/app"));
        assert!(html.contains(r#"href="/app""#));
        assert!(html.contains(r#"href="/app/hosts/7""#));
    }

    #[test]
    fn renders_outside_a_router() {
        let html = render(|| html! { <RouteLink<Route> to={Route::Host { id: 7 }}>{ "Host 7" }</RouteLink<Route>> });
        assert!(html.contains(r#"href="/hosts/7""#));
        assert!(!html.contains("aria-current"));
    }
}
