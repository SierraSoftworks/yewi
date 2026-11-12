use yewi_test::render;

use super::*;
use crate::attrs;

#[test]
fn renders_its_stages_in_order_as_a_labelled_list() {
    let html = render(|| {
        html! {
            <Pipeline label="Release stages">
                <PipelineStage title="Build" status="done" meta={html! { "3 jobs" }}>{ "artefacts" }</PipelineStage>
                <PipelineStage title="Test" />
            </Pipeline>
        }
    });
    assert!(html.contains("<ol"));
    assert!(html.contains(r#"aria-label="Release stages""#));
    assert_eq!(html.matches("<li").count(), 2);
    assert!(html.find("Build").unwrap() < html.find("Test").unwrap());
    for text in ["done", "3 jobs", "artefacts"] {
        assert!(html.contains(text), "{text}");
    }
}

#[test]
fn only_the_current_stage_is_marked_current() {
    let current = render(|| html! { <PipelineStage title="Test" current=true /> });
    assert!(current.contains(r#"aria-current="step""#));
    let other = render(|| html! { <PipelineStage title="Test" /> });
    assert!(!other.contains("aria-current"));
}

#[test]
fn a_gate_is_rendered_in_front_of_its_stage_and_named_as_a_gate() {
    let html = render(|| {
        html! {
            <PipelineStage
                title="Production"
                gate={html! { <PipelineGate label="needs approval" state={GateState::Closed} /> }}
            />
        }
    });
    let gate = html.find("needs approval").unwrap();
    assert!(html[..gate].contains("Gate: "));
    assert!(gate < html.find("Production").unwrap());
}

#[test]
fn gate_states_and_tones_are_distinguishable() {
    let mut rendered: Vec<String> = GateState::ALL
        .into_iter()
        .map(|state| render(move || html! { <PipelineGate label="gate" {state} /> }))
        .collect();
    rendered.push(render(|| html! { <PipelineGate label="gate" state={GateState::Closed} tone={Tone::Warn} /> }));
    for (i, a) in rendered.iter().enumerate() {
        for b in &rendered[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn a_unit_always_has_an_accessible_name() {
    let plain = render(|| html! { <PipelineUnit label="web-01: healthy" /> });
    assert!(plain.contains(r#"role="img""#));
    assert!(plain.contains(r#"aria-label="web-01: healthy""#));

    let link = render(|| html! { <PipelineUnit label="web-01: healthy" href="/hosts/web-01" /> });
    assert!(link.starts_with("<a"));
    assert!(link.contains(r#"href="/hosts/web-01""#) && link.contains(r#"aria-label="web-01: healthy""#));

    let button = render(|| html! { <PipelineUnit label="web-01: healthy" on_click={Callback::noop()} /> });
    assert!(button.starts_with("<button"));
    assert!(button.contains(r#"aria-label="web-01: healthy""#));
}

#[test]
fn unit_tones_and_fills_are_distinguishable() {
    let mut rendered = Vec::new();
    for tone in Tone::ALL {
        for fill in UnitFill::ALL {
            rendered.push(render(move || html! { <PipelineUnit label="unit" {tone} {fill} /> }));
        }
    }
    for (i, a) in rendered.iter().enumerate() {
        for b in &rendered[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn units_are_a_labelled_group() {
    let html = render(|| {
        html! {
            <PipelineUnits label="Hosts in canary" per_row=4>
                <PipelineUnit label="a" />
            </PipelineUnits>
        }
    });
    assert!(html.contains(r#"role="group""#));
    assert!(html.contains(r#"aria-label="Hosts in canary""#));
}

#[test]
fn id_class_and_attrs_reach_the_root_elements() {
    let html = render(|| {
        html! {
            <Pipeline id="rollout" class="mine" attrs={attrs! { "data-rollout" => "42" }}>
                <PipelineStage id="canary" class="yours" title="canary" attrs={attrs! { "data-stage" => "canary" }} />
            </Pipeline>
        }
    });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="rollout""#) && root.contains("mine") && root.contains(r#"data-rollout="42""#));
    let item = &html[html.find("<li").unwrap()..];
    let item = &item[..item.find('>').unwrap()];
    assert!(item.contains(r#"id="canary""#) && item.contains("yours") && item.contains(r#"data-stage="canary""#));
}
