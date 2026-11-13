//! Modal dialogs, built on the native `<dialog>` element.
//!
//! The browser supplies the hard parts: the top layer, the focus trap, an
//! inert page behind, `Escape`, and focus returning to the trigger. The
//! components here add the controlled `open` prop, the frame (title, body,
//! footer) and dismissal by clicking the backdrop.
//!
//! [`Dialog`] is the general frame; [`ConfirmDialog`] is the confirmation an
//! irreversible action goes through.

use yew::prelude::*;

use crate::{
    Attrs, Button, ButtonVariant, Checkbox, Field, IconButton, Size, TextInput, hooks::use_id, icons, is_empty,
};

/// What [`use_modal`] hands a component that renders a modal `<dialog>`.
pub(crate) struct Modal {
    /// Goes on the `<dialog>` element.
    pub node: NodeRef,
    /// The `closedby` attribute: `none` when it cannot be dismissed.
    pub closedby: Option<&'static str>,
    pub onkeydown: Callback<KeyboardEvent>,
    pub oncancel: Callback<Event>,
    pub onclose: Callback<Event>,
    pub onmousedown: Callback<MouseEvent>,
    pub onclick: Callback<MouseEvent>,
    /// Requests a close, for a close button.
    pub close: Callback<MouseEvent>,
}

/// Keeps a native `<dialog>` in step with a controlled `open` prop.
///
/// - `open` shows the element as a modal; anything else closes it. Closing
///   (or unmounting while open) hands focus back to where it came from.
/// - `Escape` and a click on the backdrop emit `on_close` when `dismissible`;
///   the dialog only closes once the owner sets `open` to false.
#[hook]
pub(crate) fn use_modal(open: bool, dismissible: bool, on_close: &Callback<()>) -> Modal {
    let node = use_node_ref();
    // What the owner last asked for, readable from event handlers.
    let wanted = use_mut_ref(|| open);
    *wanted.borrow_mut() = open;
    // Whether the press that precedes a click also began on the backdrop, so
    // dragging a text selection out of the dialog does not dismiss it.
    let pressed_backdrop = use_mut_ref(|| false);

    {
        let node = node.clone();
        use_effect_with(open, move |&open| {
            if open {
                dom::show(&node);
            }
            move || {
                if open {
                    dom::close(&node);
                }
            }
        });
    }

    Modal {
        closedby: (!dismissible).then_some("none"),
        // A dialog that may not be dismissed swallows `Escape` before the
        // browser acts on it.
        onkeydown: Callback::from(move |event: KeyboardEvent| {
            if !dismissible && event.key() == "Escape" {
                event.prevent_default();
            }
        }),
        // `Escape`. The owner decides, so the native close is always cancelled.
        oncancel: {
            let on_close = on_close.clone();
            Callback::from(move |event: Event| {
                event.prevent_default();
                if dismissible {
                    on_close.emit(());
                }
            })
        },
        // The browser closed it without being asked: it stops honouring a
        // cancelled `Escape` when the key is pressed again. Hand the decision
        // to the owner, or put back a dialog that may not be dismissed.
        onclose: {
            let (node, wanted, on_close) = (node.clone(), wanted.clone(), on_close.clone());
            Callback::from(move |_| {
                if !*wanted.borrow() {
                    return;
                }
                if dismissible {
                    on_close.emit(());
                } else {
                    dom::show(&node);
                }
            })
        },
        onmousedown: {
            let (node, pressed_backdrop) = (node.clone(), pressed_backdrop.clone());
            Callback::from(move |event: MouseEvent| {
                *pressed_backdrop.borrow_mut() = node.get().as_deref() == event.target().as_ref();
            })
        },
        onclick: {
            let (node, on_close) = (node.clone(), on_close.clone());
            Callback::from(move |event: MouseEvent| {
                let on_backdrop = node.get().as_deref() == event.target().as_ref();
                if dismissible && on_backdrop && pressed_backdrop.replace(false) {
                    on_close.emit(());
                }
            })
        },
        close: on_close.reform(|_| ()),
        node,
    }
}

/// The close button shared by dialog and drawer headers.
pub(crate) fn close_button(label: &AttrValue, on_click: Callback<MouseEvent>) -> Html {
    html! {
        <IconButton
            class="yi-dialog__close"
            icon={icons::X}
            label={label.clone()}
            variant={ButtonVariant::Subtle}
            size={Size::Small}
            {on_click}
        />
    }
}

#[derive(Properties, PartialEq)]
pub struct DialogProps {
    /// Whether the dialog is showing. The owner holds this state and clears
    /// it in `on_close`.
    #[prop_or_default]
    pub open: bool,
    /// The heading, which also names the dialog for assistive technology.
    pub title: AttrValue,
    /// The body. Only rendered while open, so its state starts fresh each
    /// time. Give an element `autofocus` to choose where focus lands.
    #[prop_or_default]
    pub children: Html,
    /// Actions, pinned below the body and aligned to the end.
    #[prop_or_default]
    pub footer: Html,
    /// The maximum width: `Small` for a short question, `Large` for a form
    /// with columns or a table.
    #[prop_or_default]
    pub size: Size,
    /// Whether `Escape`, the backdrop and the close button dismiss it. Turn
    /// this off while work is in flight or when an answer is required.
    #[prop_or(true)]
    pub dismissible: bool,
    /// An interruption that needs an answer: announced as an alert dialog,
    /// with the body read out as its description.
    #[prop_or_default]
    pub alert: bool,
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

/// A modal dialog: a focused task or question that holds the keyboard until
/// it is closed.
#[function_component]
pub fn Dialog(props: &DialogProps) -> Html {
    let modal = use_modal(props.open, props.dismissible, &props.on_close);
    let title_id = use_id();
    let body_id = format!("{title_id}-body");

    let class = classes!("yi-dialog", format!("yi-dialog--{}", props.size.as_str()), props.class.clone());

    props.attrs.apply(html! {
        <dialog
            ref={modal.node}
            id={props.id.clone()}
            {class}
            role={props.alert.then_some("alertdialog")}
            aria-labelledby={title_id.clone()}
            aria-describedby={props.alert.then(|| body_id.clone())}
            closedby={modal.closedby}
            onkeydown={modal.onkeydown}
            oncancel={modal.oncancel}
            onclose={modal.onclose}
            onmousedown={modal.onmousedown}
            onclick={modal.onclick}
        >
            if props.open {
                <div class="yi-dialog__surface">
                    <header class="yi-dialog__head">
                        <h2 class="yi-dialog__title" id={title_id}>{ props.title.clone() }</h2>
                        if props.dismissible {
                            { close_button(&props.close_label, modal.close) }
                        }
                    </header>
                    <div class="yi-dialog__body" id={body_id}>{ props.children.clone() }</div>
                    if !is_empty(&props.footer) {
                        <footer class="yi-dialog__foot">{ props.footer.clone() }</footer>
                    }
                </div>
            }
        </dialog>
    })
}

#[derive(Properties, PartialEq)]
pub struct ConfirmDialogProps {
    /// Whether the confirmation is showing.
    #[prop_or_default]
    pub open: bool,
    /// The question: "Forget pi-1?".
    pub title: AttrValue,
    /// What will happen, to what. Extra fields the action needs (a reason, a
    /// note) go here too; gate the action on them with `disabled`.
    #[prop_or_default]
    pub children: Html,
    /// The action, as the button says it: "Approve 12 hosts", "Forget pi-1".
    /// Never "Yes" or "OK", so the buttons cannot be told apart only by
    /// position.
    pub confirm_label: AttrValue,
    /// The label of the button that backs out.
    #[prop_or(AttrValue::Static("Cancel"))]
    pub cancel_label: AttrValue,
    /// The action destroys something: the confirm button is styled as danger.
    #[prop_or_default]
    pub danger: bool,
    /// The action is running: the confirm button shows progress and the
    /// dialog cannot be dismissed.
    #[prop_or_default]
    pub busy: bool,
    /// Keeps the confirm button disabled, for conditions the caller checks.
    #[prop_or_default]
    pub disabled: bool,
    /// A statement the user must tick before the action is enabled: "I have
    /// reviewed the 4 destructive steps".
    #[prop_or_default]
    pub acknowledge: Option<AttrValue>,
    /// A word the user must type before the action is enabled, usually the
    /// name of the thing being destroyed.
    #[prop_or_default]
    pub type_to_confirm: Option<AttrValue>,
    /// Why the last attempt failed, shown as an alert above the actions.
    #[prop_or_default]
    pub error: Option<AttrValue>,
    /// The user confirmed. Not fired while `busy`, `disabled` or incomplete.
    #[prop_or_default]
    pub on_confirm: Callback<()>,
    /// The user backed out: the cancel button, `Escape` or the backdrop.
    #[prop_or_default]
    pub on_cancel: Callback<()>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// The confirmation an irreversible action goes through. It names what is
/// about to happen, and can require a ticked statement or a typed word first.
#[function_component]
pub fn ConfirmDialog(props: &ConfirmDialogProps) -> Html {
    let acknowledged = use_state_eq(|| false);
    let typed = use_state_eq(String::new);

    // Each opening starts from scratch.
    {
        let (acknowledged, typed) = (acknowledged.clone(), typed.clone());
        use_effect_with(props.open, move |&open| {
            if !open {
                acknowledged.set(false);
                typed.set(String::new());
            }
        });
    }

    let ready = confirm_ready(
        props.acknowledge.is_some(),
        *acknowledged,
        props.type_to_confirm.as_deref(),
        &typed,
        props.disabled,
    );

    let on_acknowledge = {
        let acknowledged = acknowledged.clone();
        Callback::from(move |checked: bool| acknowledged.set(checked))
    };
    let on_type = {
        let typed = typed.clone();
        Callback::from(move |value: String| typed.set(value))
    };
    let on_confirm = {
        let (on_confirm, busy) = (props.on_confirm.clone(), props.busy);
        Callback::from(move |_| {
            if ready && !busy {
                on_confirm.emit(());
            }
        })
    };
    // `Enter` in the word box confirms, as submitting a form would.
    let on_keydown = {
        let on_confirm = on_confirm.clone();
        Callback::from(move |event: KeyboardEvent| {
            if event.key() == "Enter" {
                event.prevent_default();
                on_confirm.emit(());
            }
        })
    };

    // Focus lands on the first thing the user has to do, and otherwise on the
    // way out, so a stray `Enter` never destroys anything.
    let has_steps = props.acknowledge.is_some() || props.type_to_confirm.is_some();

    let footer = html! {
        <>
            <Button
                disabled={props.busy}
                attrs={Attrs::new((!has_steps).then_some(("autofocus", AttrValue::Static(""))))}
                on_click={props.on_cancel.reform(|_| ())}
            >
                { props.cancel_label.clone() }
            </Button>
            <Button
                variant={if props.danger { ButtonVariant::Danger } else { ButtonVariant::Primary }}
                disabled={!ready}
                loading={props.busy}
                on_click={on_confirm.reform(|_| ())}
            >
                { props.confirm_label.clone() }
            </Button>
        </>
    };

    html! {
        <Dialog
            open={props.open}
            title={props.title.clone()}
            size={Size::Small}
            alert=true
            dismissible={!props.busy}
            on_close={props.on_cancel.clone()}
            class={classes!("yi-confirm", props.class.clone())}
            id={props.id.clone()}
            attrs={props.attrs.clone()}
            {footer}
        >
            { props.children.clone() }
            if let Some(statement) = &props.acknowledge {
                <Checkbox checked={*acknowledged} autofocus=true on_change={on_acknowledge}>
                    { statement.clone() }
                </Checkbox>
            }
            if let Some(word) = &props.type_to_confirm {
                <Field label={html! { <>{ "Type " }<code class="yi-confirm__word">{ word.clone() }</code>{ " to confirm" }</> }}>
                    <TextInput
                        value={(*typed).clone()}
                        mono=true
                        autocomplete="off"
                        attrs={type_attrs(props.acknowledge.is_none())}
                        on_input={on_type}
                        on_keydown={on_keydown}
                    />
                </Field>
            }
            if let Some(error) = &props.error {
                <p class="yi-confirm__error" role="alert">{ error.clone() }</p>
            }
        </Dialog>
    }
}

/// What the word box needs beyond `TextInput`'s props: no capitalising on a
/// touch keyboard, and focus when it is the first step.
fn type_attrs(autofocus: bool) -> Attrs {
    Attrs::new(
        [("autocapitalize", AttrValue::Static("off"))]
            .into_iter()
            .chain(autofocus.then_some(("autofocus", AttrValue::Static("")))),
    )
}

/// Whether every condition on the confirm button is met.
fn confirm_ready(needs_acknowledge: bool, acknowledged: bool, word: Option<&str>, typed: &str, disabled: bool) -> bool {
    !disabled && (!needs_acknowledge || acknowledged) && word.is_none_or(|word| typed.trim() == word)
}

#[cfg(target_arch = "wasm32")]
mod dom {
    //! `showModal()` and `close()`, called by name so the crate needs no
    //! extra `web-sys` features.

    use wasm_bindgen::{JsCast, JsValue};
    use yew::NodeRef;

    fn is_open(element: &web_sys::Element) -> bool {
        element.has_attribute("open")
    }

    fn call(element: &web_sys::Element, method: &str) {
        if let Ok(function) = js_sys::Reflect::get(element, &JsValue::from_str(method))
            && let Ok(function) = function.dyn_into::<js_sys::Function>()
        {
            let _ = function.call0(element);
        }
    }

    pub fn show(node: &NodeRef) {
        if let Some(element) = node.cast::<web_sys::Element>()
            && !is_open(&element)
        {
            call(&element, "showModal");
        }
    }

    pub fn close(node: &NodeRef) {
        if let Some(element) = node.cast::<web_sys::Element>()
            && is_open(&element)
        {
            call(&element, "close");
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod dom {
    use yew::NodeRef;

    pub fn show(_node: &NodeRef) {}

    pub fn close(_node: &NodeRef) {}
}

#[cfg(test)]
mod tests;
