use yewi_test::render;

use super::*;
use crate::attrs;

#[test]
fn renders_a_single_key() {
    let html = render(|| html! { <Kbd>{ "Esc" }</Kbd> });
    assert!(html.starts_with("<kbd"));
    assert!(html.contains("Esc"));
    assert_eq!(html.matches("<kbd").count(), 1);
}

#[test]
fn shortcuts_split_into_steps_of_keys() {
    let cases: [(&str, &[&[&str]]); 7] = [
        ("K", &[&["K"]]),
        ("Ctrl+K", &[&["Ctrl", "K"]]),
        ("Ctrl+Shift+P", &[&["Ctrl", "Shift", "P"]]),
        ("G H", &[&["G"], &["H"]]),
        ("Ctrl+K  Ctrl+S", &[&["Ctrl", "K"], &["Ctrl", "S"]]),
        ("Ctrl++", &[&["Ctrl", "+"]]),
        ("+", &[&["+"]]),
    ];
    for (keys, expected) in cases {
        let actual: Vec<Vec<&str>> = steps(keys).collect();
        assert_eq!(actual, expected, "{keys:?}");
    }
}

#[test]
fn a_shortcut_nests_one_key_cap_per_key() {
    let html = render(|| html! { <Kbd keys="Ctrl+Shift+P" /> });
    assert!(html.starts_with("<kbd"));
    assert_eq!(html.matches("<kbd").count(), 4);
    for key in ["Ctrl", "Shift", "P"] {
        assert!(html.contains(&format!(">{key}</kbd>")));
    }
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    for keys in [None, Some(AttrValue::from("Ctrl+K"))] {
        let html = render(move || {
            html! { <Kbd id="k" class="mine" keys={keys.clone()} attrs={attrs! { "aria-label" => "Search" }}>{ "K" }</Kbd> }
        });
        let root = &html[..html.find('>').unwrap()];
        assert!(root.contains(r#"id="k""#));
        assert!(root.contains("mine"));
        assert!(root.contains(r#"aria-label="Search""#));
    }
}
