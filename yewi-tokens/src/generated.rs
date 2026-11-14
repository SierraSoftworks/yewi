// Generated from yewi/scss/tokens.scss by `cargo xtask tokens`. Do not edit.

use crate::{Color, Rgba, Shadow};

/// `--yi-accent`
pub const ACCENT: Color = Color { light: Rgba::new(64, 158, 255, 255), dark: Rgba::new(105, 180, 255, 255) };

/// `--yi-neutral-tint`
pub const NEUTRAL_TINT: f32 = 1.0;

/// `--yi-bg`
pub const BG: Color = Color { light: Rgba::new(244, 246, 249, 255), dark: Rgba::new(23, 27, 32, 255) };

/// `--yi-surface`
pub const SURFACE: Color = Color { light: Rgba::new(255, 255, 255, 255), dark: Rgba::new(30, 35, 40, 255) };

/// `--yi-surface-raised`
pub const SURFACE_RAISED: Color = Color { light: Rgba::new(255, 255, 255, 255), dark: Rgba::new(38, 43, 49, 255) };

/// `--yi-tone-1`
pub const TONE_1: Color = Color { light: Rgba::new(240, 243, 248, 255), dark: Rgba::new(38, 43, 48, 255) };

/// `--yi-tone-2`
pub const TONE_2: Color = Color { light: Rgba::new(247, 249, 252, 255), dark: Rgba::new(34, 39, 44, 255) };

/// `--yi-line`
pub const LINE: Color = Color { light: Rgba::new(225, 231, 238, 255), dark: Rgba::new(49, 54, 59, 255) };

/// `--yi-line-strong`
pub const LINE_STRONG: Color = Color { light: Rgba::new(199, 206, 214, 255), dark: Rgba::new(68, 75, 83, 255) };

/// `--yi-fg`
pub const FG: Color = Color { light: Rgba::new(13, 24, 36, 255), dark: Rgba::new(204, 209, 214, 255) };

/// `--yi-fg-muted`
pub const FG_MUTED: Color = Color { light: Rgba::new(86, 95, 106, 255), dark: Rgba::new(154, 161, 169, 255) };

/// `--yi-fg-subtle`
pub const FG_SUBTLE: Color = Color { light: Rgba::new(99, 108, 119, 255), dark: Rgba::new(141, 150, 159, 255) };

/// `--yi-accent-solid`
pub const ACCENT_SOLID: Color = Color { light: Rgba::new(14, 106, 187, 255), dark: Rgba::new(105, 180, 255, 255) };

/// `--yi-accent-solid-hover`
pub const ACCENT_SOLID_HOVER: Color = Color { light: Rgba::new(10, 88, 158, 255), dark: Rgba::new(124, 189, 255, 255) };

/// `--yi-accent-text`
pub const ACCENT_TEXT: Color = Color { light: Rgba::new(0, 93, 180, 255), dark: Rgba::new(107, 182, 255, 255) };

/// `--yi-accent-soft`
pub const ACCENT_SOFT: Color = Color { light: Rgba::new(64, 158, 255, 31), dark: Rgba::new(105, 180, 255, 31) };

/// `--yi-accent-line`
pub const ACCENT_LINE: Color = Color { light: Rgba::new(64, 158, 255, 115), dark: Rgba::new(105, 180, 255, 115) };

/// `--yi-on-solid`
pub const ON_SOLID: Color = Color { light: Rgba::new(255, 255, 255, 255), dark: Rgba::new(12, 17, 23, 255) };

/// `--yi-focus`
pub const FOCUS: Color = Color { light: Rgba::new(0, 93, 180, 255), dark: Rgba::new(107, 182, 255, 255) };

/// `--yi-ok`
pub const OK: Color = Color { light: Rgba::new(20, 111, 67, 255), dark: Rgba::new(76, 191, 127, 255) };

/// `--yi-warn`
pub const WARN: Color = Color { light: Rgba::new(133, 89, 0, 255), dark: Rgba::new(224, 169, 58, 255) };

/// `--yi-danger`
pub const DANGER: Color = Color { light: Rgba::new(184, 44, 36, 255), dark: Rgba::new(242, 121, 112, 255) };

/// `--yi-info`
pub const INFO: Color = Color { light: Rgba::new(42, 99, 181, 255), dark: Rgba::new(122, 174, 240, 255) };

/// `--yi-active`
pub const ACTIVE: Color = Color { light: Rgba::new(15, 102, 150, 255), dark: Rgba::new(95, 183, 232, 255) };

/// `--yi-neutral`
pub const NEUTRAL: Color = Color { light: Rgba::new(90, 100, 112, 255), dark: Rgba::new(154, 163, 176, 255) };

/// `--yi-ok-soft`
pub const OK_SOFT: Color = Color { light: Rgba::new(20, 111, 67, 31), dark: Rgba::new(76, 191, 127, 31) };

/// `--yi-warn-soft`
pub const WARN_SOFT: Color = Color { light: Rgba::new(133, 89, 0, 33), dark: Rgba::new(224, 169, 58, 33) };

/// `--yi-danger-soft`
pub const DANGER_SOFT: Color = Color { light: Rgba::new(184, 44, 36, 31), dark: Rgba::new(242, 121, 112, 31) };

/// `--yi-info-soft`
pub const INFO_SOFT: Color = Color { light: Rgba::new(42, 99, 181, 31), dark: Rgba::new(122, 174, 240, 31) };

/// `--yi-active-soft`
pub const ACTIVE_SOFT: Color = Color { light: Rgba::new(15, 102, 150, 31), dark: Rgba::new(95, 183, 232, 31) };

/// `--yi-neutral-soft`
pub const NEUTRAL_SOFT: Color = Color { light: Rgba::new(90, 100, 112, 33), dark: Rgba::new(154, 163, 176, 33) };

/// `--yi-code-bg`
pub const CODE_BG: Color = Color { light: Rgba::new(242, 247, 252, 255), dark: Rgba::new(22, 28, 33, 255) };

/// `--yi-syntax-comment`
pub const SYNTAX_COMMENT: Color = Color { light: Rgba::new(99, 108, 119, 255), dark: Rgba::new(141, 150, 159, 255) };

/// `--yi-syntax-keyword`
pub const SYNTAX_KEYWORD: Color = Color { light: Rgba::new(143, 58, 128, 255), dark: Rgba::new(213, 139, 208, 255) };

/// `--yi-syntax-string`
pub const SYNTAX_STRING: Color = Color { light: Rgba::new(20, 111, 67, 255), dark: Rgba::new(76, 191, 127, 255) };

/// `--yi-syntax-number`
pub const SYNTAX_NUMBER: Color = Color { light: Rgba::new(133, 89, 0, 255), dark: Rgba::new(224, 169, 58, 255) };

/// `--yi-syntax-type`
pub const SYNTAX_TYPE: Color = Color { light: Rgba::new(11, 106, 117, 255), dark: Rgba::new(92, 196, 207, 255) };

/// `--yi-syntax-function`
pub const SYNTAX_FUNCTION: Color = Color { light: Rgba::new(42, 99, 181, 255), dark: Rgba::new(122, 174, 240, 255) };

/// `--yi-syntax-property`
pub const SYNTAX_PROPERTY: Color = Color { light: Rgba::new(168, 56, 44, 255), dark: Rgba::new(240, 144, 127, 255) };

/// `--yi-font-system`
pub const FONT_SYSTEM: &[&str] = &["system-ui", "-apple-system", "Segoe UI", "Roboto", "Helvetica Neue", "Arial", "sans-serif"];

/// `--yi-font-system-mono`
pub const FONT_SYSTEM_MONO: &[&str] = &["ui-monospace", "SFMono-Regular", "Menlo", "Consolas", "Liberation Mono", "monospace"];

/// `--yi-font-heading`
pub const FONT_HEADING: &[&str] = &["Montserrat", "system-ui", "-apple-system", "Segoe UI", "Roboto", "Helvetica Neue", "Arial", "sans-serif"];

/// `--yi-font-body`
pub const FONT_BODY: &[&str] = &["Inter", "system-ui", "-apple-system", "Segoe UI", "Roboto", "Helvetica Neue", "Arial", "sans-serif"];

/// `--yi-font-mono`
pub const FONT_MONO: &[&str] = &["Fira Code", "ui-monospace", "SFMono-Regular", "Menlo", "Consolas", "Liberation Mono", "monospace"];

/// `--yi-font-mono-ligatures`
pub const FONT_MONO_LIGATURES: &str = "none";

/// `--yi-text-xs`
pub const TEXT_XS: f32 = 12.0;

/// `--yi-text-sm`
pub const TEXT_SM: f32 = 13.0;

/// `--yi-text-md`
pub const TEXT_MD: f32 = 14.0;

/// `--yi-text-lg`
pub const TEXT_LG: f32 = 15.0;

/// `--yi-text-xl`
pub const TEXT_XL: f32 = 17.0;

/// `--yi-text-2xl`
pub const TEXT_2XL: f32 = 20.0;

/// `--yi-text-3xl`
pub const TEXT_3XL: f32 = 24.0;

/// `--yi-text-4xl`
pub const TEXT_4XL: f32 = 30.0;

/// `--yi-heading-weight-md`
pub const HEADING_WEIGHT_MD: u16 = 600;

/// `--yi-heading-weight-lg`
pub const HEADING_WEIGHT_LG: u16 = 600;

/// `--yi-heading-weight-xl`
pub const HEADING_WEIGHT_XL: u16 = 500;

/// `--yi-heading-weight-2xl`
pub const HEADING_WEIGHT_2XL: u16 = 400;

/// `--yi-heading-weight-3xl`
pub const HEADING_WEIGHT_3XL: u16 = 400;

/// `--yi-heading-weight-4xl`
pub const HEADING_WEIGHT_4XL: u16 = 300;

/// `--yi-leading-tight`
pub const LEADING_TIGHT: f32 = 1.25;

/// `--yi-leading`
pub const LEADING: f32 = 1.5;

/// `--yi-leading-relaxed`
pub const LEADING_RELAXED: f32 = 1.65;

/// `--yi-space-2xs`
pub const SPACE_2XS: f32 = 2.0;

/// `--yi-space-xs`
pub const SPACE_XS: f32 = 4.0;

/// `--yi-space-sm`
pub const SPACE_SM: f32 = 8.0;

/// `--yi-space-md`
pub const SPACE_MD: f32 = 12.0;

/// `--yi-space-lg`
pub const SPACE_LG: f32 = 16.0;

/// `--yi-space-xl`
pub const SPACE_XL: f32 = 24.0;

/// `--yi-space-2xl`
pub const SPACE_2XL: f32 = 32.0;

/// `--yi-space-3xl`
pub const SPACE_3XL: f32 = 48.0;

/// `--yi-space-4xl`
pub const SPACE_4XL: f32 = 64.0;

/// `--yi-radius-sm`
pub const RADIUS_SM: f32 = 4.0;

/// `--yi-radius-control`
pub const RADIUS_CONTROL: f32 = 6.0;

/// `--yi-radius-card`
pub const RADIUS_CARD: f32 = 9.0;

/// `--yi-radius-full`
pub const RADIUS_FULL: f32 = 999.0;

/// `--yi-control-height-sm`
pub const CONTROL_HEIGHT_SM: f32 = 28.0;

/// `--yi-control-height`
pub const CONTROL_HEIGHT: f32 = 32.0;

/// `--yi-control-height-lg`
pub const CONTROL_HEIGHT_LG: f32 = 36.0;

/// `--yi-control-padding`
pub const CONTROL_PADDING: f32 = 14.0;

/// `--yi-icon-stroke`
pub const ICON_STROKE: f32 = 1.5;

/// `--yi-disabled-opacity`
pub const DISABLED_OPACITY: f32 = 0.5;

/// `--yi-card-line`
pub const CARD_LINE: Color = Color { light: Rgba::new(225, 231, 238, 179), dark: Rgba::new(49, 54, 59, 179) };

/// `--yi-shadow-card`
pub const SHADOW_CARD: &[Shadow] = &[
    Shadow { x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color { light: Rgba::new(20, 26, 34, 15), dark: Rgba::new(0, 0, 0, 77) } },
    Shadow { x: 0.0, y: 4.0, blur: 14.0, spread: 0.0, color: Color { light: Rgba::new(20, 26, 34, 18), dark: Rgba::new(0, 0, 0, 56) } },
];

/// `--yi-shadow-overlay`
pub const SHADOW_OVERLAY: &[Shadow] = &[
    Shadow { x: 0.0, y: 1.0, blur: 2.0, spread: 0.0, color: Color { light: Rgba::new(20, 26, 34, 20), dark: Rgba::new(0, 0, 0, 102) } },
    Shadow { x: 0.0, y: 8.0, blur: 24.0, spread: 0.0, color: Color { light: Rgba::new(20, 26, 34, 31), dark: Rgba::new(0, 0, 0, 97) } },
];

/// `--yi-scrim`
pub const SCRIM: Color = Color { light: Rgba::new(20, 26, 34, 97), dark: Rgba::new(0, 0, 0, 115) };

/// `--yi-z-overlay`
pub const Z_OVERLAY: u16 = 60;

/// `--yi-duration-fast`
pub const DURATION_FAST: f32 = 120.0;

/// `--yi-duration`
pub const DURATION: f32 = 180.0;

/// `--yi-ease`
pub const EASE: [f32; 4] = [0.2, 0.0, 0.0, 1.0];

/// `--yi-duration-spin`
pub const DURATION_SPIN: f32 = 810.0;

/// `--yi-duration-pulse`
pub const DURATION_PULSE: f32 = 1600.0;

/// The tokens that `data-yi-density="compact"` changes.
pub mod compact {
    /// `--yi-control-height`
    pub const CONTROL_HEIGHT: f32 = 28.0;

    /// `--yi-control-height-sm`
    pub const CONTROL_HEIGHT_SM: f32 = 24.0;

    /// `--yi-control-height-lg`
    pub const CONTROL_HEIGHT_LG: f32 = 32.0;

    /// `--yi-control-padding`
    pub const CONTROL_PADDING: f32 = 10.0;
}
