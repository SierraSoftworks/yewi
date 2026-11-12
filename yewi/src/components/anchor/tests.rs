use super::*;

const VIEWPORT: AnchorRect = AnchorRect::new(0.0, 0.0, 1000.0, 800.0);
const SIZE: (f64, f64) = (200.0, 40.0);
/// An anchor with room on every side.
const CENTRED: AnchorRect = AnchorRect::new(450.0, 380.0, 100.0, 40.0);

fn place(anchor: AnchorRect, placement: Placement) -> Anchored {
    place_anchored(anchor, SIZE, VIEWPORT, &AnchorOptions::new(placement))
}

fn overlaps(placed: &Anchored, anchor: AnchorRect) -> bool {
    placed.x < anchor.x + anchor.width
        && placed.x + SIZE.0 > anchor.x
        && placed.y < anchor.y + anchor.height
        && placed.y + SIZE.1 > anchor.y
}

#[test]
fn every_placement_sits_on_its_side_without_covering_the_anchor() {
    for placement in Placement::ALL {
        let placed = place(CENTRED, placement);
        assert_eq!(placed.placement, placement, "{placement:?} has room, so it does not flip");
        assert!(!overlaps(&placed, CENTRED), "{placement:?} covers its anchor");
        match placement.side() {
            "top" => assert!(placed.y + SIZE.1 <= CENTRED.y),
            "bottom" => assert!(placed.y >= CENTRED.y + CENTRED.height),
            "left" => assert!(placed.x + SIZE.0 <= CENTRED.x),
            _ => assert!(placed.x >= CENTRED.x + CENTRED.width),
        }
    }
}

#[test]
fn alignment_lines_up_the_matching_edges() {
    assert_eq!(place(CENTRED, Placement::BottomStart).x, CENTRED.x);
    assert_eq!(place(CENTRED, Placement::BottomEnd).x + SIZE.0, CENTRED.x + CENTRED.width);
    assert_eq!(place(CENTRED, Placement::Bottom).x + SIZE.0 / 2.0, CENTRED.x + CENTRED.width / 2.0);
    assert_eq!(place(CENTRED, Placement::RightStart).y, CENTRED.y);
    assert_eq!(place(CENTRED, Placement::RightEnd).y + SIZE.1, CENTRED.y + CENTRED.height);
}

#[test]
fn flips_to_the_opposite_side_when_the_preferred_one_lacks_room() {
    let cases = [
        (AnchorRect::new(450.0, 10.0, 100.0, 40.0), Placement::Top, Placement::Bottom),
        (AnchorRect::new(450.0, 750.0, 100.0, 40.0), Placement::BottomStart, Placement::TopStart),
        (AnchorRect::new(10.0, 380.0, 100.0, 40.0), Placement::Left, Placement::Right),
        (AnchorRect::new(890.0, 380.0, 100.0, 40.0), Placement::RightEnd, Placement::LeftEnd),
    ];
    for (anchor, preferred, expected) in cases {
        assert_eq!(place(anchor, preferred).placement, expected, "{preferred:?}");
    }
}

#[test]
fn keeps_the_preferred_side_when_the_opposite_one_is_worse_or_flipping_is_off() {
    // Too little room above, even less below.
    let squeezed = AnchorRect::new(450.0, 30.0, 100.0, 760.0);
    assert_eq!(place(squeezed, Placement::Top).placement, Placement::Top);

    let near_top = AnchorRect::new(450.0, 10.0, 100.0, 40.0);
    let fixed = place_anchored(near_top, SIZE, VIEWPORT, &AnchorOptions::new(Placement::Top).flip(false));
    assert_eq!(fixed.placement, Placement::Top);
}

#[test]
fn shifts_along_the_side_to_stay_inside_the_viewport() {
    let options = AnchorOptions::new(Placement::Bottom);
    let cases = [AnchorRect::new(2.0, 380.0, 40.0, 40.0), AnchorRect::new(958.0, 380.0, 40.0, 40.0)];
    for anchor in cases {
        let placed = place_anchored(anchor, SIZE, VIEWPORT, &options);
        assert!(placed.x >= options.padding, "{anchor:?}");
        assert!(placed.x + SIZE.0 <= VIEWPORT.width - options.padding, "{anchor:?}");

        let unshifted = place_anchored(anchor, SIZE, VIEWPORT, &options.shift(false));
        assert_eq!(unshifted.x + SIZE.0 / 2.0, anchor.x + anchor.width / 2.0);
    }
}

#[test]
fn the_arrow_points_at_the_anchor_and_stays_off_the_corners() {
    let options = AnchorOptions::new(Placement::Bottom);

    let centred = place_anchored(CENTRED, SIZE, VIEWPORT, &options);
    assert_eq!(centred.arrow, SIZE.0 / 2.0);

    // Shifted away from the left edge: the arrow follows the anchor.
    let anchor = AnchorRect::new(40.0, 380.0, 40.0, 40.0);
    let shifted = place_anchored(anchor, SIZE, VIEWPORT, &options);
    assert_eq!(shifted.x + shifted.arrow, anchor.x + anchor.width / 2.0);

    // An anchor past the floating element's end cannot drag the arrow off it.
    let far = place_anchored(AnchorRect::new(0.0, 380.0, 4.0, 40.0), SIZE, VIEWPORT, &options);
    assert!(far.arrow >= options.arrow_padding);
}

#[test]
fn the_gap_separates_the_floating_element_from_its_anchor() {
    let placed = place_anchored(CENTRED, SIZE, VIEWPORT, &AnchorOptions::new(Placement::Bottom).offset(12.0));
    assert_eq!(placed.y, CENTRED.y + CENTRED.height + 12.0);
}

#[test]
fn reports_an_anchor_that_left_the_viewport() {
    assert!(place(CENTRED, Placement::Top).anchor_visible);
    assert!(!place(AnchorRect::new(450.0, -100.0, 100.0, 40.0), Placement::Top).anchor_visible);
    assert!(!place(AnchorRect::new(1200.0, 380.0, 100.0, 40.0), Placement::Top).anchor_visible);
}
