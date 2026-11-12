use yew::prelude::*;
use yewi_test::render;

use super::*;
use crate::attrs;

fn entry(id: u64) -> Entry {
    Entry { id: ToastId(id), message: Rc::new(ToastMessage::new(Tone::Ok, format!("toast {id}"))) }
}

fn stack_after(actions: impl IntoIterator<Item = StackAction>) -> Vec<u64> {
    let stack = actions.into_iter().fold(Rc::new(Stack::default()), Stack::reduce);
    stack.list.iter().map(|entry| entry.id.0).collect()
}

fn push(id: u64, max: usize) -> StackAction {
    StackAction::Push { entry: entry(id), max }
}

#[test]
fn danger_is_an_alert_and_every_other_tone_a_status() {
    for tone in Tone::ALL {
        let html = render(move || html! { <Toast {tone}>{ "Saved" }</Toast> });
        let expected = if tone == Tone::Danger { r#"role="alert""# } else { r#"role="status""# };
        assert!(html.contains(expected), "{tone:?}: {html}");
        assert!(html.contains("Saved"));
    }
}

#[test]
fn every_tone_and_accent_is_distinguishable() {
    let rendered: Vec<String> = Tone::ALL
        .into_iter()
        .flat_map(|tone| [ToastAccent::Icon, ToastAccent::Stripe].map(|accent| (tone, accent)))
        .map(|(tone, accent)| render(move || html! { <Toast {tone} {accent}>{ "Saved" }</Toast> }))
        .collect();
    for (i, a) in rendered.iter().enumerate() {
        for b in &rendered[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn tone_icons_are_decorative() {
    let html = render(|| html! { <Toast tone={Tone::Ok}>{ "Saved" }</Toast> });
    assert!(html.contains("<svg"));
    assert!(!html.contains(r#"role="img""#));
}

#[test]
fn the_dismiss_button_appears_only_when_dismissal_is_handled() {
    let fixed = render(|| html! { <Toast>{ "Saved" }</Toast> });
    assert!(!fixed.contains("<button"));

    let dismissible = render(|| html! { <Toast on_dismiss={Callback::noop()}>{ "Saved" }</Toast> });
    assert!(dismissible.contains(r#"aria-label="Dismiss""#));
}

#[test]
fn title_and_action_are_rendered() {
    let html = render(|| {
        html! {
            <Toast title="Archived" action={ToastAction::new("Undo", Callback::noop())}>{ "3 hosts" }</Toast>
        }
    });
    for text in ["Archived", "3 hosts", "Undo"] {
        assert!(html.contains(text), "missing {text}: {html}");
    }
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html = render(|| {
        html! { <Toast id="saved" class="mine" attrs={attrs! { "data-kind" => "save" }}>{ "Saved" }</Toast> }
    });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="saved""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"data-kind="save""#));
}

#[test]
fn the_host_renders_its_children_and_an_empty_labelled_live_region() {
    #[function_component]
    fn Page() -> Html {
        let _toast = use_toast();
        html! { <main>{ "The app" }</main> }
    }

    let html = render(|| html! { <ToastHost label="Updates" id="toasts"><Page /></ToastHost> });
    assert!(html.contains("The app"));
    assert!(html.contains(r#"role="region""#));
    assert!(html.contains(r#"aria-label="Updates""#));
    assert!(html.contains(r#"aria-live="polite""#));
    assert!(html.contains(r#"id="toasts""#));
    assert!(!html.contains(r#"role="status""#));
}

#[test]
fn every_placement_is_distinguishable() {
    let rendered: Vec<String> =
        ToastPlacement::ALL.into_iter().map(|placement| render(move || html! { <ToastHost {placement} /> })).collect();
    for (i, a) in rendered.iter().enumerate() {
        for b in &rendered[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn the_stack_keeps_the_newest_toasts_up_to_its_limit() {
    assert_eq!(stack_after([push(1, 4), push(2, 4)]), [1, 2]);
    assert_eq!(stack_after((1..=6).map(|id| push(id, 4))), [3, 4, 5, 6]);
    // A limit of zero would hide every toast, so it is treated as one.
    assert_eq!(stack_after([push(1, 0), push(2, 0)]), [2]);
}

#[test]
fn toasts_are_dismissed_by_id_or_all_at_once() {
    let three = || (1..=3).map(|id| push(id, 4));
    assert_eq!(stack_after(three().chain([StackAction::Dismiss(ToastId(2))])), [1, 3]);
    assert_eq!(stack_after(three().chain([StackAction::Dismiss(ToastId(9))])), [1, 2, 3]);
    assert_eq!(stack_after(three().chain([StackAction::Clear])), [] as [u64; 0]);
}

#[test]
fn failures_and_work_in_flight_stay_until_dismissed() {
    let cases = [
        (ToastDuration::Auto, Tone::Ok, Some(5_000)),
        (ToastDuration::Auto, Tone::Info, Some(5_000)),
        (ToastDuration::Auto, Tone::Warn, Some(5_000)),
        (ToastDuration::Auto, Tone::Neutral, Some(5_000)),
        (ToastDuration::Auto, Tone::Danger, None),
        (ToastDuration::Auto, Tone::Active, None),
        (ToastDuration::Millis(800), Tone::Danger, Some(800)),
        (ToastDuration::Sticky, Tone::Ok, None),
    ];
    for (duration, tone, expected) in cases {
        assert_eq!(duration.resolve(tone, 5_000), expected, "{duration:?} {tone:?}");
    }
}
