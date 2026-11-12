use yewi_test::render;

use super::*;
use crate::attrs;

fn bones(html: &str) -> usize {
    html.matches("yi-skeleton__bone").count()
}

#[test]
fn is_hidden_from_assistive_technology() {
    for shape in SkeletonShape::ALL {
        let html = render(move || html! { <Skeleton {shape} /> });
        assert!(html.contains(r#"aria-hidden="true""#), "{shape:?}");
    }
}

#[test]
fn text_renders_one_placeholder_per_line() {
    for (lines, expected) in [(0, 1), (1, 1), (4, 4)] {
        let html = render(move || html! { <Skeleton {lines} /> });
        assert_eq!(bones(&html), expected, "lines={lines}");
    }
}

#[test]
fn only_text_repeats() {
    for shape in [SkeletonShape::Block, SkeletonShape::Circle] {
        let html = render(move || html! { <Skeleton {shape} lines=3 /> });
        assert_eq!(bones(&html), 1, "{shape:?}");
    }
}

#[test]
fn width_and_height_reach_the_element() {
    let html = render(|| html! { <Skeleton shape={SkeletonShape::Block} width="12rem" height="6rem" /> });
    assert!(html.contains("12rem"));
    assert!(html.contains("6rem"));
}

#[test]
fn shapes_and_animations_are_distinguishable() {
    let mut rendered: Vec<String> =
        SkeletonShape::ALL.into_iter().map(|shape| render(move || html! { <Skeleton {shape} /> })).collect();
    rendered.extend(
        SkeletonAnimation::ALL
            .into_iter()
            .skip(1)
            .map(|animation| render(move || html! { <Skeleton {animation} /> })),
    );
    for (i, a) in rendered.iter().enumerate() {
        for b in &rendered[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html = render(|| html! { <Skeleton id="x" class="mine" attrs={attrs! { "data-test" => "1" }} /> });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="x""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"data-test="1""#));
}
