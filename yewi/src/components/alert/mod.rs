//! Messages about the state of a page: [`Alert`] for a block in the page
//! flow, [`Banner`] for a full-width strip and [`Note`] for a quiet aside.
//!
//! None of them float or time out. A transient message that should not move
//! the layout is a toast.

use yew::prelude::*;

use crate::{Attrs, ButtonVariant, Icon, IconButton, IconData, Size, Tone, icons, is_empty};

/// Problems interrupt assistive technology; everything else waits its turn.
const fn live_role(tone: Tone) -> &'static str {
    match tone {
        Tone::Warn | Tone::Danger => "alert",
        _ => "status",
    }
}

fn dismiss_button(class: &'static str, label: &AttrValue, on_dismiss: &Option<Callback<()>>) -> Html {
    let Some(on_dismiss) = on_dismiss else { return Html::default() };
    let on_click = on_dismiss.reform(|_: MouseEvent| ());

    html! {
        <IconButton {class} icon={icons::X} label={label.clone()} variant={ButtonVariant::Subtle} size={Size::Small} {on_click} />
    }
}

#[derive(Properties, PartialEq)]
pub struct AlertProps {
    /// The message. Rich content such as a list of advice is fine.
    #[prop_or_default]
    pub children: Html,
    /// A short headline above the message.
    #[prop_or_default]
    pub title: Option<AttrValue>,
    /// What kind of message this is. Sets the colour, the icon and how
    /// urgently it is announced.
    #[prop_or(Tone::Info)]
    pub tone: Tone,
    /// Replaces the tone's icon.
    #[prop_or_default]
    pub icon: Option<IconData>,
    /// Leaves the icon out.
    #[prop_or_default]
    pub hide_icon: bool,
    /// Ways out of the problem, usually one or two buttons, shown in a row
    /// under the message.
    #[prop_or_default]
    pub actions: Html,
    /// Shows a dismiss button and fires when it is pressed. The alert does
    /// not remove itself: stop rendering it. Leave unset for anything the
    /// page still depends on being true.
    #[prop_or_default]
    pub on_dismiss: Option<Callback<()>>,
    /// Accessible name of the dismiss button.
    #[prop_or(AttrValue::Static("Dismiss"))]
    pub dismiss_label: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A block-level message in the page flow that surfaces a problem or an
/// outcome and offers a way forward: a raised surface with a rule in the
/// tone along its leading edge. `Warn` and `Danger` use `role="alert"`,
/// other tones `role="status"`; pass `role` through `attrs` to override.
#[function_component]
pub fn Alert(props: &AlertProps) -> Html {
    let class = classes!(
        "yi-alert",
        format!("yi-alert--{}", props.tone.as_str()),
        props.class.clone(),
    );

    props.attrs.apply(html! {
        <div id={props.id.clone()} {class} role={live_role(props.tone)}>
            if !props.hide_icon {
                <span class="yi-alert__icon">
                    <Icon icon={props.icon.unwrap_or(props.tone.icon())} />
                </span>
            }
            <div class="yi-alert__body">
                if let Some(title) = &props.title {
                    <div class="yi-alert__title">{ title.clone() }</div>
                }
                if !is_empty(&props.children) {
                    <div class="yi-alert__message">{ props.children.clone() }</div>
                }
                if !is_empty(&props.actions) {
                    <div class="yi-alert__actions">{ props.actions.clone() }</div>
                }
            </div>
            { dismiss_button("yi-alert__dismiss", &props.dismiss_label, &props.on_dismiss) }
        </div>
    })
}

#[derive(Properties, PartialEq)]
pub struct BannerProps {
    /// The message: a sentence, not a paragraph.
    #[prop_or_default]
    pub children: Html,
    /// What kind of message this is. Sets the colour, the icon and how
    /// urgently it is announced.
    #[prop_or(Tone::Info)]
    pub tone: Tone,
    /// Fills the banner with the tone itself instead of a tint, for the one
    /// status a page exists to report.
    #[prop_or_default]
    pub bold: bool,
    /// `Small` is a slim strip, `Large` a headline with room around it.
    #[prop_or_default]
    pub size: Size,
    /// Replaces the tone's icon.
    #[prop_or_default]
    pub icon: Option<IconData>,
    /// Leaves the icon out.
    #[prop_or_default]
    pub hide_icon: bool,
    /// A link or small button at the trailing end.
    #[prop_or_default]
    pub actions: Html,
    /// Shows a dismiss button and fires when it is pressed. The banner does
    /// not remove itself: stop rendering it.
    #[prop_or_default]
    pub on_dismiss: Option<Callback<()>>,
    /// Accessible name of the dismiss button.
    #[prop_or(AttrValue::Static("Dismiss"))]
    pub dismiss_label: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A full-width strip for a message about the whole page or app: an outage,
/// a read-only mode, the overall status. It is square and takes the width of
/// its container; where it sits (and whether it sticks) is up to the layout.
#[function_component]
pub fn Banner(props: &BannerProps) -> Html {
    let class = classes!(
        "yi-banner",
        format!("yi-banner--{}", props.tone.as_str()),
        props.bold.then_some("yi-banner--bold"),
        format!("yi-banner--{}", props.size.as_str()),
        props.class.clone(),
    );

    props.attrs.apply(html! {
        <div id={props.id.clone()} {class} role={live_role(props.tone)}>
            if !props.hide_icon {
                <span class="yi-banner__icon">
                    <Icon icon={props.icon.unwrap_or(props.tone.icon())} />
                </span>
            }
            <div class="yi-banner__message">{ props.children.clone() }</div>
            if !is_empty(&props.actions) {
                <div class="yi-banner__actions">{ props.actions.clone() }</div>
            }
            { dismiss_button("yi-banner__dismiss", &props.dismiss_label, &props.on_dismiss) }
        </div>
    })
}

#[derive(Properties, PartialEq)]
pub struct NoteProps {
    /// The text. Inline content only: the note is a paragraph.
    #[prop_or_default]
    pub children: Html,
    /// `Neutral` is muted text without an icon; any other tone colours the
    /// note and shows the tone's icon before the text.
    #[prop_or_default]
    pub tone: Tone,
    /// Replaces the tone's icon, or gives a `Neutral` note one.
    #[prop_or_default]
    pub icon: Option<IconData>,
    /// Leaves the icon out.
    #[prop_or_default]
    pub hide_icon: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A quiet line of supporting text beside the thing it explains: a caveat
/// under a form, the outcome of the last save, why a control is unavailable.
/// It is not announced; add `role="status"` through `attrs` when it appears
/// in response to an action.
#[function_component]
pub fn Note(props: &NoteProps) -> Html {
    let class = classes!("yi-note", format!("yi-note--{}", props.tone.as_str()), props.class.clone());
    let icon = props.icon.or((props.tone != Tone::Neutral).then_some(props.tone.icon())).filter(|_| !props.hide_icon);

    props.attrs.apply(html! {
        <p id={props.id.clone()} {class}>
            if let Some(icon) = icon {
                <Icon {icon} class="yi-note__icon" />
            }
            <span class="yi-note__text">{ props.children.clone() }</span>
        </p>
    })
}

#[cfg(test)]
mod tests;
