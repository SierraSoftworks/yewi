//! Pipeline: ordered stages side by side, an optional gate in front of each,
//! and the stage's own content inside it.
//!
//! [`Pipeline`] holds [`PipelineStage`]s. A stage takes a [`PipelineGate`]
//! through its `gate` prop and any content as children; [`PipelineUnits`] and
//! [`PipelineUnit`] draw one small block per unit of work in a stage.

use yew::prelude::*;

use crate::{Attrs, Icon, IconData, Tone, VisuallyHidden, icons};

#[derive(Properties, PartialEq)]
pub struct PipelineProps {
    /// The [`PipelineStage`]s, in order.
    #[prop_or_default]
    pub children: Html,
    /// Accessible name of the list of stages, e.g. "Rollout stages".
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// Smaller stages and units and no stage `meta`, for an overview.
    #[prop_or_default]
    pub compact: bool,
    /// Stages share the available width equally instead of sizing to their
    /// content.
    #[prop_or_default]
    pub fill: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// Stages in a row that wraps, and stacks when the container is narrow.
#[function_component]
pub fn Pipeline(props: &PipelineProps) -> Html {
    let class = classes!(
        "yi-pipeline",
        props.compact.then_some("yi-pipeline--compact"),
        props.fill.then_some("yi-pipeline--fill"),
        props.class.clone(),
    );
    props.attrs.apply(html! {
        <div id={props.id.clone()} {class}>
            <ol class="yi-pipeline__track" aria-label={props.label.clone()}>
                { props.children.clone() }
            </ol>
        </div>
    })
}

#[derive(Properties, PartialEq)]
pub struct PipelineStageProps {
    /// The stage's name.
    pub title: AttrValue,
    /// A short status beside the title: a pill, a count.
    #[prop_or_default]
    pub status: Html,
    /// Quiet supporting detail under the title. Hidden in a compact pipeline.
    #[prop_or_default]
    pub meta: Html,
    /// The stage's content.
    #[prop_or_default]
    pub children: Html,
    /// The [`PipelineGate`] that must open before this stage starts. Without
    /// one, a plain connector joins the stage to the one before it.
    #[prop_or_default]
    pub gate: Html,
    /// The stage the pipeline is working on now. Marked with
    /// `aria-current="step"`.
    #[prop_or_default]
    pub current: bool,
    /// A stage that is not part of the run itself, such as "everything
    /// else": drawn as a dashed outline.
    #[prop_or_default]
    pub placeholder: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// One stage of a [`Pipeline`].
#[function_component]
pub fn PipelineStage(props: &PipelineStageProps) -> Html {
    let empty = Html::default();
    let class = classes!(
        "yi-pipeline__step",
        props.current.then_some("yi-pipeline__step--current"),
        props.placeholder.then_some("yi-pipeline__step--placeholder"),
        props.class.clone(),
    );
    props.attrs.apply(html! {
        <li id={props.id.clone()} {class} aria-current={props.current.then_some("step")}>
            if props.gate == empty {
                <span class="yi-pipeline__link" aria-hidden="true"><Icon icon={icons::CHEVRON_RIGHT} /></span>
            } else {
                { props.gate.clone() }
            }
            <div class="yi-pipeline-stage">
                <div class="yi-pipeline-stage__head">
                    <div class="yi-pipeline-stage__title-row">
                        <span class="yi-pipeline-stage__title">{ props.title.clone() }</span>
                        if props.status != empty {
                            <span class="yi-pipeline-stage__status">{ props.status.clone() }</span>
                        }
                    </div>
                    if props.meta != empty {
                        <div class="yi-pipeline-stage__meta">{ props.meta.clone() }</div>
                    }
                </div>
                { props.children.clone() }
            </div>
        </li>
    })
}

/// Whether a gate lets the pipeline through.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum GateState {
    /// Not reached yet.
    #[default]
    Waiting,
    /// Reached and shut: the pipeline is held here.
    Closed,
    /// Passed.
    Open,
}

impl GateState {
    pub const ALL: [GateState; 3] = [GateState::Waiting, GateState::Closed, GateState::Open];

    const fn as_str(self) -> &'static str {
        match self {
            GateState::Waiting => "waiting",
            GateState::Closed => "closed",
            GateState::Open => "open",
        }
    }

    const fn icon(self) -> IconData {
        match self {
            GateState::Waiting => icons::CLOCK,
            GateState::Closed => icons::LOCK,
            GateState::Open => icons::CHECK,
        }
    }

    const fn tone(self) -> Tone {
        match self {
            GateState::Waiting => Tone::Neutral,
            GateState::Closed => Tone::Info,
            GateState::Open => Tone::Ok,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct PipelineGateProps {
    /// What the gate is, or what it waits for: "needs approval", "paused".
    pub label: AttrValue,
    #[prop_or_default]
    pub state: GateState,
    /// Overrides the state's colour: `Warn` for a pause, `Danger` for a
    /// failed check.
    #[prop_or_default]
    pub tone: Option<Tone>,
    /// Overrides the state's icon.
    #[prop_or_default]
    pub icon: Option<IconData>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A barrier in front of a stage, passed to [`PipelineStage`]'s `gate` prop.
#[function_component]
pub fn PipelineGate(props: &PipelineGateProps) -> Html {
    let tone = props.tone.unwrap_or(props.state.tone());
    let class = classes!(
        "yi-pipeline-gate",
        format!("yi-pipeline-gate--{}", props.state.as_str()),
        format!("yi-pipeline-gate--{}", tone.as_str()),
        props.class.clone(),
    );
    props.attrs.apply(html! {
        <div id={props.id.clone()} {class}>
            <span class="yi-pipeline-gate__bar" aria-hidden="true" />
            <span class="yi-pipeline-gate__icon" aria-hidden="true">
                <Icon icon={props.icon.unwrap_or(props.state.icon())} />
            </span>
            <span class="yi-pipeline-gate__label">
                <VisuallyHidden>{ "Gate: " }</VisuallyHidden>
                { props.label.clone() }
            </span>
            <span class="yi-pipeline-gate__bar" aria-hidden="true" />
        </div>
    })
}

#[derive(Properties, PartialEq)]
pub struct PipelineUnitsProps {
    /// The [`PipelineUnit`]s.
    #[prop_or_default]
    pub children: Html,
    /// Accessible name of the group, e.g. "Hosts in canary".
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// The most units on a row before the group grows downwards instead.
    /// Unset, units wrap at the stage's width.
    #[prop_or_default]
    pub per_row: Option<u32>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A wrapping grid of [`PipelineUnit`]s inside a stage.
#[function_component]
pub fn PipelineUnits(props: &PipelineUnitsProps) -> Html {
    props.attrs.apply(html! {
        <div
            id={props.id.clone()}
            class={classes!("yi-pipeline-units", props.class.clone())}
            role="group"
            aria-label={props.label.clone()}
            style={props.per_row.map(|n| format!("--yi-pipeline-units-per-row:{n}"))}
        >
            { props.children.clone() }
        </div>
    })
}

/// How a unit's block is filled. Together with its tone this tells apart
/// states that share a colour, such as planned and applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum UnitFill {
    /// A full block of the tone: the state has been reached.
    #[default]
    Solid,
    /// A tinted block with an outline: prepared, not yet acted on.
    Soft,
    /// An outline only: nothing has happened yet.
    Hollow,
    /// A dashed outline: waiting on something else.
    Dashed,
}

impl UnitFill {
    pub const ALL: [UnitFill; 4] = [UnitFill::Solid, UnitFill::Soft, UnitFill::Hollow, UnitFill::Dashed];

    const fn as_str(self) -> &'static str {
        match self {
            UnitFill::Solid => "solid",
            UnitFill::Soft => "soft",
            UnitFill::Hollow => "hollow",
            UnitFill::Dashed => "dashed",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct PipelineUnitProps {
    /// The unit's name and state, e.g. "web-03: healthy". It is the
    /// accessible name and the hover text, since the block shows no text.
    pub label: AttrValue,
    #[prop_or_default]
    pub tone: Tone,
    #[prop_or_default]
    pub fill: UnitFill,
    /// A circle instead of a rounded square, for a second kind of unit.
    #[prop_or_default]
    pub round: bool,
    /// Pulses to show the unit is being watched or worked on.
    #[prop_or_default]
    pub pulse: bool,
    /// Makes the block a link to the unit.
    #[prop_or_default]
    pub href: Option<AttrValue>,
    /// Makes the block a button. Ignored with `href`.
    #[prop_or_default]
    pub on_click: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// One unit of work in a stage (a host, a job, a shard) as a small block.
#[function_component]
pub fn PipelineUnit(props: &PipelineUnitProps) -> Html {
    let class = classes!(
        "yi-pipeline-unit",
        format!("yi-pipeline-unit--{}", props.tone.as_str()),
        format!("yi-pipeline-unit--{}", props.fill.as_str()),
        props.round.then_some("yi-pipeline-unit--round"),
        props.pulse.then_some("yi-pipeline-unit--pulse"),
        props.class.clone(),
    );
    let id = props.id.clone();
    let label = props.label.clone();

    props.attrs.apply(if let Some(href) = &props.href {
        html! { <a {id} {class} href={href.clone()} aria-label={label.clone()} title={label} /> }
    } else if let Some(on_click) = &props.on_click {
        html! {
            <button {id} {class} type="button" aria-label={label.clone()} title={label} onclick={on_click.clone()} />
        }
    } else {
        html! { <span {id} {class} role="img" aria-label={label.clone()} title={label} /> }
    })
}

#[cfg(test)]
mod tests;
