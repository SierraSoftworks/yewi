#![cfg(feature = "qr")]
//! QR codes, encoded in the browser (or on the server) and drawn as SVG.
//!
//! The value never leaves the page: it is encoded by a pure-Rust encoder and
//! drawn as a single `<path>`, so a secret such as an enrolment link is not
//! handed to an image service, and no markup is injected.

use qrcodegen::{QrCode as Encoded, QrCodeEcc};
use yew::prelude::*;

use crate::Attrs;

/// How much damage a code survives, traded against how dense it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum QrEcc {
    /// Recovers about 7% damage. The sparsest code, for clean screens.
    Low,
    /// Recovers about 15% damage. Right for most on-screen codes.
    #[default]
    Medium,
    /// Recovers about 25% damage.
    Quartile,
    /// Recovers about 30% damage, for print and codes that may be obscured.
    High,
}

impl QrEcc {
    pub const ALL: [QrEcc; 4] = [QrEcc::Low, QrEcc::Medium, QrEcc::Quartile, QrEcc::High];

    const fn level(self) -> QrCodeEcc {
        match self {
            QrEcc::Low => QrCodeEcc::Low,
            QrEcc::Medium => QrCodeEcc::Medium,
            QrEcc::Quartile => QrCodeEcc::Quartile,
            QrEcc::High => QrCodeEcc::High,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct QrCodeProps {
    /// The text the code encodes, usually a link.
    pub value: AttrValue,
    /// A caption shown under the code, such as what scanning it does.
    #[prop_or_default]
    pub children: Html,
    /// What the code is, for people who cannot see it.
    #[prop_or(AttrValue::Static("QR code"))]
    pub label: AttrValue,
    /// Error correction. Higher levels survive more damage but produce a
    /// denser code that is harder to scan when small.
    #[prop_or_default]
    pub ecc: QrEcc,
    /// The edge of the code in CSS pixels. Defaults to the
    /// `--yi-qr-code-size` token.
    #[prop_or_default]
    pub size: Option<u32>,
    /// The blank margin around the code, in modules. Scanners expect 4;
    /// lower it only when the surrounding surface is already white.
    #[prop_or(4)]
    pub quiet_zone: u32,
    /// Shown instead of the code when `value` is too long to encode.
    /// Defaults to a short message.
    #[prop_or_default]
    pub fallback: Option<Html>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A scannable QR code for a link or other short text.
///
/// The code is always drawn dark on white, in both themes, because many
/// scanners cannot read an inverted code.
#[function_component]
pub fn QrCode(props: &QrCodeProps) -> Html {
    let drawing = use_memo((props.value.clone(), props.ecc, props.quiet_zone), |(value, ecc, quiet_zone)| {
        Drawing::new(value, *ecc, *quiet_zone)
    });

    let body = match drawing.as_ref() {
        Some(drawing) => html! {
            <svg
                class="yi-qr-code__image"
                viewBox={format!("0 0 {0} {0}", drawing.span)}
                role="img"
                aria-label={props.label.clone()}
                shape-rendering="crispEdges"
            >
                <rect class="yi-qr-code__paper" width="100%" height="100%" fill="#fff" />
                <path class="yi-qr-code__modules" d={drawing.path.clone()} fill="#000" />
            </svg>
        },
        None => html! {
            <div class="yi-qr-code__error" role="alert">
                if let Some(fallback) = &props.fallback {
                    { fallback.clone() }
                } else {
                    { "This value is too long to show as a QR code." }
                }
            </div>
        },
    };

    props.attrs.apply(html! {
        <figure
            id={props.id.clone()}
            class={classes!("yi-qr-code", props.class.clone())}
            style={props.size.map(|size| format!("--yi-qr-code-size:{size}px"))}
        >
            { body }
            if props.children != Html::default() {
                <figcaption class="yi-qr-code__caption">{ props.children.clone() }</figcaption>
            }
        </figure>
    })
}

/// An encoded value, ready to draw.
#[derive(Debug, PartialEq)]
struct Drawing {
    /// The edge of the drawing in modules, quiet zone included.
    span: u32,
    /// Every dark module, as SVG path data in module units.
    path: AttrValue,
}

impl Drawing {
    /// `None` when `value` does not fit in the largest QR code.
    fn new(value: &str, ecc: QrEcc, quiet_zone: u32) -> Option<Self> {
        let code = Encoded::encode_text(value, ecc.level()).ok()?;
        let width = code.size() as u32;
        let path = modules_path(width, quiet_zone, |x, y| code.get_module(x as i32, y as i32));
        Some(Self { span: width + quiet_zone * 2, path: path.into() })
    }
}

/// One path covering every dark module. Horizontal runs are merged, so a
/// code is a single DOM node and a few kilobytes of path data at most.
fn modules_path(width: u32, offset: u32, is_dark: impl Fn(u32, u32) -> bool) -> String {
    use std::fmt::Write as _;

    let mut path = String::new();
    for y in 0..width {
        let mut x = 0;
        while x < width {
            if !is_dark(x, y) {
                x += 1;
                continue;
            }
            let start = x;
            while x < width && is_dark(x, y) {
                x += 1;
            }
            let run = x - start;
            let _ = write!(path, "M{} {}h{run}v1h-{run}z", start + offset, y + offset);
        }
    }
    path
}

#[cfg(test)]
mod tests;
