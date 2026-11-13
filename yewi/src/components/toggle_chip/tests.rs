use yewi_test::render;

use super::*;
use crate::Field;

#[test]
fn a_chip_is_a_button_that_reports_whether_it_is_pressed() {
    for (pressed, expected) in [(true, r#"aria-pressed="true""#), (false, r#"aria-pressed="false""#)] {
        let html = render(move || html! { <ToggleChip {pressed}>{ "prod" }</ToggleChip> });
        assert!(html.contains(r#"type="button""#));
        assert!(html.contains(expected), "pressed {pressed}: {html}");
        assert!(html.contains("prod"));
    }
}

#[test]
fn only_a_pressed_chip_without_its_own_icon_shows_a_mark() {
    let marks = |html: String| html.matches("<svg").count();
    assert_eq!(marks(render(|| html! { <ToggleChip>{ "prod" }</ToggleChip> })), 0);
    assert_eq!(marks(render(|| html! { <ToggleChip pressed=true>{ "prod" }</ToggleChip> })), 1);
    for pressed in [false, true] {
        let html = render(move || html! { <ToggleChip {pressed} icon={icons::PIN}>{ "Pinned" }</ToggleChip> });
        assert_eq!(marks(html), 1, "pressed {pressed}");
    }
}

#[test]
fn a_disabled_chip_is_a_disabled_button() {
    let html = render(|| html! { <ToggleChip disabled=true>{ "prod" }</ToggleChip> });
    assert!(html.contains(" disabled"));
}

#[test]
fn a_group_is_named_by_its_label_or_its_field() {
    let named = render(|| html! { <ToggleChipGroup label="Tags"><ToggleChip>{ "prod" }</ToggleChip></ToggleChipGroup> });
    assert!(named.contains(r#"role="group""#));
    assert!(named.contains(r#"aria-label="Tags""#));

    let in_field = render(|| {
        html! {
            <Field label="Tags" group=true>
                <ToggleChipGroup><ToggleChip>{ "prod" }</ToggleChip></ToggleChipGroup>
            </Field>
        }
    });
    assert!(in_field.contains("aria-labelledby="));
}
