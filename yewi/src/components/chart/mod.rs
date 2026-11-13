#![cfg(feature = "charts")]
//! Charts: a time-series [`Chart`] and an inline [`Sparkline`].
//!
//! Both are plain SVG and CSS with no charting dependency, and are built to
//! stay cheap on large datasets:
//!
//! - Marks are drawn in a fixed coordinate space that the browser stretches
//!   to fit, so a chart renders fully on the server and a resize runs no code.
//! - A long series is reduced to a minimum and a maximum per horizontal
//!   bucket (see `resolution`), so path size is bounded however much data
//!   there is, and spikes survive.
//! - Geometry is computed once per dataset. [`SeriesData`] compares by
//!   pointer, so hovering, or a parent re-rendering with the same data, never
//!   rebuilds or re-compares the points.

use std::{ops::Deref, rc::Rc};

use yew::prelude::*;

use crate::{Attrs, Tone, use_id};

mod geometry;

use geometry::{Axis, Scale, VH, VW, bar_path, format_compact, format_number, line_paths, percent};

/// The values of one series, shared without copying.
///
/// Equality is by pointer, not content: two `SeriesData` are equal only when
/// they are clones of each other. Create it once per dataset (in a
/// `use_memo`, or wherever the data is fetched) and clone it into the chart;
/// replace it to update the chart. A non-finite value (`f64::NAN`) is a gap.
#[derive(Debug, Clone)]
pub struct SeriesData(Rc<[f64]>);

impl PartialEq for SeriesData {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Deref for SeriesData {
    type Target = [f64];

    fn deref(&self) -> &[f64] {
        &self.0
    }
}

impl Default for SeriesData {
    fn default() -> Self {
        Self(Rc::from([]))
    }
}

impl From<Vec<f64>> for SeriesData {
    fn from(values: Vec<f64>) -> Self {
        Self(values.into())
    }
}

impl From<Rc<[f64]>> for SeriesData {
    fn from(values: Rc<[f64]>) -> Self {
        Self(values)
    }
}

impl From<&[f64]> for SeriesData {
    fn from(values: &[f64]) -> Self {
        Self(values.into())
    }
}

impl<const N: usize> From<[f64; N]> for SeriesData {
    fn from(values: [f64; N]) -> Self {
        Self(Rc::new(values))
    }
}

impl FromIterator<f64> for SeriesData {
    fn from_iter<I: IntoIterator<Item = f64>>(values: I) -> Self {
        Self(values.into_iter().collect())
    }
}

/// How a series is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SeriesKind {
    /// A line over a soft fill down to zero. The default for the first series.
    Area,
    /// A line. The default for every other series.
    Line,
    /// A bar per position, for counts that are better read one by one.
    Bars,
}

/// The colour of a series. Colour identifies a series, so give a series the
/// same colour wherever it appears.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum SeriesColor {
    /// The next palette slot, in series order: the accent first.
    #[default]
    Auto,
    /// A fixed palette slot, 1 to 6 (`--yi-chart-series-1` and so on), so a
    /// series keeps its colour when others are filtered out.
    Slot(u8),
    /// A recessive grey, for context such as a previous period or a target.
    Muted,
    /// A status colour, only for a series that means that status (errors).
    Tone(Tone),
}

/// The stroke pattern of a line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum SeriesDash {
    #[default]
    Solid,
    Dashed,
    Dotted,
}

impl SeriesDash {
    const fn modifier(self) -> Option<&'static str> {
        match self {
            SeriesDash::Solid => None,
            SeriesDash::Dashed => Some("yi-chart--dashed"),
            SeriesDash::Dotted => Some("yi-chart--dotted"),
        }
    }
}

/// One named series of a [`Chart`]. All series share the same positions:
/// value `i` of each belongs to position `i`.
///
/// ```ignore
/// Series::new("Previous period", previous).color(SeriesColor::Muted).dash(SeriesDash::Dashed)
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Series {
    /// Shown in the legend and the tooltip.
    pub name: AttrValue,
    pub data: SeriesData,
    /// `None` draws the first series as an area and the rest as lines.
    pub kind: Option<SeriesKind>,
    pub color: SeriesColor,
    pub dash: SeriesDash,
    /// Draw a dot on every point. Ignored once the series is long enough to
    /// be reduced.
    pub markers: bool,
}

impl Series {
    pub fn new(name: impl Into<AttrValue>, data: impl Into<SeriesData>) -> Self {
        Self {
            name: name.into(),
            data: data.into(),
            kind: None,
            color: SeriesColor::Auto,
            dash: SeriesDash::Solid,
            markers: false,
        }
    }

    pub fn kind(mut self, kind: SeriesKind) -> Self {
        self.kind = Some(kind);
        self
    }

    pub fn color(mut self, color: SeriesColor) -> Self {
        self.color = color;
        self
    }

    pub fn dash(mut self, dash: SeriesDash) -> Self {
        self.dash = dash;
        self
    }

    pub fn markers(mut self, markers: bool) -> Self {
        self.markers = markers;
        self
    }
}

/// The number of palette slots (`--yi-chart-series-N`).
const SLOTS: u8 = 6;

/// The class that sets `--yi-chart-color` for a series.
fn color_class(color: SeriesColor, auto_slot: u8) -> String {
    let name = match color {
        // Past the palette there is no further distinguishable hue.
        SeriesColor::Auto if auto_slot > SLOTS => "muted".into(),
        SeriesColor::Auto => auto_slot.to_string(),
        SeriesColor::Slot(slot) => slot.clamp(1, SLOTS).to_string(),
        SeriesColor::Muted => "muted".into(),
        SeriesColor::Tone(tone) => tone.as_str().into(),
    };
    format!("yi-chart__c--{name}")
}

/// Everything about a chart that depends only on its data.
struct Geometry {
    axis: Axis,
    scale: Scale,
    marks: Vec<Marks>,
}

struct Marks {
    kind: SeriesKind,
    /// Colour and dash classes.
    class: Classes,
    line: AttrValue,
    area: AttrValue,
    markers: Option<AttrValue>,
}

impl Geometry {
    fn new(series: &[Series], resolution: usize, banded: Option<bool>, whole: bool) -> Self {
        let kinds: Vec<SeriesKind> = series
            .iter()
            .enumerate()
            .map(|(i, s)| s.kind.unwrap_or(if i == 0 { SeriesKind::Area } else { SeriesKind::Line }))
            .collect();
        let axis = Axis {
            n: series.iter().map(|s| s.data.len()).max().unwrap_or(0),
            banded: banded.unwrap_or_else(|| kinds.contains(&SeriesKind::Bars)),
        };
        let scale = Scale::covering(series.iter().map(|s| &*s.data), whole);

        let mut auto_slot = 0;
        let marks = series
            .iter()
            .zip(kinds)
            .map(|(series, kind)| {
                if series.color == SeriesColor::Auto {
                    auto_slot += 1;
                }
                let class = classes!(color_class(series.color, auto_slot), series.dash.modifier());
                if kind == SeriesKind::Bars {
                    let area = bar_path(&series.data, axis, &scale, resolution).into();
                    return Marks { kind, class, line: AttrValue::default(), area, markers: None };
                }
                let paths = line_paths(&series.data, axis, &scale, resolution);
                Marks {
                    kind,
                    class,
                    line: paths.line.into(),
                    area: paths.area.into(),
                    markers: (series.markers && !paths.reduced).then(|| paths.markers.into()),
                }
            })
            .collect();

        Self { axis, scale, marks }
    }
}

#[derive(Properties)]
struct MarksProps {
    geometry: Rc<Geometry>,
    id: AttrValue,
}

impl PartialEq for MarksProps {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.geometry, &other.geometry) && self.id == other.id
    }
}

/// The SVG layer. A component of its own so that hovering, which re-renders
/// the chart, does not diff the path data.
#[function_component]
fn ChartMarks(props: &MarksProps) -> Html {
    let Geometry { scale, marks, .. } = &*props.geometry;

    let mut grid = String::new();
    for tick in scale.ticks.iter().filter(|tick| **tick != 0.0) {
        grid.push_str(&format!("M0 {y}H{VW}", y = (scale.frac(*tick) * f64::from(VH)).round() as i32));
    }
    let zero = format!("M0 {y}H{VW}", y = (scale.frac(0.0) * f64::from(VH)).round() as i32);

    // Fills first, so no line is hidden behind another series' area.
    let fills = marks.iter().enumerate().map(|(i, marks)| match marks.kind {
        SeriesKind::Line => html! {},
        SeriesKind::Bars => html! {
            <g class={marks.class.clone()}><path class="yi-chart__bars" d={marks.area.clone()} /></g>
        },
        SeriesKind::Area => {
            let gradient = format!("{}-{i}", props.id);
            html! {
                <g class={marks.class.clone()}>
                    <linearGradient id={gradient.clone()} x1="0" y1="0" x2="0" y2="1">
                        <stop class="yi-chart__fade-from" offset="0" />
                        <stop class="yi-chart__fade-to" offset="1" />
                    </linearGradient>
                    <path class="yi-chart__area" d={marks.area.clone()} fill={format!("url(#{gradient})")} />
                </g>
            }
        }
    });
    let lines = marks.iter().enumerate().filter(|(_, marks)| marks.kind != SeriesKind::Bars).map(|(_, marks)| {
        html! {
            <g class={marks.class.clone()}>
                <path class="yi-chart__line" d={marks.line.clone()} />
                if let Some(markers) = &marks.markers {
                    <path class="yi-chart__marker-ring" d={markers.clone()} />
                    <path class="yi-chart__marker" d={markers.clone()} />
                }
            </g>
        }
    });

    html! {
        <svg
            class="yi-chart__marks"
            viewBox={format!("0 0 {VW} {VH}")}
            preserveAspectRatio="none"
            aria-hidden="true"
            focusable="false"
        >
            <path class="yi-chart__grid" d={grid} />
            <g>{ for fills }</g>
            <path class="yi-chart__zero" d={zero} />
            <g>{ for lines }</g>
        </svg>
    }
}

#[derive(Properties, PartialEq)]
pub struct ChartProps {
    /// The series to draw, primary first. Each holds its values as
    /// `SeriesData`; build the list once per dataset rather than per render.
    pub series: Vec<Series>,
    /// What the chart shows, as its accessible name: "Visitors per day".
    pub label: AttrValue,
    /// The text for position `i`, used for the axis and, unless `x_tooltip`
    /// is set, the tooltip. Defaults to the position number, counting from 1.
    #[prop_or_default]
    pub x_label: Option<Callback<usize, AttrValue>>,
    /// Formats a value for the axis and, unless `y_tooltip` is set, the
    /// tooltip. By default the axis is compact (`12.5k`) and the tooltip
    /// shows the number in full.
    #[prop_or_default]
    pub y_label: Option<Callback<f64, AttrValue>>,
    /// The text for position `i` in the tooltip, where there is room to say
    /// more than the axis does: "6 Nov 2026, 10:00" over a tick of "6 Nov".
    #[prop_or_default]
    pub x_tooltip: Option<Callback<usize, AttrValue>>,
    /// Formats a value for the tooltip only.
    #[prop_or_default]
    pub y_tooltip: Option<Callback<f64, AttrValue>>,
    /// The most labels to show along the horizontal axis. Narrow containers
    /// show every other one. Zero removes the axis, for a chart stacked on
    /// another that labels the same positions.
    #[prop_or(8)]
    pub x_ticks: usize,
    /// Keeps the value axis to whole numbers, for counts: a small maximum
    /// otherwise gets ticks such as 0.5.
    #[prop_or_default]
    pub integer_ticks: bool,
    /// Shows the crosshair and tooltip at this position while the chart is
    /// not being hovered itself. Feed it the position another chart reports
    /// through `on_hover` to keep stacked charts in step.
    #[prop_or_default]
    pub cursor: Option<usize>,
    /// Whether to list the series above the plot. Defaults to showing the
    /// legend when there are two or more series.
    #[prop_or_default]
    pub legend: Option<bool>,
    /// Whether to show the crosshair and tooltip and take keyboard focus.
    /// Turn off for a purely illustrative chart.
    #[prop_or(true)]
    pub interactive: bool,
    /// Whether a range can be chosen by dragging, or with Shift and the
    /// arrow keys then Enter. The range is reported through `on_select`.
    #[prop_or_default]
    pub selectable: bool,
    /// The number of horizontal buckets a long series is reduced to. Series
    /// with up to twice this many points are drawn in full.
    #[prop_or(1000)]
    pub resolution: usize,
    /// Whether positions are the centres of equal bands rather than running
    /// edge to edge. Defaults to bands when any series is drawn as bars. Set
    /// it on a line chart stacked above a bar chart so their positions align.
    #[prop_or_default]
    pub banded: Option<bool>,
    /// Shown instead of the plot when no series has any points.
    #[prop_or(AttrValue::Static("No data"))]
    pub empty: AttrValue,
    /// Fired with the position under the pointer or keyboard cursor, and
    /// with `None` when it leaves.
    #[prop_or_default]
    pub on_hover: Callback<Option<usize>>,
    /// Fired with the first and last position of a completed selection.
    #[prop_or_default]
    pub on_select: Callback<(usize, usize)>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A chart of one or more series over shared, evenly spaced positions
/// (usually time buckets). The first series is drawn as an area and the rest
/// as lines; each `Series` can override its kind, colour, dash and markers.
///
/// Set the height with `--yi-chart-height`; the width follows the container.
#[function_component]
pub fn Chart(props: &ChartProps) -> Html {
    let uid = use_id();
    let cursor = use_state_eq(|| None::<usize>);
    // `(anchor, current)` of a selection in progress.
    let drag = use_state_eq(|| None::<(usize, usize)>);
    let geometry = use_memo(
        (props.series.clone(), props.resolution, props.banded, props.integer_ticks),
        |(series, resolution, banded, whole)| Geometry::new(series, *resolution, *banded, *whole),
    );

    let class = classes!("yi-chart", props.class.clone());
    let Geometry { axis, scale, .. } = &*geometry;
    let (axis, n) = (*axis, axis.n);

    if n == 0 {
        return props.attrs.apply(html! {
            <div id={props.id.clone()} {class} role="group" aria-label={props.label.clone()}>
                <div class="yi-chart__empty">{ props.empty.clone() }</div>
            </div>
        });
    }

    let last = n - 1;
    // State can outlive the data it was set for, so clamp rather than trust it.
    let hovered = (*cursor).or(props.cursor).map(|i| i.min(last));
    let selection = (*drag).map(|(a, b)| (a.min(b).min(last), a.max(b).min(last)));

    let x_text = |i: usize| match &props.x_label {
        Some(label) => label.emit(i),
        None => AttrValue::from((i + 1).to_string()),
    };
    let compact = scale.hi.abs().max(scale.lo.abs()) >= 10_000.0;
    let tick_text = |value: f64| match &props.y_label {
        Some(label) => label.emit(value),
        None if compact => format_compact(value).into(),
        None => format_number(value).into(),
    };
    let value_text = |value: f64| match props.y_tooltip.as_ref().or(props.y_label.as_ref()) {
        Some(label) if value.is_finite() => label.emit(value),
        _ => AttrValue::from(format_number(value)),
    };
    let title_text = |i: usize| match &props.x_tooltip {
        Some(label) => label.emit(i),
        None => x_text(i),
    };

    // ---- Axes and legend ---------------------------------------------------
    let y_ticks = scale.ticks.iter().map(|tick| {
        html! {
            <span class="yi-chart__tick" style={format!("inset-block-start:{}", percent(scale.frac(*tick)))}>
                { tick_text(*tick) }
            </span>
        }
    });

    let step = n.div_ceil(props.x_ticks.max(1));
    let mut previous_label = None;
    let x_ticks = (0..if props.x_ticks == 0 { 0 } else { n })
        .step_by(step)
        .filter_map(|i| {
            let text = x_text(i);
            if previous_label.as_ref() == Some(&text) {
                return None;
            }
            previous_label = Some(text.clone());
            let at = axis.frac(i as f64);
            let edge = if at < 0.04 {
                Some("yi-chart__tick--start")
            } else if at > 0.96 {
                Some("yi-chart__tick--end")
            } else {
                None
            };
            Some(html! {
                <span class={classes!("yi-chart__tick", edge)} style={format!("inset-inline-start:{}", percent(at))}>
                    { text }
                </span>
            })
        })
        .collect::<Html>();

    let key = |index: usize| {
        let marks = &geometry.marks[index];
        let shape = (marks.kind == SeriesKind::Bars).then_some("yi-chart__key--block");
        html! { <span class={classes!("yi-chart__key", shape, marks.class.clone())} /> }
    };
    let legend = props.legend.unwrap_or(props.series.len() > 1).then(|| {
        html! {
            <ul class="yi-chart__legend" aria-label="Series">
                { for props.series.iter().enumerate().map(|(i, series)| html! {
                    <li class="yi-chart__legend-item">{ key(i) }{ series.name.clone() }</li>
                }) }
            </ul>
        }
    });

    // ---- Cursor: crosshair, point dots and tooltip ---------------------------
    let describe = |i: usize| {
        let values: Vec<String> = props
            .series
            .iter()
            .filter_map(|s| s.data.get(i).map(|value| format!("{} {}", s.name, value_text(*value))))
            .collect();
        format!("{}: {}", title_text(i), values.join(", "))
    };

    let overlay = hovered.filter(|_| props.interactive).map(|i| {
        let at = axis.frac(i as f64);
        let dots = props.series.iter().enumerate().filter_map(|(s, series)| {
            let value = series.data.get(i).copied().filter(|v| v.is_finite())?;
            (geometry.marks[s].kind != SeriesKind::Bars).then(|| {
                html! {
                    <span
                        class={classes!("yi-chart__dot", geometry.marks[s].class.clone())}
                        style={format!("inset-inline-start:{};inset-block-start:{}", percent(at), percent(scale.frac(value)))}
                    />
                }
            })
        });
        let rows = props.series.iter().enumerate().filter_map(|(s, series)| {
            let value = series.data.get(i)?;
            Some(html! {
                <div class="yi-chart__tooltip-row">
                    { key(s) }
                    <span>{ series.name.clone() }</span>
                    <strong>{ value_text(*value) }</strong>
                </div>
            })
        });
        let (side, place) = if at > 0.5 {
            ("yi-chart__tooltip--before", format!("inset-inline-end:{}", percent(1.0 - at)))
        } else {
            ("yi-chart__tooltip--after", format!("inset-inline-start:{}", percent(at)))
        };
        html! {
            <>
                <span class="yi-chart__crosshair" style={format!("inset-inline-start:{}", percent(at))} />
                { for dots }
                <div class={classes!("yi-chart__tooltip", side)} style={place}>
                    <div class="yi-chart__tooltip-title">{ title_text(i) }</div>
                    { for rows }
                </div>
            </>
        }
    });

    let selected = selection.filter(|(lo, hi)| hi > lo).map(|(lo, hi)| {
        let (from, to) = (axis.frac(lo as f64), axis.frac(hi as f64));
        html! {
            <span
                class="yi-chart__selection"
                style={format!("inset-inline-start:{};inline-size:{}", percent(from), percent(to - from))}
            />
        }
    });

    // ---- Interaction -------------------------------------------------------
    let move_cursor = {
        let (cursor, on_hover) = (cursor.clone(), props.on_hover.clone());
        Callback::from(move |next: Option<usize>| {
            if *cursor != next {
                cursor.set(next);
                on_hover.emit(next);
            }
        })
    };
    let finish = {
        let (drag, on_select) = (drag.clone(), props.on_select.clone());
        Callback::from(move |()| {
            if let Some((a, b)) = *drag
                && a != b
            {
                on_select.emit((a.min(b).min(last), a.max(b).min(last)));
            }
            drag.set(None);
        })
    };

    let onpointermove = {
        let (drag, move_cursor) = (drag.clone(), move_cursor.clone());
        Callback::from(move |event: PointerEvent| {
            let Some(i) = pointer_index(&event, axis) else { return };
            move_cursor.emit(Some(i));
            if let Some((anchor, _)) = *drag
                && event.buttons() & 1 == 1
            {
                drag.set(Some((anchor, i)));
            }
        })
    };
    let onpointerdown = {
        let (drag, move_cursor, selectable) = (drag.clone(), move_cursor.clone(), props.selectable);
        Callback::from(move |event: PointerEvent| {
            let Some(i) = pointer_index(&event, axis) else { return };
            move_cursor.emit(Some(i));
            if selectable && event.button() == 0 {
                drag.set(Some((i, i)));
            }
        })
    };
    let onpointerup = {
        let finish = finish.clone();
        Callback::from(move |_: PointerEvent| finish.emit(()))
    };
    let onpointerleave = {
        let (drag, move_cursor) = (drag.clone(), move_cursor.clone());
        Callback::from(move |_: PointerEvent| {
            move_cursor.emit(None);
            drag.set(None);
        })
    };
    let onfocus = {
        let move_cursor = move_cursor.clone();
        Callback::from(move |_: FocusEvent| {
            if hovered.is_none() {
                move_cursor.emit(Some(last));
            }
        })
    };
    let onblur = {
        let (drag, move_cursor) = (drag.clone(), move_cursor.clone());
        Callback::from(move |_: FocusEvent| {
            move_cursor.emit(None);
            drag.set(None);
        })
    };
    let onkeydown = {
        let (drag, selectable) = (drag.clone(), props.selectable);
        Callback::from(move |event: KeyboardEvent| {
            let current = hovered.unwrap_or(last);
            let page = (n / 10).max(1);
            let next = match event.key().as_str() {
                "ArrowLeft" | "ArrowDown" => current.saturating_sub(1),
                "ArrowRight" | "ArrowUp" => (current + 1).min(last),
                "PageDown" => current.saturating_sub(page),
                "PageUp" => (current + page).min(last),
                "Home" => 0,
                "End" => last,
                "Enter" if drag.is_some() => {
                    event.prevent_default();
                    return finish.emit(());
                }
                "Escape" => return drag.set(None),
                _ => return,
            };
            event.prevent_default();
            move_cursor.emit(Some(next));
            if selectable && event.shift_key() {
                let anchor = (*drag).map_or(current, |(anchor, _)| anchor);
                drag.set(Some((anchor, next)));
            } else {
                drag.set(None);
            }
        })
    };

    let plot_class = classes!("yi-chart__plot", props.selectable.then_some("yi-chart__plot--selectable"));
    let layers = html! {
        <>
            <ChartMarks geometry={geometry.clone()} id={uid} />
            { selected }
            { overlay }
        </>
    };
    // An interactive plot is a slider over its positions: arrow keys move the
    // cursor and assistive technology reads the values under it.
    let plot = if props.interactive {
        html! {
            <div
                class={plot_class}
                role="slider"
                tabindex="0"
                aria-label={props.label.clone()}
                aria-valuemin="0"
                aria-valuemax={last.to_string()}
                aria-valuenow={hovered.unwrap_or(last).to_string()}
                aria-valuetext={describe(hovered.unwrap_or(last))}
                {onpointermove}
                {onpointerdown}
                {onpointerup}
                {onpointerleave}
                {onfocus}
                {onblur}
                {onkeydown}
            >
                { layers }
            </div>
        }
    } else {
        html! { <div class={plot_class} role="img" aria-label={props.label.clone()}>{ layers }</div> }
    };

    props.attrs.apply(html! {
        <div id={props.id.clone()} {class}>
            { legend }
            <div class="yi-chart__body">
                <div class="yi-chart__y" aria-hidden="true">{ for y_ticks }</div>
                { plot }
                if props.x_ticks > 0 {
                    <div class="yi-chart__x" aria-hidden="true">{ x_ticks }</div>
                }
            </div>
        </div>
    })
}

/// The position under a pointer event. Every layer inside the plot ignores
/// the pointer, so the event's offset is always relative to the plot itself.
#[cfg(target_arch = "wasm32")]
fn pointer_index(event: &PointerEvent, axis: Axis) -> Option<usize> {
    let plot = event.target_dyn_into::<web_sys::Element>()?;
    let width = f64::from(plot.client_width());
    (width > 0.0).then(|| axis.index(f64::from(event.offset_x()) / width))
}

#[cfg(not(target_arch = "wasm32"))]
fn pointer_index(_event: &PointerEvent, _axis: Axis) -> Option<usize> {
    None
}

#[derive(Properties, PartialEq)]
pub struct SparklineProps {
    /// The values, oldest first.
    pub data: SeriesData,
    #[prop_or_default]
    pub color: SeriesColor,
    /// Fill under the line.
    #[prop_or(true)]
    pub area: bool,
    /// What the trend shows, for assistive technology: "Errors, last 24
    /// hours, rising". Without it the sparkline is decorative and hidden.
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// The number of horizontal buckets a long series is reduced to.
    #[prop_or(100)]
    pub resolution: usize,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A small trend line with no axes, for table rows, tiles and headings. It
/// fills the width of its container; size it with `--yi-sparkline-height`
/// and the container's width.
#[function_component]
pub fn Sparkline(props: &SparklineProps) -> Html {
    let paths = use_memo((props.data.clone(), props.resolution), |(data, resolution)| {
        let axis = Axis { n: data.len(), banded: false };
        let (min, max) = data
            .iter()
            .filter(|v| v.is_finite())
            .fold((0.0_f64, 0.0_f64), |(min, max), v| (min.min(*v), max.max(*v)));
        // An exact range, not a rounded one: a sparkline has no axis to read.
        let scale = Scale { lo: min, hi: if max > min { max } else { min + 1.0 }, ticks: Vec::new() };
        let paths = line_paths(data, axis, &scale, *resolution);
        (AttrValue::from(paths.line), AttrValue::from(paths.area))
    });

    let class = classes!("yi-sparkline", color_class(props.color, 1), props.class.clone());
    props.attrs.apply(html! {
        <svg
            id={props.id.clone()}
            {class}
            viewBox={format!("0 0 {VW} {VH}")}
            preserveAspectRatio="none"
            focusable="false"
            role={props.label.as_ref().map(|_| "img")}
            aria-label={props.label.clone()}
            aria-hidden={props.label.is_none().then_some("true")}
        >
            if props.area {
                <path class="yi-sparkline__area" d={paths.1.clone()} />
            }
            <path class="yi-sparkline__line" d={paths.0.clone()} />
        </svg>
    })
}

#[cfg(test)]
mod tests;
