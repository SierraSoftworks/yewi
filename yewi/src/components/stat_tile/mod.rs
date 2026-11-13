//! A headline figure: label, value, and optionally a change and a detail line.

use yew::prelude::*;

use crate::{Attrs, Icon, IconData, Size, Tone, VisuallyHidden, icons, is_empty};

/// How a stat tile sits on the page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum StatTileVariant {
    /// A lifted card of its own, for a row of headline figures.
    #[default]
    Card,
    /// No surface or padding, for figures inside a card or panel.
    Plain,
}

impl StatTileVariant {
    pub const ALL: [StatTileVariant; 2] = [StatTileVariant::Card, StatTileVariant::Plain];

    const fn as_str(self) -> &'static str {
        match self {
            StatTileVariant::Card => "card",
            StatTileVariant::Plain => "plain",
        }
    }
}

/// The direction a figure moved since the comparison period.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatTrend {
    Up,
    Down,
    /// No meaningful change.
    Flat,
}

impl StatTrend {
    pub const ALL: [StatTrend; 3] = [StatTrend::Up, StatTrend::Down, StatTrend::Flat];

    /// The trend of a signed change; changes smaller than `tolerance` are flat.
    pub fn of(change: f64, tolerance: f64) -> Self {
        if change.abs() < tolerance || change.is_nan() {
            StatTrend::Flat
        } else if change > 0.0 {
            StatTrend::Up
        } else {
            StatTrend::Down
        }
    }

    const fn label(self) -> &'static str {
        match self {
            StatTrend::Up => "Up",
            StatTrend::Down => "Down",
            StatTrend::Flat => "No change",
        }
    }

    const fn icon(self) -> IconData {
        match self {
            StatTrend::Up => icons::ARROW_UP,
            StatTrend::Down => icons::ARROW_DOWN,
            StatTrend::Flat => icons::MINUS,
        }
    }

    /// Whether the movement is good news, bad news or neither.
    const fn tone(self, lower_is_better: bool) -> Tone {
        match (self, lower_is_better) {
            (StatTrend::Flat, _) => Tone::Neutral,
            (StatTrend::Up, false) | (StatTrend::Down, true) => Tone::Ok,
            (StatTrend::Up, true) | (StatTrend::Down, false) => Tone::Danger,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct StatTileProps {
    /// What the figure measures.
    pub label: AttrValue,
    /// The figure, already formatted for display. Use an en dash or similar
    /// when there is no data.
    pub value: AttrValue,
    /// A unit shown small after the value, such as `ms` or `%`.
    #[prop_or_default]
    pub unit: Option<AttrValue>,
    /// An icon shown before the label.
    #[prop_or_default]
    pub icon: Option<IconData>,
    /// The change since the comparison period, already formatted, such as
    /// `12%`. Shown with an arrow for `trend`.
    #[prop_or_default]
    pub delta: Option<AttrValue>,
    /// The direction of `delta`. `Up` reads as good and `Down` as bad unless
    /// `lower_is_better` is set.
    #[prop_or_default]
    pub trend: Option<StatTrend>,
    /// Flips the colouring of `trend` for figures where a fall is good news,
    /// such as error or bounce rates.
    #[prop_or_default]
    pub lower_is_better: bool,
    /// A short line of context under the value, such as the comparison
    /// period or what the figure means.
    #[prop_or_default]
    pub detail: Option<AttrValue>,
    /// Colours the value to carry a status. Leave `Neutral` unless the
    /// figure itself is the alert.
    #[prop_or_default]
    pub tone: Tone,
    #[prop_or_default]
    pub variant: StatTileVariant,
    /// The size of the value.
    #[prop_or_default]
    pub size: Size,
    /// Makes the tile a toggle button, for tiles that choose what a chart or
    /// table below them shows.
    #[prop_or_default]
    pub on_click: Option<Callback<MouseEvent>>,
    /// Marks the tile as the chosen one. Only used with `on_click`.
    #[prop_or_default]
    pub selected: bool,
    /// Makes the tile a link to where the figure is explained.
    #[prop_or_default]
    pub href: Option<AttrValue>,
    /// Extra content under the figure, such as a sparkline. Must be phrasing
    /// content when the tile is a button or link.
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// One headline figure with its label, and optionally its change since a
/// comparison period. Lay several out with `Grid` or `Cluster`.
#[function_component]
pub fn StatTile(props: &StatTileProps) -> Html {
    let interactive = props.on_click.is_some() || props.href.is_some();
    let selected = props.selected && props.on_click.is_some();

    let class = classes!(
        "yi-stat-tile",
        format!("yi-stat-tile--{}", props.variant.as_str()),
        format!("yi-stat-tile--{}", props.size.as_str()),
        (props.tone != Tone::Neutral).then(|| format!("yi-stat-tile--{}", props.tone.as_str())),
        interactive.then_some("yi-stat-tile--interactive"),
        selected.then_some("yi-stat-tile--selected"),
        props.class.clone(),
    );

    let delta = props.delta.as_ref().map(|delta| {
        let trend = props.trend.unwrap_or(StatTrend::Flat);
        let tone = trend.tone(props.lower_is_better);
        html! {
            <span class={classes!("yi-stat-tile__delta", format!("yi-stat-tile__delta--{}", tone.as_str()))}>
                <Icon icon={trend.icon()} />
                <VisuallyHidden>{ trend.label() }{ " " }</VisuallyHidden>
                { delta.clone() }
            </span>
        }
    });

    let content = html! {
        <>
            <span class="yi-stat-tile__label">
                if let Some(icon) = props.icon {
                    <Icon {icon} />
                }
                { props.label.clone() }
            </span>
            <span class="yi-stat-tile__value">
                { props.value.clone() }
                if let Some(unit) = &props.unit {
                    <span class="yi-stat-tile__unit">{ unit.clone() }</span>
                }
            </span>
            if delta.is_some() || props.detail.is_some() {
                <span class="yi-stat-tile__foot">
                    { delta }
                    if let Some(detail) = &props.detail {
                        <span class="yi-stat-tile__detail">{ detail.clone() }</span>
                    }
                </span>
            }
            if !is_empty(&props.children) {
                <span class="yi-stat-tile__extra">{ props.children.clone() }</span>
            }
        </>
    };

    let id = props.id.clone();
    props.attrs.apply(match (&props.on_click, &props.href) {
        (Some(on_click), _) => html! {
            <button {id} {class} type="button" aria-pressed={if selected { "true" } else { "false" }} onclick={on_click.clone()}>
                { content }
            </button>
        },
        (None, Some(href)) => html! { <a {id} {class} href={href.clone()}>{ content }</a> },
        (None, None) => html! { <div {id} {class}>{ content }</div> },
    })
}

#[cfg(test)]
mod tests;
