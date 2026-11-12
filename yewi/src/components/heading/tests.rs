use yew::prelude::*;
use yewi_test::render;

use super::*;
use crate::attrs;

#[test]
fn level_selects_the_element() {
    for (level, element) in HeadingLevel::ALL.into_iter().zip(["<h1", "<h2", "<h3", "<h4", "<h5", "<h6"]) {
        let html = render(move || html! { <Heading {level}>{ "Rollouts" }</Heading> });
        assert!(html.starts_with(element), "{level:?} rendered {html}");
        assert!(html.contains("Rollouts"));
    }
}

#[test]
fn defaults_to_a_section_heading() {
    assert!(render(|| html! { <Heading>{ "Hosts" }</Heading> }).starts_with("<h2"));
}

#[test]
fn size_changes_the_look_but_not_the_element() {
    let plain = render(|| html! { <Heading level={HeadingLevel::H2}>{ "Hosts" }</Heading> });
    let resized = render(|| html! { <Heading level={HeadingLevel::H2} size={TextSize::Xl}>{ "Hosts" }</Heading> });
    assert!(resized.starts_with("<h2"));
    assert_ne!(plain, resized);
}

#[test]
fn a_size_equal_to_the_level_default_changes_nothing() {
    for level in HeadingLevel::ALL {
        let plain = render(move || html! { <Heading {level}>{ "x" }</Heading> });
        let explicit = render(move || html! { <Heading {level} size={level.default_size()}>{ "x" }</Heading> });
        assert_eq!(plain, explicit);
    }
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html =
        render(|| html! { <Heading id="hosts" class="mine" attrs={attrs! { "data-section" => "1" }}>{ "x" }</Heading> });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="hosts""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"data-section="1""#));
}
