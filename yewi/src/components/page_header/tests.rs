use yew::prelude::*;
use yewi_test::render;

use super::*;
use crate::attrs;

#[test]
fn the_title_is_the_only_h1() {
    let html = render(|| html! { <PageHeader title="Rollouts" /> });
    assert_eq!(html.matches("<h1").count(), 1);
    assert!(html.contains("Rollouts"));
}

#[test]
fn every_slot_is_rendered_when_given() {
    let html = render(|| {
        html! {
            <PageHeader
                title="Rollouts"
                subtitle={html! { "Subtitle text" }}
                breadcrumbs={html! { <a href="/">{ "Back home" }</a> }}
                meta={html! { <span>{ "Meta text" }</span> }}
                actions={html! { <button>{ "New rollout" }</button> }}
            >
                { "Tabs here" }
            </PageHeader>
        }
    });
    for text in ["Subtitle text", "Back home", "Meta text", "New rollout", "Tabs here"] {
        assert!(html.contains(text), "missing {text}");
    }
}

#[test]
fn empty_slots_leave_no_wrappers_behind() {
    let bare = render(|| html! { <PageHeader title="Rollouts" /> });
    let full = render(|| html! { <PageHeader title="Rollouts" actions={html! { <button>{ "New" }</button> }} /> });
    assert!(bare.matches("<div").count() < full.matches("<div").count());
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html = render(|| {
        html! { <PageHeader title="Rollouts" id="head" class="mine" attrs={attrs! { "data-page" => "rollouts" }} /> }
    });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.starts_with("<header"));
    assert!(root.contains(r#"id="head""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"data-page="rollouts""#));
}
