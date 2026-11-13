use yewi_test::render;

use super::*;
use crate::attrs;

#[test]
fn renders_only_its_trigger_until_open() {
    let html = render(|| html! { <Popover content="Last deployed by Ada"><button>{ "Details" }</button></Popover> });
    assert!(html.contains("<button>Details</button>"));
    assert!(!html.contains("Last deployed by Ada"));
    assert!(!html.contains("popover="));
}

#[test]
fn keep_mounted_renders_the_content_while_closed() {
    let html = render(|| html! { <Popover keep_mounted=true content="Three members">{ "Trigger" }</Popover> });
    assert!(html.contains("Three members"), "{html}");
    // The browser keeps a popover it has not been asked to show hidden.
    assert!(html.contains("popover="), "{html}");
}

#[test]
fn open_and_default_open_show_the_content_for_every_trigger() {
    for trigger in PopoverTrigger::ALL {
        for controlled in [true, false] {
            let html = render(move || {
                html! {
                    <Popover content="Body" {trigger} open={controlled.then_some(true)} default_open={!controlled}>
                        { "Trigger" }
                    </Popover>
                }
            });
            assert!(html.contains("Body"), "{trigger:?} controlled={controlled}: {html}");
        }
    }
}

#[test]
fn head_and_foot_are_rendered_around_the_body_in_order() {
    let html = render(|| {
        html! { <Popover open=true header="Passing" content="Latency 42 ms" footer="12:00 UTC">{ "Probe" }</Popover> }
    });
    let at = |text: &str| html.find(text).unwrap_or_else(|| panic!("{text} missing: {html}"));
    assert!(at("Passing") < at("Latency 42 ms"));
    assert!(at("Latency 42 ms") < at("12:00 UTC"));
}

#[test]
fn nothing_shows_a_closed_or_disabled_popover() {
    let cases = [
        render(|| html! { <Popover content="Hidden" open=false default_open=true>{ "Closed" }</Popover> }),
        render(|| html! { <Popover content="Hidden" open=true disabled=true>{ "Disabled" }</Popover> }),
    ];
    for html in cases {
        assert!(!html.contains("Hidden"), "{html}");
    }
}

#[test]
fn hover_content_describes_and_other_triggers_group() {
    let role = |trigger| {
        render(move || html! { <Popover content="Body" {trigger} open=true label="Details">{ "x" }</Popover> })
    };
    assert!(role(PopoverTrigger::Hover).contains(r#"role="tooltip""#));
    for trigger in [PopoverTrigger::Click, PopoverTrigger::Focus, PopoverTrigger::Manual] {
        let html = role(trigger);
        assert!(html.contains(r#"role="group""#), "{trigger:?}");
        assert!(html.contains(r#"aria-label="Details""#), "{trigger:?}");
    }

    let custom = render(|| html! { <Popover content="Body" role="dialog" open=true>{ "x" }</Popover> });
    assert!(custom.contains(r#"role="dialog""#));
}

#[test]
fn focusable_makes_the_wrapper_a_button_only_for_click() {
    let wrapper = |trigger, focusable| {
        let html = render(move || html! { <Popover content="Body" {trigger} {focusable}>{ "Text" }</Popover> });
        html[..html.find('>').unwrap()].to_string()
    };
    assert!(!wrapper(PopoverTrigger::Click, false).contains("tabindex"));
    let click = wrapper(PopoverTrigger::Click, true);
    assert!(click.contains(r#"tabindex="0""#) && click.contains(r#"role="button""#));
    let hover = wrapper(PopoverTrigger::Hover, true);
    assert!(hover.contains(r#"tabindex="0""#) && !hover.contains("role="));
}

#[test]
fn arrow_and_flush_change_the_surface() {
    let rendered: Vec<String> = [(true, false), (false, false), (true, true), (false, true)]
        .into_iter()
        .map(|(arrow, flush)| {
            render(move || html! { <Popover content="Body" open=true {arrow} {flush}>{ "x" }</Popover> })
        })
        .collect();
    for (i, a) in rendered.iter().enumerate() {
        for b in &rendered[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn id_class_and_attrs_reach_the_root_and_surface_class_the_surface() {
    let html = render(|| {
        html! {
            <Popover content="Body" open=true id="probe" class="mine" surface_class="wide" attrs={attrs! { "data-row" => "7" }}>
                { "Text" }
            </Popover>
        }
    });
    let (root, rest) = html.split_at(html.find('>').unwrap());
    assert!(root.contains(r#"id="probe""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"data-row="7""#));
    assert!(!root.contains("wide") && rest.contains("wide"));
}

#[test]
fn without_a_client_the_surface_is_a_native_popover_already_in_the_page() {
    let cases = [(PopoverTrigger::Click, "click", r#"popover="auto""#), (PopoverTrigger::Hover, "hover", r#"popover="manual""#)];
    for (trigger, kind, popover) in cases {
        let html = render(move || {
            html! {
                <crate::ServerOnly script="/yewi.js">
                    <Popover content="Body" {trigger} placement={Placement::BottomEnd}>{ "Text" }</Popover>
                </crate::ServerOnly>
            }
        });
        assert!(html.contains(&format!(r#"data-yi-popover="{kind}""#)), "{html}");
        assert!(html.contains(popover), "{html}");
        assert!(html.contains("Body"));
        assert!(html.contains(r#"data-yi-side="bottom""#) && html.contains(r#"data-yi-align="end""#));
    }
}

#[test]
fn without_a_client_a_disabled_popover_has_no_surface_and_one_wasm_drives_is_left_alone() {
    let disabled = render(|| {
        html! { <crate::ServerOnly script="/yewi.js"><Popover content="Body" disabled=true>{ "x" }</Popover></crate::ServerOnly> }
    });
    assert!(!disabled.contains("Body"));
    let manual = render(|| {
        html! {
            <crate::ServerOnly script="/yewi.js">
                <Popover content="Body" trigger={PopoverTrigger::Manual}>{ "x" }</Popover>
            </crate::ServerOnly>
        }
    });
    assert!(!manual.contains("data-yi-popover") && !manual.contains("Body"));
}
