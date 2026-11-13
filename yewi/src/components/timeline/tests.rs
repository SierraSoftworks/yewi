use yewi_test::render;

use super::*;
use crate::attrs;

#[test]
fn renders_an_ordered_list_with_its_label_and_events_in_order() {
    let html = render(|| {
        html! {
            <Timeline label="Rollout history">
                <TimelineItem title="Deployed" />
                <TimelineItem title="Built" />
            </Timeline>
        }
    });
    assert!(html.contains("<ol"));
    assert!(html.contains(r#"aria-label="Rollout history""#));
    assert_eq!(html.matches("<li").count(), 2);
    assert!(html.find("Deployed").unwrap() < html.find("Built").unwrap());
}

#[test]
fn title_meta_aside_and_detail_are_rendered() {
    let html = render(|| {
        html! {
            <TimelineItem title="Fix the proxy" meta={html! { "Ada, 2 hours ago" }} aside={html! { "4f2a9c1" }}>
                <p>{ "Reverted the upstream change." }</p>
            </TimelineItem>
        }
    });
    for text in ["Fix the proxy", "Ada, 2 hours ago", "4f2a9c1", "Reverted the upstream change."] {
        assert!(html.contains(text), "{text}");
    }
}

#[test]
fn every_tone_marker_and_option_is_distinguishable() {
    let mut rendered: Vec<String> = vec![render(|| html! { <TimelineItem title="Event" /> })];
    rendered.extend(Tone::ALL.map(|tone| render(move || html! { <TimelineItem title="Event" {tone} /> })));
    rendered.extend(
        TimelineMarker::ALL[1..].iter().map(|&marker| render(move || html! { <TimelineItem title="Event" {marker} /> })),
    );
    rendered.push(render(|| html! { <TimelineItem title="Event" active=true /> }));
    rendered.push(render(|| html! { <TimelineItem title="Event" icon={crate::icons::CHECK} /> }));
    for (i, a) in rendered.iter().enumerate() {
        for b in &rendered[i + 1..] {
            assert_ne!(a, b);
        }
    }

    let lists: Vec<String> = TimelineRunOff::ALL
        .into_iter()
        .map(|run_off| render(move || html! { <Timeline {run_off}><TimelineItem title="Event" /></Timeline> }))
        .chain([render(|| html! { <Timeline divided=true><TimelineItem title="Event" /></Timeline> })])
        .collect();
    for (i, a) in lists.iter().enumerate() {
        for b in &lists[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn a_tone_is_read_out_only_when_given_words() {
    let silent = render(|| html! { <TimelineItem title="Probe failed" tone={Tone::Danger} /> });
    assert!(!silent.contains("yi-visually-hidden"));

    let spoken = render(|| html! { <TimelineItem title="Probe" tone={Tone::Danger} tone_label="failing" /> });
    assert!(spoken.contains("failing"));
}

#[test]
fn the_marker_is_hidden_from_assistive_technology() {
    let html = render(|| html! { <TimelineItem title="Event" icon={crate::icons::CHECK} /> });
    assert!(html.contains(r#"aria-hidden="true""#));
}

#[test]
fn an_event_is_plain_text_unless_it_is_given_somewhere_to_go() {
    let plain = render(|| html! { <TimelineItem title="Deployed" /> });
    assert!(!plain.contains("<a") && !plain.contains("<button"));

    let link = render(|| html! { <TimelineItem title="Deployed" href="/rollouts/42" /> });
    assert!(link.contains(r#"href="/rollouts/42""#));

    let button = render(|| html! { <TimelineItem title="Deployed" on_click={Callback::noop()} /> });
    assert!(button.contains(r#"<button type="button""#));
}

#[test]
fn a_horizontal_timeline_keeps_each_event_reachable_and_named() {
    let html = render(|| {
        html! {
            <Timeline orientation={Orientation::Horizontal} label="Incident">
                <TimelineItem title="Investigating" tone={Tone::Danger} tone_label="outage">
                    { "Probe detail" }
                </TimelineItem>
                <TimelineItem title="Resolved" tone={Tone::Ok} />
            </Timeline>
        }
    });
    assert_eq!(html.matches("<li").count(), 2);
    assert_eq!(html.matches(r#"tabindex="0""#).count(), 2);
    assert!(html.contains("Investigating") && html.contains("outage") && html.contains("Resolved"));
    // The detail belongs to the popover, which exists only while open.
    assert!(!html.contains("Probe detail"));
}

#[test]
fn id_class_and_attrs_reach_the_root_elements() {
    let html = render(|| {
        html! {
            <Timeline id="history" class="mine" attrs={attrs! { "data-kind" => "log" }}>
                <TimelineItem id="first" class="yours" title="Deployed" attrs={attrs! { "data-event" => "42" }} />
            </Timeline>
        }
    });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="history""#) && root.contains("mine") && root.contains(r#"data-kind="log""#));
    let item = &html[html.find("<li").unwrap()..];
    let item = &item[..item.find('>').unwrap()];
    assert!(item.contains(r#"id="first""#) && item.contains("yours") && item.contains(r#"data-event="42""#));
}
