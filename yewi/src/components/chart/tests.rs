use yew::prelude::*;
use yewi_test::render;

use super::geometry::*;
use super::*;
use crate::attrs;

fn chart(series: Vec<Series>) -> String {
    render(move || html! { <Chart label="Visitors per day" series={series.clone()} /> })
}

/// The number of drawing commands in the path data of a rendered chart.
fn commands(html: &str) -> usize {
    html.matches(['M', 'L']).count()
}

// ---- Geometry ----------------------------------------------------------------

#[test]
fn scales_round_outwards_and_include_zero() {
    for (min, max) in [(0.0, 87.0), (3.0, 1_204.0), (-40.0, 12.0), (0.0, 0.0), (0.2, 0.7), (5e6, 9.1e6)] {
        let scale = Scale::nice(f64::min(min, 0.0), f64::max(max, 0.0));
        assert!(scale.lo <= min.min(0.0) && scale.hi >= max, "{min}..{max} gave {scale:?}");
        assert!(scale.hi > scale.lo);
        assert!((2..=7).contains(&scale.ticks.len()), "{min}..{max} gave {} ticks", scale.ticks.len());
        assert!(scale.ticks.iter().any(|t| t.abs() < 1e-9), "{min}..{max} has no zero tick");
    }
}

#[test]
fn axis_positions_round_trip() {
    for axis in [Axis { n: 1, banded: false }, Axis { n: 30, banded: false }, Axis { n: 30, banded: true }] {
        for i in 0..axis.n {
            assert_eq!(axis.index(axis.frac(i as f64)), i);
        }
        assert_eq!(axis.index(-1.0), 0);
        assert_eq!(axis.index(2.0), axis.n - 1);
    }
}

#[test]
fn reducing_a_long_series_bounds_its_size_and_keeps_extremes() {
    let mut data: Vec<f64> = (0..100_000).map(|i| f64::from(i % 97)).collect();
    data[31_337] = 5_000.0;
    data[77_777] = -5_000.0;

    let points: Vec<(usize, f64)> = samples(&data, 500).into_iter().flatten().collect();
    assert!(points.len() <= 1_000);
    assert!(points.contains(&(31_337, 5_000.0)));
    assert!(points.contains(&(77_777, -5_000.0)));
    assert!(points.windows(2).all(|pair| pair[0].0 < pair[1].0), "points stay in order");
}

#[test]
fn short_series_are_not_reduced() {
    let data = [1.0, 2.0, 3.0];
    assert_eq!(samples(&data, 1000).len(), 3);
}

#[test]
fn non_finite_values_split_a_line() {
    let scale = Scale::nice(0.0, 10.0);
    let axis = Axis { n: 5, banded: false };
    let whole = line_paths(&[1.0, 2.0, 3.0, 4.0, 5.0], axis, &scale, 1000);
    let split = line_paths(&[1.0, 2.0, f64::NAN, 4.0, 5.0], axis, &scale, 1000);
    assert_eq!(whole.line.matches('M').count(), 1);
    assert_eq!(split.line.matches('M').count(), 2);
    assert_eq!(split.area.matches('Z').count(), 2);
}

#[test]
fn numbers_format_for_reading() {
    for (value, full, compact) in [
        (0.0, "0", "0"),
        (950.0, "950", "950"),
        (1_204.0, "1,204", "1.2k"),
        (12_500.0, "12,500", "12.5k"),
        (3_000_000.0, "3,000,000", "3M"),
        (-1_250.5, "-1,250.5", "-1.3k"),
        (0.25, "0.25", "0.25"),
    ] {
        assert_eq!(format_number(value), full);
        assert_eq!(format_compact(value), compact);
    }
    assert_eq!(percent(0.375), "37.50%");
}

// ---- Chart -------------------------------------------------------------------

#[test]
fn is_named_and_exposes_the_latest_values_to_assistive_technology() {
    let html = chart(vec![Series::new("Visitors", [10.0, 20.0, 1_204.0])]);
    assert!(html.contains(r#"role="slider""#));
    assert!(html.contains(r#"aria-label="Visitors per day""#));
    assert!(html.contains(r#"aria-valuemax="2""#));
    assert!(html.contains(r#"aria-valuetext="3: Visitors 1,204""#));
}

#[test]
fn a_static_chart_is_an_image() {
    let html = render(|| {
        html! { <Chart label="Visitors per day" interactive=false series={vec![Series::new("Visitors", [1.0, 2.0])]} /> }
    });
    assert!(html.contains(r#"role="img""#));
    assert!(!html.contains("tabindex"));
}

#[test]
fn the_legend_appears_once_there_is_more_than_one_series() {
    let one = chart(vec![Series::new("Visitors", [1.0, 2.0])]);
    let two = chart(vec![Series::new("Visitors", [1.0, 2.0]), Series::new("Previous period", [2.0, 1.0])]);
    assert!(!one.contains("<ul"));
    assert!(two.contains("Previous period"));
    assert_eq!(two.matches("<li ").count(), 2);
}

#[test]
fn the_first_series_is_an_area_and_the_rest_are_lines() {
    let one = chart(vec![Series::new("A", [1.0, 2.0])]);
    let two = chart(vec![Series::new("A", [1.0, 2.0]), Series::new("B", [2.0, 1.0])]);
    let overridden = chart(vec![Series::new("A", [1.0, 2.0]).kind(SeriesKind::Line)]);
    assert_eq!(one.matches("<linearGradient").count(), 1);
    assert_eq!(two.matches("<linearGradient").count(), 1);
    assert_eq!(overridden.matches("<linearGradient").count(), 0);
}

#[test]
fn series_options_change_the_rendering() {
    let plain = chart(vec![Series::new("A", [1.0, 2.0])]);
    for series in [
        Series::new("A", [1.0, 2.0]).dash(SeriesDash::Dashed),
        Series::new("A", [1.0, 2.0]).dash(SeriesDash::Dotted),
        Series::new("A", [1.0, 2.0]).markers(true),
        Series::new("A", [1.0, 2.0]).color(SeriesColor::Muted),
        Series::new("A", [1.0, 2.0]).color(SeriesColor::Slot(3)),
        Series::new("A", [1.0, 2.0]).color(SeriesColor::Tone(Tone::Danger)),
        Series::new("A", [1.0, 2.0]).kind(SeriesKind::Bars),
    ] {
        assert_ne!(chart(vec![series.clone()]), plain, "{series:?}");
    }
}

#[test]
fn large_datasets_render_a_bounded_amount_of_markup() {
    let small = chart(vec![Series::new("A", (0..10_000).map(|i| f64::from(i % 113)).collect::<SeriesData>())]);
    let large = chart(vec![Series::new("A", (0..1_000_000).map(|i| f64::from(i % 113)).collect::<SeriesData>())]);
    // 1000 buckets, two points each, for the line and again for the area.
    assert!(commands(&large) <= 4_200, "{} commands", commands(&large));
    assert!(large.len() < small.len() * 2);
}

#[test]
fn labels_come_from_the_callbacks() {
    let html = render(|| {
        let x_label = Callback::from(|i: usize| AttrValue::from(format!("Day {}", i + 1)));
        let y_label = Callback::from(|v: f64| AttrValue::from(format!("{v} ms")));
        html! { <Chart label="Latency" series={vec![Series::new("p95", [120.0, 80.0])]} {x_label} {y_label} /> }
    });
    assert!(html.contains("Day 1"));
    assert!(html.contains(r#"aria-valuetext="Day 2: p95 80 ms""#));
}

#[test]
fn no_data_shows_the_empty_message() {
    let html = render(|| html! { <Chart label="Visitors" series={Vec::new()} empty="Nothing in this period" /> });
    assert!(html.contains("Nothing in this period"));
    assert!(!html.contains("<svg"));
}

#[test]
fn series_data_is_equal_only_to_its_clones() {
    let data = SeriesData::from(vec![1.0, 2.0]);
    assert_eq!(data, data.clone());
    assert_ne!(data, SeriesData::from(vec![1.0, 2.0]));
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html = render(|| {
        html! {
            <Chart id="traffic" class="mine" attrs={attrs! { "data-panel" => "main" }} label="Visitors"
                series={vec![Series::new("Visitors", [1.0, 2.0])]} />
        }
    });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="traffic""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"data-panel="main""#));
}

// ---- Sparkline ---------------------------------------------------------------

#[test]
fn a_sparkline_is_decorative_unless_labelled() {
    let plain = render(|| html! { <Sparkline data={SeriesData::from([1.0, 3.0, 2.0])} /> });
    let labelled = render(|| html! { <Sparkline data={SeriesData::from([1.0, 3.0, 2.0])} label="Errors, rising" /> });
    assert!(plain.contains(r#"aria-hidden="true""#));
    assert!(labelled.contains(r#"role="img""#));
    assert!(labelled.contains(r#"aria-label="Errors, rising""#));
    assert!(!labelled.contains("aria-hidden"));
}

#[test]
fn a_sparkline_renders_any_data_without_panicking() {
    for data in [vec![], vec![0.0], vec![0.0; 8], vec![f64::NAN, 1.0], vec![-3.0, 4.0, -1.0]] {
        let html = render(move || html! { <Sparkline data={SeriesData::from(data.clone())} area=false /> });
        assert!(html.starts_with("<svg"));
    }
}
