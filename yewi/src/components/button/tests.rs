use yewi_test::render;

use super::*;
use crate::{attrs, icons, Orientation};

#[test]
fn renders_a_native_button_with_its_label() {
    let html = render(|| html! { <Button>{ "Save" }</Button> });
    assert!(html.starts_with("<button"));
    assert!(html.contains(r#"type="button""#));
    assert!(html.contains("Save"));
}

#[test]
fn href_renders_a_link() {
    let html = render(|| html! { <Button href="/docs">{ "Docs" }</Button> });
    assert!(html.starts_with("<a"));
    assert!(html.contains(r#"href="/docs""#));
}

#[test]
fn every_variant_is_distinguishable() {
    let rendered: Vec<String> = ButtonVariant::ALL
        .into_iter()
        .map(|variant| render(move || html! { <Button {variant}>{ "Go" }</Button> }))
        .collect();
    for (i, a) in rendered.iter().enumerate() {
        for b in &rendered[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn disabled_uses_the_native_attribute() {
    let html = render(|| html! { <Button disabled=true>{ "Save" }</Button> });
    assert!(html.contains(" disabled"));
}

#[test]
fn loading_is_announced_and_stays_focusable() {
    let html = render(|| html! { <Button loading=true>{ "Save" }</Button> });
    assert!(html.contains(r#"aria-busy="true""#));
    assert!(html.contains(r#"aria-disabled="true""#));
    assert!(!html.contains(" disabled"));
}

#[test]
fn an_inert_link_cannot_be_followed() {
    let html = render(|| html! { <Button href="/docs" disabled=true>{ "Docs" }</Button> });
    assert!(!html.contains("href="));
    assert!(html.contains(r#"aria-disabled="true""#));
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html = render(|| {
        html! {
            <Button id="save" class="mine" attrs={attrs! { "aria-describedby" => "hint" }} icon={icons::CHECK}>
                { "Save" }
            </Button>
        }
    });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="save""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"aria-describedby="hint""#));
}

#[test]
fn pressed_reports_toggle_state_on_buttons_only() {
    for (pressed, expected) in [(true, r#"aria-pressed="true""#), (false, r#"aria-pressed="false""#)] {
        let html = render(move || html! { <Button {pressed}>{ "Watch" }</Button> });
        assert!(html.contains(expected), "{html}");
    }
    let plain = render(|| html! { <Button>{ "Watch" }</Button> });
    assert!(!plain.contains("aria-pressed"));
    let link = render(|| html! { <Button href="/docs" pressed=true>{ "Docs" }</Button> });
    assert!(!link.contains("aria-pressed"));
}

#[test]
fn icon_button_is_named_by_its_label() {
    let html = render(|| html! { <IconButton icon={icons::TRASH_2} label="Delete" /> });
    assert!(html.starts_with("<button"));
    assert!(html.contains(r#"aria-label="Delete""#));
    assert!(html.contains("<svg"));
}

#[test]
fn icon_button_forwards_button_behaviour() {
    let disabled = render(|| html! { <IconButton icon={icons::COPY} label="Copy" disabled=true /> });
    assert!(disabled.contains(" disabled"));

    let loading = render(|| html! { <IconButton icon={icons::COPY} label="Copy" loading=true /> });
    assert!(loading.contains(r#"aria-busy="true""#));

    let pressed = render(|| html! { <IconButton icon={icons::PIN} label="Pin" pressed=true /> });
    assert!(pressed.contains(r#"aria-pressed="true""#));

    let link = render(|| html! { <IconButton icon={icons::EXTERNAL_LINK} label="Source" href="/src" /> });
    assert!(link.starts_with("<a"));
    assert!(link.contains(r#"href="/src""#));
}

#[test]
fn icon_button_attrs_override_the_default_names() {
    let html = render(|| {
        html! {
            <IconButton id="pin" class="mine" icon={icons::PIN} label="Pin" attrs={attrs! { "title" => "Pin (P)" }} />
        }
    });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="pin""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"aria-label="Pin""#));
    assert!(root.contains(r#"title="Pin (P)""#));
    assert!(!root.contains(r#"title="Pin""#));
}

#[test]
fn group_is_a_labelled_group_containing_its_buttons() {
    let html = render(|| {
        html! {
            <ButtonGroup id="actions" class="mine" label="Connection actions" attrs={attrs! { "data-row" => "7" }}>
                <Button>{ "Edit" }</Button>
                <Button>{ "Unlink" }</Button>
            </ButtonGroup>
        }
    });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"role="group""#));
    assert!(root.contains(r#"aria-label="Connection actions""#));
    assert!(root.contains(r#"id="actions""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"data-row="7""#));
    assert_eq!(html.matches("<button").count(), 2);
}

#[test]
fn vertical_group_differs_from_horizontal() {
    let render_group = |orientation: Orientation| {
        render(move || html! { <ButtonGroup {orientation}><Button>{ "A" }</Button></ButtonGroup> })
    };
    assert_ne!(render_group(Orientation::Vertical), render_group(Orientation::Horizontal));
}
