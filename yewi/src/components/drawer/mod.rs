//! A panel that slides in from the edge of the viewport, for a task that
//! keeps the page behind it in view: create and edit forms, details, and
//! navigation on a narrow screen.
//!
//! It is a modal `<dialog>` like [`Dialog`](crate::Dialog) and behaves the
//! same way: it holds the keyboard, closes on `Escape` or a click on the
//! backdrop, and returns focus when it closes.

use yew::prelude::*;

use crate::{
    Attrs, Size,
    components::dialog::{close_button, use_modal},
    hooks::use_id,
};

/// The edge a drawer is attached to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum DrawerSide {
    /// The inline end (the right, in left-to-right text): forms and details.
    #[default]
    End,
    /// The inline start: navigation.
    Start,
}

impl DrawerSide {
    pub const ALL: [DrawerSide; 2] = [DrawerSide::End, DrawerSide::Start];

    const fn as_str(self) -> &'static str {
        match self {
            DrawerSide::End => "end",
            DrawerSide::Start => "start",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct DrawerProps {
    /// Whether the drawer is showing. The owner holds this state and clears
    /// it in `on_close`.
    #[prop_or_default]
    pub open: bool,
    /// The heading, which also names the drawer for assistive technology.
    /// Without one there is no header: pass `label` instead.
    #[prop_or_default]
    pub title: Option<AttrValue>,
    /// The accessible name when there is no `title`.
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// The body, which scrolls. Only rendered while open, so its state
    /// starts fresh each time.
    #[prop_or_default]
    pub children: Html,
    /// Actions, pinned to the bottom and aligned to the end.
    #[prop_or_default]
    pub footer: Option<Html>,
    #[prop_or_default]
    pub side: DrawerSide,
    /// The width: `Small` for navigation, `Large` for a wide form or a table.
    #[prop_or_default]
    pub size: Size,
    /// Whether `Escape`, the backdrop and the close button dismiss it.
    #[prop_or(true)]
    pub dismissible: bool,
    /// The accessible name of the close button.
    #[prop_or(AttrValue::Static("Close"))]
    pub close_label: AttrValue,
    /// The user asked to close it: `Escape`, the backdrop or the close button.
    #[prop_or_default]
    pub on_close: Callback<()>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A modal panel attached to one edge of the viewport.
#[function_component]
pub fn Drawer(props: &DrawerProps) -> Html {
    let modal = use_modal(props.open, props.dismissible, &props.on_close);
    let title_id = use_id();

    let class = classes!(
        "yi-drawer",
        format!("yi-drawer--{}", props.side.as_str()),
        format!("yi-drawer--{}", props.size.as_str()),
        props.class.clone(),
    );

    props.attrs.apply(html! {
        <dialog
            ref={modal.node}
            id={props.id.clone()}
            {class}
            aria-labelledby={props.title.is_some().then(|| title_id.clone())}
            aria-label={if props.title.is_none() { props.label.clone() } else { None }}
            oncancel={modal.oncancel}
            onclose={modal.onclose}
            onmousedown={modal.onmousedown}
            onclick={modal.onclick}
        >
            if props.open {
                <div class="yi-drawer__surface">
                    if let Some(title) = &props.title {
                        <header class="yi-drawer__head">
                            <h2 class="yi-drawer__title" id={title_id}>{ title.clone() }</h2>
                            if props.dismissible {
                                { close_button(&props.close_label, modal.close) }
                            }
                        </header>
                    }
                    <div class="yi-drawer__body">{ props.children.clone() }</div>
                    if let Some(footer) = &props.footer {
                        <footer class="yi-drawer__foot">{ footer.clone() }</footer>
                    }
                </div>
            }
        </dialog>
    })
}

#[cfg(test)]
mod tests;
