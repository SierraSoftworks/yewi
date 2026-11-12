use yew::prelude::*;
use yewi_test::render;

use super::*;
use crate::attrs;

const LINK: &str = "https://example.com/enrol?token=7f3a9c";

/// Rasterises the path the component draws and reads it back with a QR
/// decoder, as a phone camera would.
fn scan(drawing: &Drawing) -> String {
    const SCALE: usize = 4;
    let span = drawing.span as usize;
    let mut dark = vec![false; span * span];
    for run in drawing.path.split('M').skip(1) {
        let (start, rest) = run.split_once('h').unwrap();
        let (x, y) = start.split_once(' ').unwrap();
        let (x, y): (usize, usize) = (x.parse().unwrap(), y.parse().unwrap());
        let len: usize = rest.split_once('v').unwrap().0.parse().unwrap();
        dark[y * span + x..y * span + x + len].fill(true);
    }

    let mut image = rqrr::PreparedImage::prepare_from_greyscale(span * SCALE, span * SCALE, |x, y| {
        if dark[(y / SCALE) * span + x / SCALE] { 0 } else { 255 }
    });
    let grids = image.detect_grids();
    assert_eq!(grids.len(), 1, "expected exactly one code in the drawing");
    grids[0].decode().expect("the code decodes").1
}

#[test]
fn adjacent_modules_become_one_run() {
    // ██
    //  █
    let dark = [true, true, false, true];
    let path = modules_path(2, 4, |x, y| dark[(y * 2 + x) as usize]);
    assert_eq!(path, "M4 4h2v1h-2zM5 5h1v1h-1z");
}

#[test]
fn the_drawn_code_scans_back_to_its_value() {
    let values = [LINK, "0123456789", "WIFI:T:WPA;S:Guest;P:correct horse;;", "Grüße, 世界"];
    for value in values {
        for ecc in QrEcc::ALL {
            let drawing = Drawing::new(value, ecc, 4).expect("fits");
            assert_eq!(scan(&drawing), value, "{value:?} at {ecc:?}");
        }
    }
}

#[test]
fn the_quiet_zone_pads_every_side() {
    let tight = Drawing::new(LINK, QrEcc::Medium, 0).unwrap();
    let padded = Drawing::new(LINK, QrEcc::Medium, 4).unwrap();
    assert_eq!(padded.span, tight.span + 8);
    assert!(padded.path.starts_with("M4 4h7"), "the top-left finder moves in by the quiet zone");
}

#[test]
fn higher_error_correction_never_shrinks_the_code() {
    let spans: Vec<u32> = QrEcc::ALL.into_iter().map(|ecc| Drawing::new(LINK, ecc, 4).unwrap().span).collect();
    assert!(spans.is_sorted(), "{spans:?}");
    assert!(spans.first() < spans.last());
}

#[test]
fn renders_a_labelled_image() {
    let html = render(|| html! { <QrCode value={LINK} label="Enrolment link" /> });
    assert!(html.contains(r#"role="img""#));
    assert!(html.contains(r#"aria-label="Enrolment link""#));
    assert!(!html.contains(LINK), "the value is not exposed as text");
}

#[test]
fn children_become_the_caption() {
    let with = render(|| html! { <QrCode value={LINK}>{ "Scan to enrol" }</QrCode> });
    let without = render(|| html! { <QrCode value={LINK} /> });
    assert!(with.contains("<figcaption") && with.contains("Scan to enrol"));
    assert!(!without.contains("<figcaption"));
}

#[test]
fn a_value_that_cannot_fit_is_reported_instead_of_drawn() {
    let default = render(|| html! { <QrCode value={"x".repeat(4000)} /> });
    assert!(default.contains(r#"role="alert""#));
    assert!(default.contains("too long"));
    assert!(!default.contains("<svg"));

    let custom = render(|| html! { <QrCode value={"x".repeat(4000)} fallback={html! { "Use the link instead." }} /> });
    assert!(custom.contains("Use the link instead."));
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html = render(|| {
        html! { <QrCode value={LINK} id="enrol" class="mine" size=160 attrs={attrs! { "data-kind" => "enrol" }} /> }
    });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="enrol""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"data-kind="enrol""#));
    assert!(root.contains("160px"));
}
