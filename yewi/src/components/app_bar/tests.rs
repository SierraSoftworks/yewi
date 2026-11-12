use yew::prelude::*;
use yewi_test::render;

use super::*;
use crate::attrs;

#[test]
fn renders_a_header_with_its_brand_content_and_actions() {
    let html = render(|| {
        html! {
            <AppBar
                brand={html! { <AppBarBrand href="/">{ "Buckle" }</AppBarBrand> }}
                actions={html! { <button>{ "Sign out" }</button> }}
            >
                { "production" }
            </AppBar>
        }
    });
    assert!(html.starts_with("<header"));
    for text in ["Buckle", "production", "Sign out"] {
        assert!(html.contains(text), "{text} missing from {html}");
    }
    let (brand, actions) = (html.find("Buckle").unwrap(), html.find("Sign out").unwrap());
    assert!(brand < html.find("production").unwrap() && html.find("production").unwrap() < actions);
}

#[test]
fn outside_a_shell_there_is_no_menu_button() {
    let html = render(|| html! { <AppBar brand={html! { <AppBarBrand>{ "Buckle" }</AppBarBrand> }} /> });
    assert!(!html.contains("<button"));
}

#[test]
fn the_brand_is_a_link_only_with_an_href() {
    let link = render(|| html! { <AppBarBrand href="/" label="Buckle overview">{ "Buckle" }</AppBarBrand> });
    assert!(link.starts_with("<a"));
    assert!(link.contains(r#"href="/""#));
    assert!(link.contains(r#"aria-label="Buckle overview""#));

    let text = render(|| html! { <AppBarBrand>{ "Buckle" }</AppBarBrand> });
    assert!(!text.contains("<a"));
    assert!(text.contains("Buckle"));
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html = render(|| html! { <AppBar id="bar" class="mine" attrs={attrs! { "data-env" => "prod" }} /> });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="bar""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"data-env="prod""#));
}
