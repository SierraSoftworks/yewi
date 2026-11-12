#![cfg(feature = "terminal")]
//! Terminal: a macOS-style window for shell sessions and command output.

use std::fmt::Write as _;

use yew::prelude::*;

use crate::{Attrs, Tone};

mod ansi;

use ansi::{Colour, Span, Style};

/// The window buttons drawn in the title bar. They are decoration only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum TerminalControls {
    /// The red, amber and green macOS buttons.
    #[default]
    Coloured,
    /// Three neutral dots, for pages where colour would compete with content.
    Muted,
    /// No buttons.
    Hidden,
}

impl TerminalControls {
    pub const ALL: [TerminalControls; 3] =
        [TerminalControls::Coloured, TerminalControls::Muted, TerminalControls::Hidden];
}

#[derive(Properties, PartialEq)]
pub struct TerminalProps {
    /// The session: `TerminalCommand` and `TerminalOutput` lines.
    #[prop_or_default]
    pub children: Html,
    /// Text centred in the title bar, such as the shell or working directory.
    #[prop_or_default]
    pub title: Option<AttrValue>,
    /// The accessible name. Defaults to `title`, then to "Terminal".
    #[prop_or_default]
    pub label: Option<AttrValue>,
    #[prop_or_default]
    pub controls: TerminalControls,
    /// Caps the height at this many lines of text; longer sessions scroll.
    #[prop_or_default]
    pub max_rows: Option<u32>,
    /// Keeps the newest output in view as lines are added, until the reader
    /// scrolls up. Also announces new output to assistive technology.
    #[prop_or_default]
    pub follow: bool,
    /// Scrolls long lines sideways instead of wrapping them.
    #[prop_or_default]
    pub nowrap: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A terminal window. It is dark in both themes.
#[function_component]
pub fn Terminal(props: &TerminalProps) -> Html {
    let body = use_node_ref();
    // Whether the reader is at the end of the output. Following pauses while
    // they are not.
    let pinned = use_mut_ref(|| true);

    let onscroll = {
        let (body, pinned) = (body.clone(), pinned.clone());
        Callback::from(move |_: Event| {
            #[cfg(target_arch = "wasm32")]
            if let Some(body) = body.cast::<web_sys::Element>() {
                let remaining = body.scroll_height() - body.scroll_top() - body.client_height();
                *pinned.borrow_mut() = remaining <= 4;
            }
            #[cfg(not(target_arch = "wasm32"))]
            let _ = (&body, &pinned);
        })
    };

    {
        let (body, pinned, follow) = (body.clone(), pinned.clone(), props.follow);
        use_effect(move || {
            #[cfg(target_arch = "wasm32")]
            if follow
                && *pinned.borrow()
                && let Some(body) = body.cast::<web_sys::Element>()
            {
                body.set_scroll_top(body.scroll_height());
            }
            #[cfg(not(target_arch = "wasm32"))]
            let _ = (&body, &pinned, follow);
        });
    }

    let label = props.label.clone().or_else(|| props.title.clone()).unwrap_or_else(|| "Terminal".into());
    let scrolls = props.max_rows.is_some() || props.nowrap;
    let class = classes!(
        "yi-terminal",
        (props.controls == TerminalControls::Muted).then_some("yi-terminal--muted"),
        props.nowrap.then_some("yi-terminal--nowrap"),
        props.class.clone(),
    );

    props.attrs.apply(html! {
        <div id={props.id.clone()} {class} role="group" aria-label={label.clone()}>
            if props.controls != TerminalControls::Hidden || props.title.is_some() {
                <div class="yi-terminal__bar">
                    if props.controls != TerminalControls::Hidden {
                        <span class="yi-terminal__controls" aria-hidden="true">
                            <span class="yi-terminal__control yi-terminal__control--close" />
                            <span class="yi-terminal__control yi-terminal__control--minimise" />
                            <span class="yi-terminal__control yi-terminal__control--zoom" />
                        </span>
                    }
                    if let Some(title) = &props.title {
                        <span class="yi-terminal__title">{ title }</span>
                    }
                </div>
            }
            <div
                ref={body}
                class="yi-terminal__body"
                style={props.max_rows.map(|rows| format!("--yi-terminal-rows:{rows}"))}
                // A scrolling region must be reachable from the keyboard.
                tabindex={scrolls.then_some("0")}
                role={props.follow.then_some("log")}
                aria-label={props.follow.then(|| format!("{label} output"))}
                {onscroll}
            >
                { props.children.clone() }
            </div>
        </div>
    })
}

#[derive(Properties, PartialEq)]
pub struct TerminalCommandProps {
    /// The command as typed.
    #[prop_or_default]
    pub children: Html,
    /// The prompt shown before the command. It is drawn by CSS, so selecting
    /// and copying the line yields the command alone.
    #[prop_or(AttrValue::Static("$"))]
    pub prompt: AttrValue,
    /// Shows a block cursor after the command, as if input is awaited.
    #[prop_or_default]
    pub cursor: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A line the user typed, after a prompt.
#[function_component]
pub fn TerminalCommand(props: &TerminalCommandProps) -> Html {
    props.attrs.apply(html! {
        <div
            id={props.id.clone()}
            class={classes!("yi-terminal__command", props.class.clone())}
            data-prompt={props.prompt.clone()}
        >
            <span class="yi-terminal__input">{ props.children.clone() }</span>
            if props.cursor {
                <span class="yi-terminal__cursor" aria-hidden="true" />
            }
        </div>
    })
}

#[derive(Properties, PartialEq)]
pub struct TerminalOutputProps {
    /// Output to show as written. Use this for markup of your own; use `text`
    /// for raw program output.
    #[prop_or_default]
    pub children: Html,
    /// Raw program output. ANSI colour and style sequences are rendered, and
    /// every other escape sequence is removed. Shown before `children`.
    #[prop_or_default]
    pub text: Option<AttrValue>,
    /// Colours the whole block by meaning, for example `Tone::Danger` for an
    /// error. `Neutral` keeps the default text colour.
    #[prop_or_default]
    pub tone: Tone,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// One or more lines a program printed. Whitespace and line breaks are kept.
#[function_component]
pub fn TerminalOutput(props: &TerminalOutputProps) -> Html {
    let spans = use_memo(props.text.clone(), |text| text.as_deref().map(ansi::parse).unwrap_or_default());

    let class = classes!(
        "yi-terminal__output",
        (props.tone != Tone::Neutral).then(|| format!("yi-terminal__output--{}", props.tone.as_str())),
        props.class.clone(),
    );

    props.attrs.apply(html! {
        <div id={props.id.clone()} {class}>
            { for spans.iter().map(span) }
            { props.children.clone() }
        </div>
    })
}

fn span(span: &Span) -> Html {
    if span.style.is_plain() {
        return html! { { span.text.clone() } };
    }
    let (class, style) = presentation(&span.style);
    html! { <span {class} {style}>{ span.text.clone() }</span> }
}

const FG: [&str; 16] = ["yi-terminal__sgr--fg-0", "yi-terminal__sgr--fg-1", "yi-terminal__sgr--fg-2", "yi-terminal__sgr--fg-3", "yi-terminal__sgr--fg-4", "yi-terminal__sgr--fg-5", "yi-terminal__sgr--fg-6", "yi-terminal__sgr--fg-7", "yi-terminal__sgr--fg-8", "yi-terminal__sgr--fg-9", "yi-terminal__sgr--fg-10", "yi-terminal__sgr--fg-11", "yi-terminal__sgr--fg-12", "yi-terminal__sgr--fg-13", "yi-terminal__sgr--fg-14", "yi-terminal__sgr--fg-15"];
const BG: [&str; 16] = ["yi-terminal__sgr--bg-0", "yi-terminal__sgr--bg-1", "yi-terminal__sgr--bg-2", "yi-terminal__sgr--bg-3", "yi-terminal__sgr--bg-4", "yi-terminal__sgr--bg-5", "yi-terminal__sgr--bg-6", "yi-terminal__sgr--bg-7", "yi-terminal__sgr--bg-8", "yi-terminal__sgr--bg-9", "yi-terminal__sgr--bg-10", "yi-terminal__sgr--bg-11", "yi-terminal__sgr--bg-12", "yi-terminal__sgr--bg-13", "yi-terminal__sgr--bg-14", "yi-terminal__sgr--bg-15"];

/// The classes, and for colours outside the palette the inline style, that
/// draw `style`.
fn presentation(style: &Style) -> (Classes, Option<String>) {
    let mut class = Classes::new();
    let mut inline = String::new();

    // Inverse swaps the colours, falling back to the window's own pair.
    let (fg, bg) = if style.inverse { (style.bg, style.fg) } else { (style.fg, style.bg) };
    for (colour, classes, property) in [(fg, &FG, "color"), (bg, &BG, "background-color")] {
        match colour {
            Some(Colour::Palette(n)) => class.push(classes[usize::from(n & 15)]),
            Some(Colour::Rgb(r, g, b)) => write!(inline, "{property}:rgb({r} {g} {b});").unwrap(),
            None => {}
        }
    }
    for (on, name) in [
        (style.inverse, "yi-terminal__sgr--inverse"),
        (style.bold, "yi-terminal__sgr--bold"),
        (style.dim, "yi-terminal__sgr--dim"),
        (style.italic, "yi-terminal__sgr--italic"),
        (style.underline, "yi-terminal__sgr--underline"),
        (style.strike, "yi-terminal__sgr--strike"),
    ] {
        if on {
            class.push(name);
        }
    }

    (class, (!inline.is_empty()).then_some(inline))
}

#[cfg(test)]
mod tests;
