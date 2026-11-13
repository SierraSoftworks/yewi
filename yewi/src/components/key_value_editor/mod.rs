//! [`KeyValueEditor`]: a short list of name and value pairs, such as labels,
//! environment variables or headers, edited a row at a time.

use yew::prelude::*;

use crate::{
    Attrs, Button, ButtonVariant, FieldContext, IconButton, Size, TextInput, attrs, icons, use_field_control, use_id,
};

/// One row of a [`KeyValueEditor`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

impl KeyValue {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self { key: key.into(), value: value.into() }
    }

    /// Whether neither side has anything but whitespace: a row the user
    /// added and has not filled in, which most callers skip.
    pub fn is_blank(&self) -> bool {
        self.key.trim().is_empty() && self.value.trim().is_empty()
    }
}

impl<K: Into<String>, V: Into<String>> From<(K, V)> for KeyValue {
    fn from((key, value): (K, V)) -> Self {
        Self::new(key, value)
    }
}

#[derive(Properties, PartialEq)]
pub struct KeyValueEditorProps {
    /// The rows shown, in order. Update them from `on_change` to keep the
    /// editor controlled.
    #[prop_or_default]
    pub rows: Vec<KeyValue>,
    /// Fired with every row after an edit, an added row or a removed one.
    /// Rows are passed as typed: trim and check them where they are used.
    #[prop_or_default]
    pub on_change: Callback<Vec<KeyValue>>,
    /// Problems to show, each as the index of a row and what is wrong with
    /// it. The row's inputs are marked invalid and described by the message.
    #[prop_or_default]
    pub errors: Vec<(usize, AttrValue)>,
    /// The accessible name of the editor. Not needed inside a `Field`, whose
    /// label names it.
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// What one row is called in the names of its controls: `"label"` gives
    /// "Key, label 1" and "Remove label 1".
    #[prop_or(AttrValue::Static("row"))]
    pub row_label: AttrValue,
    #[prop_or(AttrValue::Static("key"))]
    pub key_placeholder: AttrValue,
    #[prop_or(AttrValue::Static("value"))]
    pub value_placeholder: AttrValue,
    /// The text between the two inputs. Empty leaves it out.
    #[prop_or(AttrValue::Static("="))]
    pub separator: AttrValue,
    /// The text of the button that adds a row.
    #[prop_or(AttrValue::Static("Add row"))]
    pub add_label: AttrValue,
    /// The most rows allowed: at the limit the add button is disabled.
    #[prop_or_default]
    pub max_rows: Option<usize>,
    /// Renders both inputs in the monospaced face, for identifiers.
    #[prop_or_default]
    pub mono: bool,
    #[prop_or_default]
    pub size: Size,
    /// Disables every input and button.
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// Where focus goes once the rows have re-rendered.
#[derive(Clone, Copy, PartialEq)]
enum Focus {
    /// The key of the row just added.
    LastKey,
    /// The add button, after the focused remove button went away.
    Add,
}

#[cfg(target_arch = "wasm32")]
fn focus_within(root: &NodeRef, selector: &str) {
    use wasm_bindgen::JsCast;

    let target = root
        .cast::<web_sys::Element>()
        .and_then(|root| root.query_selector(selector).ok().flatten())
        .and_then(|target| target.dyn_into::<web_sys::HtmlElement>().ok());
    if let Some(target) = target {
        let _ = target.focus();
    }
}

/// Rows of a key and a value, with a button to remove each and one to add
/// another. It holds no state and does no checking of its own: pass what
/// `on_change` reports back as `rows`, and report problems through `errors`.
///
/// Put it in a `Field` with `group=true` to give it a label and hint.
#[function_component]
pub fn KeyValueEditor(props: &KeyValueEditorProps) -> Html {
    let control = use_field_control(None, false, false, props.disabled);
    let field = use_context::<FieldContext>();
    let base = use_id();
    let root = use_node_ref();
    let pending_focus = use_mut_ref(|| None::<Focus>);

    {
        let (root, pending_focus) = (root.clone(), pending_focus.clone());
        use_effect_with(props.rows.len(), move |_| {
            let focus = pending_focus.borrow_mut().take();
            #[cfg(target_arch = "wasm32")]
            match focus {
                Some(Focus::LastKey) => focus_within(&root, ".yi-key-value-editor__row:last-of-type input"),
                Some(Focus::Add) => focus_within(&root, ".yi-key-value-editor__add"),
                None => {}
            }
            #[cfg(not(target_arch = "wasm32"))]
            let _ = (root, focus);
        });
    }

    let edit = |index: usize, key: bool| {
        let (rows, on_change) = (props.rows.clone(), props.on_change.clone());
        Callback::from(move |text: String| {
            let mut rows = rows.clone();
            if let Some(row) = rows.get_mut(index) {
                if key { row.key = text } else { row.value = text }
            }
            on_change.emit(rows);
        })
    };
    let remove = |index: usize| {
        let (rows, on_change, pending_focus) = (props.rows.clone(), props.on_change.clone(), pending_focus.clone());
        Callback::from(move |_: MouseEvent| {
            let mut rows = rows.clone();
            rows.remove(index);
            *pending_focus.borrow_mut() = Some(Focus::Add);
            on_change.emit(rows);
        })
    };
    let add = {
        let (rows, on_change, pending_focus) = (props.rows.clone(), props.on_change.clone(), pending_focus.clone());
        Callback::from(move |_: MouseEvent| {
            let mut rows = rows.clone();
            rows.push(KeyValue::default());
            *pending_focus.borrow_mut() = Some(Focus::LastKey);
            on_change.emit(rows);
        })
    };

    let full = props.max_rows.is_some_and(|max| props.rows.len() >= max);
    let names_group = props.label.is_some() || field.is_some();
    let class = classes!(
        "yi-key-value-editor",
        props.separator.is_empty().then_some("yi-key-value-editor--bare"),
        props.class.clone(),
    );

    props.attrs.apply(html! {
        <div
            ref={root}
            id={props.id.clone()}
            {class}
            role={names_group.then_some("group")}
            aria-label={props.label.clone()}
            aria-labelledby={field.as_ref().filter(|_| props.label.is_none()).map(|field| field.label_id.clone())}
        >
            { for props.rows.iter().enumerate().map(|(index, row)| {
                let number = index + 1;
                let error = props.errors.iter().find(|(row, _)| *row == index).map(|(_, message)| message.clone());
                let error_id = error.as_ref().map(|_| AttrValue::from(format!("{base}-error-{index}")));
                // Each row is its own field as far as its inputs can tell, so
                // they take the row's error and never the id of a Field around
                // the whole editor.
                let context = FieldContext {
                    control_id: None,
                    label_id: field.as_ref().map(|field| field.label_id.clone()).unwrap_or_else(|| base.clone()),
                    described_by: error_id.clone(),
                    invalid: error.is_some(),
                    required: false,
                    disabled: control.disabled,
                };
                html! {
                    <div class="yi-key-value-editor__row">
                        <ContextProvider<FieldContext> {context}>
                            <TextInput
                                class="yi-key-value-editor__key"
                                value={row.key.clone()}
                                on_input={edit(index, true)}
                                placeholder={props.key_placeholder.clone()}
                                mono={props.mono}
                                size={props.size}
                                autocomplete="off"
                                attrs={attrs! { "aria-label" => format!("Key, {} {number}", props.row_label) }}
                            />
                            if !props.separator.is_empty() {
                                <span class="yi-key-value-editor__separator" aria-hidden="true">
                                    { props.separator.clone() }
                                </span>
                            }
                            <TextInput
                                class="yi-key-value-editor__value"
                                value={row.value.clone()}
                                on_input={edit(index, false)}
                                placeholder={props.value_placeholder.clone()}
                                mono={props.mono}
                                size={props.size}
                                autocomplete="off"
                                attrs={attrs! { "aria-label" => format!("Value, {} {number}", props.row_label) }}
                            />
                        </ContextProvider<FieldContext>>
                        <IconButton
                            class="yi-key-value-editor__remove"
                            icon={icons::X}
                            label={format!("Remove {} {number}", props.row_label)}
                            variant={ButtonVariant::Subtle}
                            size={props.size}
                            disabled={control.disabled}
                            on_click={remove(index)}
                        />
                        if let Some(error) = error {
                            <p id={error_id} class="yi-key-value-editor__error">{ error }</p>
                        }
                    </div>
                }
            }) }
            <Button
                class="yi-key-value-editor__add"
                icon={icons::PLUS}
                size={Size::Small}
                disabled={control.disabled || full}
                on_click={add}
            >
                { props.add_label.clone() }
            </Button>
        </div>
    })
}

#[cfg(test)]
mod tests;
