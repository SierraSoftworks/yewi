use yew::prelude::*;
use yewi_test::render;

use super::*;
use crate::attrs;

fn root(html: &str) -> &str {
    &html[..html.find('>').unwrap()]
}

#[test]
fn a_closed_drawer_renders_no_content() {
    let html = render(|| html! { <Drawer title="New pixel">{ "Body text" }</Drawer> });
    assert!(html.starts_with("<dialog"));
    assert!(!html.contains("New pixel"));
    assert!(!html.contains("Body text"));
}

#[test]
fn an_open_drawer_is_named_by_its_title() {
    let html = render(|| {
        html! {
            <Drawer open=true title="New pixel" footer={html! { <button>{ "Create" }</button> }}>
                { "Body text" }
            </Drawer>
        }
    });
    let labelled_by = root(&html).split("aria-labelledby=\"").nth(1).unwrap().split('"').next().unwrap();
    assert!(html.contains(&format!(r#"<h2 id="{labelled_by}""#)));
    assert!(html.contains("New pixel</h2>"));
    assert!(html.contains("Body text"));
    assert!(html.contains("Create"));
    assert!(html.contains(r#"aria-label="Close""#));
    assert!(!root(&html).contains(" open"));
}

#[test]
fn without_a_title_the_label_names_it() {
    let html = render(|| html! { <Drawer open=true label="Navigation">{ "Links" }</Drawer> });
    assert!(root(&html).contains(r#"aria-label="Navigation""#));
    assert!(!html.contains("<h2"));
    assert!(html.contains("Links"));
}

#[test]
fn sides_and_sizes_are_distinguishable() {
    let sides: Vec<String> =
        DrawerSide::ALL.into_iter().map(|side| render(move || html! { <Drawer {side} title="Side" /> })).collect();
    assert_ne!(sides[0], sides[1]);

    let sizes: Vec<String> = [Size::Small, Size::Medium, Size::Large]
        .into_iter()
        .map(|size| render(move || html! { <Drawer {size} title="Sized" /> }))
        .collect();
    assert_ne!(sizes[0], sizes[1]);
    assert_ne!(sizes[1], sizes[2]);
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html =
        render(|| html! { <Drawer id="nav" class="mine" attrs={attrs! { "data-kind" => "nav" }} label="Navigation" /> });
    let root = root(&html);
    assert!(root.contains(r#"id="nav""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"data-kind="nav""#));
}
