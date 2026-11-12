//! Pure chart maths: scales, downsampling, path data and number formatting.
//!
//! Marks are drawn in a fixed integer coordinate space and stretched to the
//! plot by the browser, so nothing here depends on the rendered size and a
//! resize costs no Rust at all.

use std::fmt::Write as _;

/// Width and height of the coordinate space marks are drawn in.
pub(super) const VW: i32 = 10_000;
pub(super) const VH: i32 = 1_000;

/// The value axis: a "nice" range that includes zero, and its ticks.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct Scale {
    pub lo: f64,
    pub hi: f64,
    pub ticks: Vec<f64>,
}

impl Scale {
    /// A scale covering every finite value in `series`.
    pub fn covering<'a>(series: impl Iterator<Item = &'a [f64]>) -> Self {
        let (mut min, mut max) = (0.0_f64, 0.0_f64);
        for value in series.flatten().copied().filter(|v| v.is_finite()) {
            min = min.min(value);
            max = max.max(value);
        }
        Self::nice(min, max)
    }

    /// Rounds `min..max` outwards to a 1-2-2.5-5 ladder with about four steps.
    pub fn nice(min: f64, max: f64) -> Self {
        let span = if max > min { max - min } else { 1.0 };
        let raw = span / 4.0;
        // The power of ten at or below `raw`, by repeated scaling: `powf` and
        // `log10` would link a large share of libm into a wasm build.
        let mut magnitude = 1.0_f64;
        while magnitude > raw && magnitude > 1e-300 {
            magnitude /= 10.0;
        }
        while magnitude * 10.0 <= raw && magnitude < 1e300 {
            magnitude *= 10.0;
        }
        let step = [1.0, 2.0, 2.5, 5.0, 10.0].into_iter().map(|m| m * magnitude).find(|s| *s >= raw).unwrap_or(raw);
        let lo = (min / step).floor() * step;
        let hi = ((max / step).ceil() * step).max(lo + step);
        let count = ((hi - lo) / step).round() as usize;
        Self { lo, hi, ticks: (0..=count).map(|i| lo + step * i as f64).collect() }
    }

    /// How far down the plot `value` sits: 0 at the top, 1 at the bottom.
    pub fn frac(&self, value: f64) -> f64 {
        ((self.hi - value) / (self.hi - self.lo)).clamp(0.0, 1.0)
    }

    fn y(&self, value: f64) -> i32 {
        (self.frac(value) * f64::from(VH)).round() as i32
    }
}

/// The category axis: `n` evenly spaced positions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Axis {
    pub n: usize,
    /// Positions are the centres of `n` bands (needed for bars) rather than
    /// running edge to edge.
    pub banded: bool,
}

impl Axis {
    /// How far along the plot position `i` sits, from 0 to 1.
    pub fn frac(&self, i: f64) -> f64 {
        if self.banded {
            (i + 0.5) / self.n as f64
        } else if self.n > 1 {
            i / (self.n - 1) as f64
        } else {
            0.5
        }
    }

    /// The position nearest to `frac` of the way along the plot.
    #[cfg_attr(not(any(target_arch = "wasm32", test)), allow(dead_code))]
    pub fn index(&self, frac: f64) -> usize {
        let last = self.n.saturating_sub(1);
        let i = if self.banded { (frac * self.n as f64).floor() } else { (frac * last as f64).round() };
        (i.max(0.0) as usize).min(last)
    }

    fn x(&self, i: f64) -> i32 {
        (self.frac(i) * f64::from(VW)).round() as i32
    }
}

/// Reduces `data` to at most two points per bucket, keeping each bucket's
/// minimum and maximum so spikes survive. `None` marks a gap (a run of
/// non-finite values). Short series pass through untouched.
pub(super) fn samples(data: &[f64], resolution: usize) -> Vec<Option<(usize, f64)>> {
    let resolution = resolution.max(1);
    if data.len() <= resolution * 2 {
        return data.iter().enumerate().map(|(i, v)| v.is_finite().then_some((i, *v))).collect();
    }

    let mut out = Vec::with_capacity(resolution * 2);
    for bucket in 0..resolution {
        let start = bucket * data.len() / resolution;
        let end = ((bucket + 1) * data.len() / resolution).max(start + 1);
        let mut low: Option<(usize, f64)> = None;
        let mut high: Option<(usize, f64)> = None;
        for (i, value) in data[start..end].iter().copied().enumerate().filter(|(_, v)| v.is_finite()) {
            let point = (start + i, value);
            if low.is_none_or(|(_, v)| value < v) {
                low = Some(point);
            }
            if high.is_none_or(|(_, v)| value > v) {
                high = Some(point);
            }
        }
        match (low, high) {
            (Some(a), Some(b)) if a.0 == b.0 => out.push(Some(a)),
            (Some(a), Some(b)) if a.0 < b.0 => out.extend([Some(a), Some(b)]),
            (Some(a), Some(b)) => out.extend([Some(b), Some(a)]),
            _ => out.push(None),
        }
    }
    out
}

/// SVG path data for one line series.
#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct LinePaths {
    pub line: String,
    /// The line closed down to the zero baseline.
    pub area: String,
    /// One zero-length segment per point, drawn as a dot by a round line cap.
    pub markers: String,
    /// Whether the series was reduced, so individual points are not shown.
    pub reduced: bool,
}

pub(super) fn line_paths(data: &[f64], axis: Axis, scale: &Scale, resolution: usize) -> LinePaths {
    let points = samples(data, resolution);
    let mut paths = LinePaths { reduced: points.len() != data.len(), ..LinePaths::default() };
    let base = scale.y(0.0);
    let mut previous: Option<(i32, i32)> = None;

    for point in points.into_iter().chain([None]) {
        match (point, previous) {
            (Some((i, value)), _) => {
                let (x, y) = (axis.x(i as f64), scale.y(value));
                if previous == Some((x, y)) {
                    continue;
                }
                if previous.is_none() {
                    let _ = write!(paths.line, "M{x} {y}");
                    let _ = write!(paths.area, "M{x} {base}L{x} {y}");
                } else {
                    let _ = write!(paths.line, "L{x} {y}");
                    let _ = write!(paths.area, "L{x} {y}");
                }
                if !paths.reduced {
                    let _ = write!(paths.markers, "M{x} {y}h0");
                }
                previous = Some((x, y));
            }
            (None, Some((x, _))) => {
                let _ = write!(paths.area, "L{x} {base}Z");
                previous = None;
            }
            (None, None) => {}
        }
    }
    paths
}

/// SVG path data for one bar series: a rectangle per position, or per bucket
/// (showing its largest magnitude) once there are more positions than
/// `resolution`.
pub(super) fn bar_path(data: &[f64], axis: Axis, scale: &Scale, resolution: usize) -> String {
    let resolution = resolution.max(1);
    let base = scale.y(0.0);
    let mut path = String::new();
    let mut bar = |centre: f64, band: f64, value: f64| {
        let y = scale.y(value);
        if !value.is_finite() || y == base {
            return;
        }
        let half = (band * 0.3 * f64::from(VW)).max(1.0);
        let centre = centre * f64::from(VW);
        let (left, right) = ((centre - half).round() as i32, (centre + half).round() as i32);
        let _ = write!(path, "M{left} {base}V{y}H{right}V{base}Z");
    };

    if data.len() <= resolution {
        let band = 1.0 / axis.n.max(1) as f64;
        for (i, value) in data.iter().enumerate() {
            bar(axis.frac(i as f64), band, *value);
        }
    } else {
        let share = data.len() as f64 / axis.n.max(1) as f64;
        for bucket in 0..resolution {
            let start = bucket * data.len() / resolution;
            let end = ((bucket + 1) * data.len() / resolution).max(start + 1);
            let peak = data[start..end].iter().copied().filter(|v| v.is_finite()).fold(0.0_f64, |peak, v| {
                if v.abs() > peak.abs() { v } else { peak }
            });
            bar((bucket as f64 + 0.5) / resolution as f64 * share, share / resolution as f64, peak);
        }
    }
    path
}

/// A number in full, with thousands separators and up to two decimals.
pub(super) fn format_number(value: f64) -> String {
    if !value.is_finite() {
        return "–".into();
    }
    let hundredths = (value.abs() * 100.0).round() as u64;
    let (whole, part) = (hundredths / 100, hundredths % 100);
    let digits = whole.to_string();
    let mut out = String::with_capacity(digits.len() + 8);
    if value < 0.0 && hundredths > 0 {
        out.push('-');
    }
    for (i, digit) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(digit);
    }
    match (part / 10, part % 10) {
        (0, 0) => {}
        (tenths, 0) => {
            let _ = write!(out, ".{tenths}");
        }
        _ => {
            let _ = write!(out, ".{part:02}");
        }
    }
    out
}

/// A short number for axis ticks: `950`, `12.5k`, `3M`, `1.2B`.
pub(super) fn format_compact(value: f64) -> String {
    let magnitude = value.abs();
    let (divisor, suffix) = if magnitude >= 1e9 {
        (1e9, "B")
    } else if magnitude >= 1e6 {
        (1e6, "M")
    } else if magnitude >= 1e3 {
        (1e3, "k")
    } else {
        return format_number(value);
    };
    let tenths = (value / divisor * 10.0).round() / 10.0;
    format!("{}{suffix}", format_number(tenths))
}

/// A CSS percentage for `frac` (0 to 1), to two decimals.
pub(super) fn percent(frac: f64) -> String {
    let hundredths = (frac.clamp(0.0, 1.0) * 10_000.0).round() as u32;
    format!("{}.{:02}%", hundredths / 100, hundredths % 100)
}
