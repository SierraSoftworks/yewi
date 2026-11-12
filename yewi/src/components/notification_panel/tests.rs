use yewi_test::render;

use super::*;
use crate::attrs;

fn item() -> Html {
    html! { <NotificationItem title="web-03" tone={Tone::Danger}>{ "Agent offline" }</NotificationItem> }
}

#[test]
fn a_closed_panel_renders_the_bell_but_not_the_list() {
    let html = render(|| html! { <NotificationPanel count=1>{ item() }</NotificationPanel> });
    assert!(html.contains(r#"aria-label="Notifications""#));
    assert!(!html.contains("web-03"));
    assert!(!html.contains("popover="));
}

#[test]
fn an_open_panel_lists_its_items_under_its_title() {
    let html = render(|| {
        html! {
            <NotificationPanel title="Needs attention" description="Offline agents" count=1 open=true>
                { item() }
            </NotificationPanel>
        }
    });
    assert!(html.contains(r#"aria-label="Needs attention""#));
    assert!(html.contains("Offline agents"));
    assert!(html.contains(r#"role="list""#));
    assert!(html.contains("web-03"));
    assert!(html.contains("Agent offline"));
}

#[test]
fn the_empty_message_shows_only_without_items() {
    let cases = [
        (render(|| html! { <NotificationPanel open=true empty="All clear" /> }), true),
        // An empty iterator is still no items.
        (
            render(|| {
                let none: Vec<Html> = Vec::new();
                html! { <NotificationPanel open=true empty="All clear">{ for none }</NotificationPanel> }
            }),
            true,
        ),
        (render(|| html! { <NotificationPanel open=true empty="All clear">{ item() }</NotificationPanel> }), false),
    ];
    for (html, empty) in cases {
        assert_eq!(html.contains("All clear"), empty, "{html}");
        assert_eq!(html.contains(r#"role="list""#), !empty, "{html}");
    }
}

#[test]
fn the_count_is_capped_hidden_at_zero_and_left_to_the_label() {
    let cases: [(u32, Option<&str>); 4] = [(0, None), (3, Some(">3<")), (99, Some(">99<")), (100, Some(">99+<"))];
    for (count, expected) in cases {
        let html = render(move || html! { <NotificationPanel {count} label={format!("{count} unread")} /> });
        assert!(html.contains(&format!(r#"aria-label="{count} unread""#)), "{html}");
        match expected {
            Some(text) => {
                assert!(html.contains(text), "count {count}: {html}");
                // The number is decoration: the button's name carries it.
                assert!(html.contains(r#"aria-hidden="true""#));
            }
            None => assert!(!html.contains(">0<"), "{html}"),
        }
    }
}

#[test]
fn an_item_links_its_title_only_when_given_a_target() {
    let linked = render(|| html! { <NotificationItem title="web-03" href="/hosts/web-03" /> });
    assert!(linked.contains(r#"href="/hosts/web-03""#));
    let plain = render(|| html! { <NotificationItem title="web-03" /> });
    assert!(!plain.contains("<a"));
    assert!(plain.contains("web-03"));
}

#[test]
fn item_tones_are_distinguishable() {
    let rendered: Vec<String> = Tone::ALL
        .into_iter()
        .map(|tone| render(move || html! { <NotificationItem title="web-03" {tone} /> }))
        .collect();
    for (i, a) in rendered.iter().enumerate() {
        for b in &rendered[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn optional_parts_of_an_item_are_left_out_when_unset() {
    let bare = render(|| html! { <NotificationItem title="web-03" /> });
    let full = render(|| {
        html! {
            <NotificationItem title="web-03" meta="12 min">
                <NotificationReason tone={Tone::Warn}>{ "Plan is stale" }</NotificationReason>
            </NotificationItem>
        }
    });
    assert!(full.contains("12 min") && full.contains("Plan is stale"));
    assert!(bare.matches("<span").count() < full.matches("<span").count());
}

#[test]
fn id_class_and_attrs_reach_each_root_element() {
    let cases = [
        render(|| html! { <NotificationPanel id="bell" class="mine" attrs={attrs! { "data-area" => "bar" }} /> }),
        render(|| html! { <NotificationList id="bell" class="mine" attrs={attrs! { "data-area" => "bar" }} /> }),
        render(|| html! { <NotificationItem title="x" id="bell" class="mine" attrs={attrs! { "data-area" => "bar" }} /> }),
        render(|| html! { <NotificationReason id="bell" class="mine" attrs={attrs! { "data-area" => "bar" }} /> }),
    ];
    for html in cases {
        let root = &html[..html.find('>').unwrap()];
        assert!(root.contains(r#"id="bell""#), "{html}");
        assert!(root.contains("mine"));
        assert!(root.contains(r#"data-area="bar""#));
    }
}
