use yewi_test::render;

use super::*;
use crate::{attrs, icons};

#[test]
fn renders_label_value_unit_and_detail() {
    let html = render(|| html! { <StatTile label="Median time" value="184" unit="ms" detail="Last 7 days" /> });
    for text in ["Median time", "184", "ms", "Last 7 days"] {
        assert!(html.contains(text), "missing {text}");
    }
    assert!(html.starts_with("<div"));
}

#[test]
fn trend_direction_is_available_as_text() {
    for (trend, text) in [(StatTrend::Up, "Up"), (StatTrend::Down, "Down"), (StatTrend::Flat, "No change")] {
        let html = render(move || html! { <StatTile label="Visitors" value="1" delta="12%" {trend} /> });
        assert!(html.contains(text), "{trend:?} is not announced");
        assert!(html.contains("12%"));
    }
}

#[test]
fn lower_is_better_flips_good_and_bad() {
    let tile = |trend, lower_is_better| {
        render(move || html! { <StatTile label="Errors" value="1" delta="5%" {trend} {lower_is_better} /> })
    };
    assert_ne!(tile(StatTrend::Up, false), tile(StatTrend::Up, true));
    assert_ne!(tile(StatTrend::Down, false), tile(StatTrend::Down, true));
    assert_eq!(tile(StatTrend::Flat, false), tile(StatTrend::Flat, true));
}

#[test]
fn trend_of_a_change_respects_the_tolerance() {
    for (change, expected) in [
        (4.2, StatTrend::Up),
        (-4.2, StatTrend::Down),
        (0.01, StatTrend::Flat),
        (-0.01, StatTrend::Flat),
        (f64::NAN, StatTrend::Flat),
    ] {
        assert_eq!(StatTrend::of(change, 0.05), expected, "{change}");
    }
}

#[test]
fn on_click_renders_a_toggle_button() {
    let tile = |selected| {
        render(move || html! { <StatTile label="Visitors" value="1" {selected} on_click={Callback::noop()} /> })
    };
    assert!(tile(false).starts_with("<button"));
    assert!(tile(false).contains(r#"aria-pressed="false""#));
    assert!(tile(true).contains(r#"aria-pressed="true""#));
}

#[test]
fn href_renders_a_link() {
    let html = render(|| html! { <StatTile label="Visitors" value="1" href="/visitors" /> });
    assert!(html.starts_with("<a"));
    assert!(html.contains(r#"href="/visitors""#));
}

#[test]
fn variants_sizes_and_tones_are_distinguishable() {
    let mut rendered = vec![render(|| html! { <StatTile label="A" value="1" /> })];
    rendered.extend(
        StatTileVariant::ALL
            .into_iter()
            .skip(1)
            .map(|variant| render(move || html! { <StatTile label="A" value="1" {variant} /> })),
    );
    rendered.extend(
        [Size::Small, Size::Large].map(|size| render(move || html! { <StatTile label="A" value="1" {size} /> })),
    );
    rendered.extend(
        Tone::ALL.into_iter().skip(1).map(|tone| render(move || html! { <StatTile label="A" value="1" {tone} /> })),
    );
    for (i, a) in rendered.iter().enumerate() {
        for b in &rendered[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html = render(|| {
        html! {
            <StatTile
                id="visitors"
                class="mine"
                attrs={attrs! { "aria-describedby" => "hint" }}
                icon={icons::USERS}
                label="Visitors"
                value="1"
            />
        }
    });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="visitors""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"aria-describedby="hint""#));
}
