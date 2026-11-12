use yew::prelude::*;
use yewi_test::render;

use super::*;
use crate::{Size, Tone, attrs, icons};

#[test]
fn shows_its_title_copy_and_actions() {
    let html = render(|| {
        html! {
            <EmptyState title="No hosts yet" actions={html! { <a href="/hosts/new">{ "Add a host" }</a> }}>
                { "Hosts appear once an agent connects." }
            </EmptyState>
        }
    });
    assert!(html.contains("No hosts yet"));
    assert!(html.contains("Hosts appear once an agent connects."));
    assert!(html.contains("Add a host"));
}

#[test]
fn an_empty_state_is_not_announced_as_an_alert() {
    let html = render(|| html! { <EmptyState title="No results" icon={icons::SEARCH_X} /> });
    assert!(!html.contains("role=\"alert\""));
    assert!(html.contains(r#"aria-hidden="true""#), "the icon is decorative");
}

#[test]
fn optional_parts_are_omitted_when_unset() {
    let bare = render(|| html! { <EmptyState title="No results" /> });
    let full = render(|| {
        html! {
            <EmptyState title="No results" icon={icons::SEARCH_X} actions={html! { <button>{ "Clear" }</button> }}>
                { "Try another search." }
            </EmptyState>
        }
    });
    assert!(!bare.contains("<svg"));
    assert_eq!(bare.matches("<div").count(), 1);
    assert_eq!(full.matches("<div").count(), 3);
}

#[test]
fn alignments_sizes_and_tones_are_distinguishable() {
    let mut rendered = vec![
        render(|| html! { <EmptyState title="t" /> }),
        render(|| html! { <EmptyState title="t" align={EmptyStateAlign::Start} /> }),
        render(|| html! { <EmptyState title="t" size={Size::Small} /> }),
        render(|| html! { <EmptyState title="t" size={Size::Large} /> }),
    ];
    rendered.extend(Tone::ALL.into_iter().skip(1).map(|tone| render(move || html! { <EmptyState title="t" {tone} /> })));
    for (i, a) in rendered.iter().enumerate() {
        for b in &rendered[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn an_error_state_is_an_alert_with_a_default_title() {
    let html = render(|| html! { <ErrorState>{ "The server could not be reached." }</ErrorState> });
    assert!(html.contains(r#"role="alert""#));
    assert!(html.contains("Something went wrong"));
    assert!(html.contains("The server could not be reached."));
}

#[test]
fn retry_is_offered_only_with_a_handler() {
    let without = render(|| html! { <ErrorState title="Not permitted" /> });
    let with = render(|| html! { <ErrorState on_retry={Callback::noop()} /> });
    let custom = render(|| html! { <ErrorState on_retry={Callback::noop()} retry_label="Reload" /> });
    assert!(!without.contains("<button"));
    assert!(with.contains("<button") && with.contains("Try again"));
    assert!(custom.contains("Reload"));
}

#[test]
fn a_retry_in_flight_is_announced_as_busy() {
    let html = render(|| html! { <ErrorState on_retry={Callback::noop()} retrying=true /> });
    assert!(html.contains(r#"aria-busy="true""#));
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let roots = [
        render(|| html! { <EmptyState title="t" id="x" class="mine" attrs={attrs! { "data-test" => "1" }} /> }),
        render(|| html! { <ErrorState id="x" class="mine" attrs={attrs! { "data-test" => "1" }} /> }),
    ];
    for html in roots {
        let root = &html[..html.find('>').unwrap()];
        assert!(root.contains(r#"id="x""#));
        assert!(root.contains("mine"));
        assert!(root.contains(r#"data-test="1""#));
    }
}
