use yew::prelude::*;
use yewi_test::render;

use super::*;
use crate::attrs;

fn trail() -> String {
    render(|| {
        html! {
            <Breadcrumbs>
                <Crumb href="/">{ "Dashboard" }</Crumb>
                <Crumb href="/exceptions">{ "Exceptions" }</Crumb>
                <Crumb>{ "TimeoutError" }</Crumb>
            </Breadcrumbs>
        }
    })
}

#[test]
fn renders_a_named_landmark_holding_an_ordered_list() {
    let html = trail();
    assert!(html.starts_with("<nav") && html.contains(r#"aria-label="Breadcrumb""#));
    assert!(html.contains("<ol"));
    assert_eq!(html.matches("<li").count(), 3);
}

#[test]
fn crumbs_with_href_are_links_and_the_last_is_the_current_page() {
    let html = trail();
    assert!(html.contains(r#"href="/""#) && html.contains(r#"href="/exceptions""#));
    assert_eq!(html.matches("<a").count(), 2);
    assert_eq!(html.matches(r#"aria-current="page""#).count(), 1);
    let current = html.split("<li").last().unwrap();
    assert!(current.contains(r#"aria-current="page""#) && current.contains("TimeoutError"));
}

#[test]
fn current_can_be_overridden() {
    let html = render(|| {
        html! {
            <Breadcrumbs>
                <Crumb current=false>{ "Group" }</Crumb>
                <Crumb href="/here" current=true>{ "Here" }</Crumb>
            </Breadcrumbs>
        }
    });
    assert_eq!(html.matches(r#"aria-current="page""#).count(), 1);
    assert!(html.split("<li").last().unwrap().contains("aria-current"));
}

#[test]
fn separators_are_not_part_of_the_markup() {
    for separator in BreadcrumbSeparator::ALL {
        let html = render(move || {
            html! {
                <Breadcrumbs {separator}>
                    <Crumb href="/a">{ "A" }</Crumb>
                    <Crumb>{ "B" }</Crumb>
                </Breadcrumbs>
            }
        });
        let text: String = html.split('<').filter_map(|part| part.split_once('>')).map(|(_, text)| text).collect();
        assert_eq!(text, "AB");
    }
}

#[test]
fn the_label_id_class_and_attrs_reach_the_root_element() {
    let html = render(|| {
        html! {
            <Breadcrumbs label="You are here" id="trail" class="mine" attrs={attrs! { "data-test" => "x" }} />
        }
    });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"aria-label="You are here""#) && root.contains(r#"id="trail""#));
    assert!(root.contains("mine") && root.contains(r#"data-test="x""#));
}
