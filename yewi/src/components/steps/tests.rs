use yewi_test::render;

use super::*;
use crate::attrs;

#[test]
fn renders_an_ordered_list_with_its_label_and_titles() {
    let html = render(|| {
        html! {
            <Steps label="Checkout progress">
                <Step title="Cart" />
                <Step title="Payment" />
            </Steps>
        }
    });
    assert!(html.contains("<ol"));
    assert!(html.contains(r#"aria-label="Checkout progress""#));
    assert_eq!(html.matches("<li").count(), 2);
    assert!(html.find("Cart").unwrap() < html.find("Payment").unwrap());
}

#[test]
fn only_the_current_step_is_marked_current() {
    for status in StepStatus::ALL {
        let html = render(move || html! { <Step title="Review" {status} /> });
        assert_eq!(html.contains(r#"aria-current="step""#), status == StepStatus::Current, "{status:?}");
    }
}

#[test]
fn every_status_is_distinguishable() {
    let rendered: Vec<String> =
        StepStatus::ALL.into_iter().map(|status| render(move || html! { <Step title="Build" {status} /> })).collect();
    for (i, a) in rendered.iter().enumerate() {
        for b in &rendered[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn an_outcome_is_announced_in_words() {
    for (status, words) in [
        (StepStatus::Running, "in progress"),
        (StepStatus::Done, "completed"),
        (StepStatus::Failed, "failed"),
        (StepStatus::Blocked, "blocked"),
        (StepStatus::Skipped, "skipped"),
    ] {
        let html = render(move || html! { <Step title="Build" {status} /> });
        assert!(html.contains(words), "{status:?}");
    }
}

#[test]
fn the_announcement_can_be_replaced() {
    let html = render(|| html! { <Step title="Build" status={StepStatus::Done} status_label="terminé" /> });
    assert!(html.contains("terminé"));
    assert!(!html.contains("completed"));
}

#[test]
fn description_meta_and_detail_are_rendered() {
    let html = render(|| {
        html! {
            <Step title="Reload" description="Restarts the service" meta={html! { "1.1s" }}>
                <pre>{ "exit status 1" }</pre>
            </Step>
        }
    });
    for text in ["Reload", "Restarts the service", "1.1s", "exit status 1"] {
        assert!(html.contains(text), "{text}");
    }
}

#[test]
fn a_step_is_plain_text_unless_it_is_given_somewhere_to_go() {
    let plain = render(|| html! { <Step title="Cart" /> });
    assert!(!plain.contains("<a") && !plain.contains("<button"));

    let link = render(|| html! { <Step title="Cart" href="/cart" /> });
    assert!(link.contains(r#"href="/cart""#));

    let button = render(|| html! { <Step title="Cart" on_click={Callback::noop()} /> });
    assert!(button.contains(r#"<button type="button""#));
}

#[test]
fn id_class_and_attrs_reach_the_root_elements() {
    let html = render(|| {
        html! {
            <Steps id="progress" class="mine" attrs={attrs! { "data-flow" => "checkout" }}>
                <Step id="first" class="yours" title="Cart" attrs={attrs! { "data-step" => "cart" }} />
            </Steps>
        }
    });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="progress""#) && root.contains("mine") && root.contains(r#"data-flow="checkout""#));
    let item = &html[html.find("<li").unwrap()..];
    let item = &item[..item.find('>').unwrap()];
    assert!(item.contains(r#"id="first""#) && item.contains("yours") && item.contains(r#"data-step="cart""#));
}

#[test]
fn only_a_running_step_shows_the_spinner() {
    let running = render(|| html! { <Steps><Step title="Deploy" status={StepStatus::Running} /></Steps> });
    assert!(running.contains("yi-spinner"), "{running}");

    let done = render(|| html! { <Steps><Step title="Deploy" status={StepStatus::Done} /></Steps> });
    assert!(!done.contains("yi-spinner"), "{done}");
}
