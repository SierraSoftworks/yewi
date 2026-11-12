use yewi_test::render;

use super::*;
use crate::attrs;

/// The rendered markup with tags removed, so only what is read out remains.
fn text(html: &str) -> String {
    let mut out = String::new();
    let mut in_tag = false;
    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            c if !in_tag => out.push(c),
            _ => {}
        }
    }
    out
}

#[test]
fn counts_are_capped_at_max() {
    let cases: [(u32, Option<u32>, &str); 5] =
        [(1, None, "1"), (99, None, "99"), (100, None, "99+"), (12, Some(9), "9+"), (9, Some(9), "9")];
    for (count, max, expected) in cases {
        let html = render(move || match max {
            Some(max) => html! { <Badge {count} {max} /> },
            None => html! { <Badge {count} /> },
        });
        assert_eq!(text(&html), expected, "count {count}, max {max:?}");
    }
}

#[test]
fn zero_is_hidden_unless_asked_for() {
    assert_eq!(render(|| html! { <Badge count=0 /> }), "");
    assert_eq!(text(&render(|| html! { <Badge count=0 show_zero=true /> })), "0");
}

#[test]
fn a_hidden_badge_leaves_its_children_in_place() {
    let html = render(|| html! { <Badge count=0><button>{ "Inbox" }</button></Badge> });
    assert_eq!(text(&html), "Inbox");
}

#[test]
fn an_attached_badge_follows_its_children() {
    let html = render(|| html! { <Badge count=3><button>{ "Inbox" }</button></Badge> });
    assert!(html.find("Inbox").unwrap() < html.find('3').unwrap());
}

#[test]
fn a_label_replaces_the_number_for_assistive_technology() {
    let html = render(|| html! { <Badge count=3 label="3 unread notifications" /> });
    assert!(html.contains(r#"<span aria-hidden="true">3</span>"#));
    assert!(html.contains("3 unread notifications"));
}

#[test]
fn a_dot_is_decorative_until_labelled() {
    let bare = render(|| html! { <Badge /> });
    assert!(bare.contains(r#"aria-hidden="true""#));
    assert_eq!(text(&bare), "");

    let labelled = render(|| html! { <Badge label="Update available" /> });
    assert!(!labelled.contains("aria-hidden"));
    assert_eq!(text(&labelled), "Update available");
}

#[test]
fn tones_and_emphasis_are_distinguishable() {
    let mut rendered: Vec<String> =
        Tone::ALL.into_iter().map(|tone| render(move || html! { <Badge count=1 {tone} /> })).collect();
    rendered.push(render(|| html! { <Badge count=1 bold=true /> }));
    for (i, a) in rendered.iter().enumerate() {
        for b in &rendered[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    for anchored in [false, true] {
        let html = render(move || {
            let attrs = attrs! { "data-area" => "inbox" };
            if anchored {
                html! { <Badge count=2 id="unread" class="mine" {attrs}><button>{ "Inbox" }</button></Badge> }
            } else {
                html! { <Badge count=2 id="unread" class="mine" {attrs} /> }
            }
        });
        let root = &html[..html.find('>').unwrap()];
        assert!(root.contains(r#"id="unread""#), "anchored: {anchored}");
        assert!(root.contains("mine"));
        assert!(root.contains(r#"data-area="inbox""#));
        assert_eq!(html.matches(r#"id="unread""#).count(), 1);
    }
}
