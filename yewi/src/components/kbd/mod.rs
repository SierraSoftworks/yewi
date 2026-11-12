//! Keys and keyboard shortcuts.

use yew::prelude::*;

use crate::Attrs;

#[derive(Properties, PartialEq)]
pub struct KbdProps {
    /// A single key, such as `Esc`. Ignored when `keys` is set.
    #[prop_or_default]
    pub children: Html,
    /// A shortcut written with `+` between keys held together and a space
    /// between steps pressed in turn: `"Ctrl+Shift+P"`, `"G H"`. Each key
    /// gets its own cap.
    #[prop_or_default]
    pub keys: Option<AttrValue>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A key cap, or with `keys` a whole shortcut.
///
/// Renders `<kbd>`; a shortcut nests one `<kbd>` per key inside an outer
/// `<kbd>`, as HTML defines for a combined input.
#[function_component]
pub fn Kbd(props: &KbdProps) -> Html {
    props.attrs.apply(match &props.keys {
        None => html! {
            <kbd id={props.id.clone()} class={classes!("yi-kbd", props.class.clone())}>
                { props.children.clone() }
            </kbd>
        },
        Some(keys) => html! {
            <kbd id={props.id.clone()} class={classes!("yi-kbd-combo", props.class.clone())}>
                { for steps(keys).enumerate().map(|(i, step)| html! {
                    <>
                        if i > 0 {
                            <span class="yi-kbd-combo__then">{ " " }</span>
                        }
                        { for step.into_iter().enumerate().map(|(j, key)| html! {
                            <>
                                if j > 0 {
                                    <span class="yi-kbd-combo__plus">{ "+" }</span>
                                }
                                <kbd class="yi-kbd">{ key }</kbd>
                            </>
                        }) }
                    </>
                }) }
            </kbd>
        },
    })
}

/// Splits a shortcut into steps (by whitespace) of keys (by `+`). A `+` that
/// is itself the key, as in `Ctrl++`, is kept.
fn steps(keys: &str) -> impl Iterator<Item = Vec<&str>> + '_ {
    keys.split_whitespace().map(|step| {
        let mut out = Vec::new();
        let mut parts = step.split('+').peekable();
        while let Some(part) = parts.next() {
            if !part.is_empty() {
                out.push(part);
            } else if parts.next_if(|next| next.is_empty()).is_some() {
                out.push("+");
            }
        }
        out
    })
}

#[cfg(test)]
mod tests;
