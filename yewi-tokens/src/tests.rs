use super::*;

/// WCAG contrast ratio of two opaque colours.
fn contrast(a: Rgba, b: Rgba) -> f32 {
    let luminance = |colour: Rgba| {
        let [r, g, b, _] =
            colour.to_f32().map(|c| if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) });
        0.2126 * r + 0.7152 * g + 0.0722 * b
    };
    let (a, b) = (luminance(a), luminance(b));
    (a.max(b) + 0.05) / (a.min(b) + 0.05)
}

#[test]
fn text_is_legible_on_its_background_in_both_schemes() {
    let cases = [
        ("fg on bg", FG, BG),
        ("fg on surface", FG, SURFACE),
        ("muted on surface", FG_MUTED, SURFACE),
        ("accent text on surface", ACCENT_TEXT, SURFACE),
        ("on-solid on accent", ON_SOLID, ACCENT_SOLID),
        ("danger on surface", DANGER, SURFACE),
        ("comment on code", SYNTAX_COMMENT, CODE_BG),
        ("keyword on code", SYNTAX_KEYWORD, CODE_BG),
        ("string on code", SYNTAX_STRING, CODE_BG),
        ("number on code", SYNTAX_NUMBER, CODE_BG),
        ("type on code", SYNTAX_TYPE, CODE_BG),
        ("function on code", SYNTAX_FUNCTION, CODE_BG),
        ("property on code", SYNTAX_PROPERTY, CODE_BG),
    ];
    for (name, text, background) in cases {
        for scheme in [Scheme::Light, Scheme::Dark] {
            let ratio = contrast(text.resolve(scheme), background.resolve(scheme));
            assert!(ratio >= 4.5, "{name} ({scheme:?}): {ratio:.2}");
        }
    }
}

#[test]
fn schemes_differ_and_soft_fills_are_translucent() {
    assert!(contrast(BG.light, BG.dark) > 10.0);
    assert_eq!(BG.resolve(Scheme::default()), BG.light);
    for soft in [ACCENT_SOFT, OK_SOFT, DANGER_SOFT] {
        assert!(soft.light.a > 0 && soft.light.a < 128);
    }
}

#[test]
fn lengths_are_pixels_and_scales_ascend() {
    assert_eq!(SPACE_LG, 16.0);
    assert_eq!(DURATION_PULSE, 1600.0);
    for scale in [
        &[TEXT_XS, TEXT_SM, TEXT_MD, TEXT_LG, TEXT_XL, TEXT_2XL, TEXT_3XL, TEXT_4XL][..],
        &[SPACE_2XS, SPACE_XS, SPACE_SM, SPACE_MD, SPACE_LG, SPACE_XL, SPACE_2XL, SPACE_3XL, SPACE_4XL],
        &[CONTROL_HEIGHT_SM, CONTROL_HEIGHT, CONTROL_HEIGHT_LG],
        &[compact::CONTROL_HEIGHT, CONTROL_HEIGHT],
    ] {
        assert!(scale.is_sorted_by(|a, b| a < b), "{scale:?}");
    }
}

#[test]
fn fonts_end_in_a_generic_family_and_shadows_have_layers() {
    for stack in [FONT_HEADING, FONT_BODY, FONT_MONO] {
        assert!(stack.len() > 1 && ["sans-serif", "monospace"].contains(stack.last().unwrap()), "{stack:?}");
    }
    for shadow in [SHADOW_CARD, SHADOW_OVERLAY] {
        assert!(!shadow.is_empty() && shadow.iter().all(|layer| layer.blur > 0.0 && layer.color.light.a > 0));
    }
}
