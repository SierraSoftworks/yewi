use yew::prelude::*;
use yewi_test::render;

use super::*;
use crate::attrs;

fn root(html: &str) -> &str {
    &html[..html.find('>').unwrap()]
}

#[test]
fn alert_shows_its_title_message_and_actions() {
    let html = render(|| {
        html! {
            <Alert title="Sync failed" actions={html! { <button>{ "Retry" }</button> }}>
                { "The server did not respond." }
            </Alert>
        }
    });
    for text in ["Sync failed", "The server did not respond.", "Retry"] {
        assert!(html.contains(text), "missing {text:?}");
    }
}

#[test]
fn problems_interrupt_and_everything_else_is_polite() {
    for tone in Tone::ALL {
        let expected = if matches!(tone, Tone::Warn | Tone::Danger) { "alert" } else { "status" };
        let alert = render(move || html! { <Alert {tone}>{ "Message" }</Alert> });
        let banner = render(move || html! { <Banner {tone}>{ "Message" }</Banner> });
        for html in [alert, banner] {
            assert!(root(&html).contains(&format!(r#"role="{expected}""#)), "{tone:?}: {html}");
        }
    }
}

#[test]
fn every_tone_and_variant_is_distinguishable() {
    let mut rendered = Vec::new();
    for tone in Tone::ALL {
        for variant in AlertVariant::ALL {
            rendered.push(render(move || html! { <Alert {tone} {variant}>{ "Message" }</Alert> }));
        }
        for variant in BannerVariant::ALL {
            rendered.push(render(move || html! { <Banner {tone} {variant}>{ "Message" }</Banner> }));
        }
        // Neutral and Info share an icon, so compare notes without one.
        rendered.push(render(move || html! { <Note {tone}>{ "Message" }</Note> }));
    }
    for (i, a) in rendered.iter().enumerate() {
        for b in &rendered[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn icons_are_decorative_and_can_be_left_out() {
    let with = render(|| html! { <Alert>{ "Message" }</Alert> });
    assert!(with.contains("<svg"));
    assert!(with.contains(r#"aria-hidden="true""#));

    let alert = render(|| html! { <Alert hide_icon=true>{ "Message" }</Alert> });
    let banner = render(|| html! { <Banner hide_icon=true>{ "Message" }</Banner> });
    let note = render(|| html! { <Note>{ "Message" }</Note> });
    for html in [alert, banner, note] {
        assert!(!html.contains("<svg"), "{html}");
    }
}

#[test]
fn note_icon_is_opt_in() {
    let html = render(|| html! { <Note tone={Tone::Warn} with_icon=true>{ "Expires soon" }</Note> });
    assert!(html.starts_with("<p"));
    assert!(html.contains("<svg"));
    assert!(!root(&html).contains("role="));
}

#[test]
fn dismiss_button_appears_only_with_a_handler() {
    let fixed = render(|| html! { <><Alert>{ "Message" }</Alert><Banner>{ "Message" }</Banner></> });
    assert!(!fixed.contains("<button"));

    let alert = render(|| html! { <Alert on_dismiss={Callback::noop()}>{ "Message" }</Alert> });
    let banner = render(|| {
        html! { <Banner on_dismiss={Callback::noop()} dismiss_label="Hide notice">{ "Message" }</Banner> }
    });
    assert!(alert.contains(r#"aria-label="Dismiss""#));
    assert!(banner.contains(r#"aria-label="Hide notice""#));
    for html in [alert, banner] {
        assert!(html.contains(r#"type="button""#));
    }
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let alert = render(|| {
        html! { <Alert id="x" class="mine" tone={Tone::Danger} attrs={attrs! { "role" => "status" }}>{ "M" }</Alert> }
    });
    let banner = render(|| html! { <Banner id="x" class="mine" attrs={attrs! { "role" => "status" }}>{ "M" }</Banner> });
    let note = render(|| html! { <Note id="x" class="mine" attrs={attrs! { "role" => "status" }}>{ "M" }</Note> });
    for html in [alert, banner, note] {
        let root = root(&html);
        assert!(root.contains(r#"id="x""#));
        assert!(root.contains("mine"));
        assert!(root.contains(r#"role="status""#));
        assert!(!root.contains(r#"role="alert""#));
    }
}
