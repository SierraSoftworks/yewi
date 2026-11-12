use yew::prelude::*;
use yewi_test::render;

use super::*;
use crate::attrs;

fn count(html: &str, needle: &str) -> usize {
    html.matches(needle).count()
}

#[test]
fn renders_a_native_description_list_with_grouped_pairs() {
    let html = render(|| {
        html! {
            <DescriptionList>
                <DescriptionItem term="Region">{ "eu-west-1" }</DescriptionItem>
                <DescriptionItem term="Hosts">{ "31" }</DescriptionItem>
            </DescriptionList>
        }
    });
    assert!(html.starts_with("<dl"));
    assert_eq!(count(&html, "<dt"), 2);
    assert_eq!(count(&html, "<dd"), 2);
    let (region, hosts) = (html.find("Region").unwrap(), html.find("Hosts").unwrap());
    let (first, second) = (html.find("eu-west-1").unwrap(), html.find("31").unwrap());
    assert!(region < first && first < hosts && hosts < second);
}

#[test]
fn terms_and_values_accept_markup() {
    let html = render(|| {
        html! {
            <DescriptionList>
                <DescriptionItem term={html! { <abbr title="Time to live">{ "TTL" }</abbr> }}>
                    <a href="/ttl">{ "90 days" }</a>
                </DescriptionItem>
            </DescriptionList>
        }
    });
    assert!(html.contains("<abbr"));
    assert!(html.contains(r#"href="/ttl""#));
}

#[test]
fn an_empty_value_keeps_its_row_with_a_placeholder() {
    let html = render(|| html! { <DescriptionList><DescriptionItem term="Renews" /></DescriptionList> });
    assert_eq!(count(&html, "<dd"), 1);
    assert!(html.contains('\u{2014}'));
}

#[test]
fn every_layout_and_option_is_distinguishable() {
    let list = |layout, ruled, mono, wide| {
        render(move || {
            html! {
                <DescriptionList {layout} {ruled}>
                    <DescriptionItem term="Path" {mono} {wide}>{ "/etc/tls" }</DescriptionItem>
                </DescriptionList>
            }
        })
    };
    let [inline, stacked] = DescriptionLayout::ALL;
    let rendered = [
        list(inline, false, false, false),
        list(stacked, false, false, false),
        list(inline, true, false, false),
        list(inline, false, true, false),
        list(stacked, false, false, true),
    ];
    for (i, a) in rendered.iter().enumerate() {
        for b in &rendered[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn column_options_only_apply_to_the_stacked_layout() {
    let list = |layout| {
        render(move || {
            html! { <DescriptionList {layout} columns={3} min="8rem"><DescriptionItem term="a" /></DescriptionList> }
        })
    };
    let stacked = list(DescriptionLayout::Stacked);
    assert!(stacked.contains(":3"));
    assert!(stacked.contains("8rem"));
    assert!(!list(DescriptionLayout::Inline).contains("style="));
}

#[test]
fn id_class_and_attrs_reach_the_root_elements() {
    let html = render(|| {
        html! {
            <DescriptionList id="facts" class="mine" attrs={attrs! { "aria-label" => "Certificate" }}>
                <DescriptionItem term="Name" id="name" class="row" attrs={attrs! { "data-row" => "1" }}>
                    { "example.org" }
                </DescriptionItem>
            </DescriptionList>
        }
    });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="facts""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"aria-label="Certificate""#));
    let item = &html[html.find("<div").unwrap()..];
    let item = &item[..item.find('>').unwrap()];
    assert!(item.contains(r#"id="name""#));
    assert!(item.contains("row"));
    assert!(item.contains(r#"data-row="1""#));
}
