use yew::prelude::*;
use yewi_test::render;

use super::*;
use crate::attrs;

#[test]
fn renders_only_its_child_until_shown() {
    let html = render(|| html! { <Tooltip tip="Copy the address"><button>{ "Copy" }</button></Tooltip> });
    assert!(html.contains("<button>Copy</button>"));
    assert!(!html.contains("Copy the address"));
    assert!(!html.contains(r#"role="tooltip""#));
}

#[test]
fn open_shows_the_tip_as_a_tooltip() {
    let html = render(|| html! { <Tooltip tip="Copy the address" open=true><button>{ "Copy" }</button></Tooltip> });
    assert!(html.contains(r#"role="tooltip""#));
    assert!(html.contains("Copy the address"));
}

#[test]
fn rich_content_is_rendered_inside_the_tooltip() {
    let html = render(|| {
        let tip = html! { <><strong>{ "Blocked" }</strong>{ " by a failed check" }</> };
        html! { <Tooltip {tip} open=true>{ "Status" }</Tooltip> }
    });
    assert!(html.contains("<strong>Blocked</strong> by a failed check"));
}

#[test]
fn nothing_can_force_an_unavailable_tooltip_open() {
    let cases = [
        render(|| html! { <Tooltip open=true>{ "No tip" }</Tooltip> }),
        render(|| html! { <Tooltip tip="Hidden" open=true disabled=true>{ "Disabled" }</Tooltip> }),
        render(|| html! { <Tooltip tip="Hidden" open=false>{ "Closed" }</Tooltip> }),
    ];
    for html in cases {
        assert!(!html.contains(r#"role="tooltip""#), "{html}");
    }
}

#[test]
fn every_variant_is_distinguishable() {
    let rendered: Vec<String> = TooltipVariant::ALL
        .into_iter()
        .flat_map(|variant| [false, true].map(|arrow| (variant, arrow)))
        .map(|(variant, arrow)| render(move || html! { <Tooltip tip="Tip" open=true {variant} {arrow}>{ "x" }</Tooltip> }))
        .collect();
    for (i, a) in rendered.iter().enumerate() {
        for b in &rendered[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn focusable_adds_a_tab_stop_only_when_asked() {
    let plain = render(|| html! { <Tooltip tip="Tip">{ "Text" }</Tooltip> });
    let focusable = render(|| html! { <Tooltip tip="Tip" focusable=true>{ "Text" }</Tooltip> });
    assert!(!plain.contains("tabindex"));
    assert!(focusable.contains(r#"tabindex="0""#));
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html = render(|| {
        html! { <Tooltip tip="Tip" id="hint" class="mine" attrs={attrs! { "data-row" => "7" }}>{ "Text" }</Tooltip> }
    });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="hint""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"data-row="7""#));
}
