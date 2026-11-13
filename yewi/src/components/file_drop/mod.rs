//! [`FileDrop`]: choosing files by dropping them or through the browser's
//! file picker.
//!
//! The zone is a `<label>` around a real `<input type="file">`, so the
//! keyboard, the focus ring and the picker come from the browser and the
//! drop handling is an addition to them. The bytes are never read here:
//! each [`SelectedFile`] carries the browser's own `File`, which can go
//! straight into a `FormData` or a `fetch` body.

use std::rc::Rc;

use yew::prelude::*;

use crate::is_empty;
use crate::{
    Attrs, FieldContext, Icon, IconData, Size, Spinner, components::server_only::use_server_only, icons,
    use_field_control, use_id,
};

/// A file the user chose or dropped.
#[derive(Debug, Clone, PartialEq)]
pub struct SelectedFile {
    /// The file's name, without a path.
    pub name: AttrValue,
    /// Its size in bytes.
    pub size: u64,
    /// The media type the browser reported, e.g. `application/zip`. Empty
    /// when the browser does not know it.
    pub mime: AttrValue,
    #[cfg(target_arch = "wasm32")]
    raw: wasm_bindgen::JsValue,
}

impl SelectedFile {
    /// Describes a file without a browser `File` behind it, for tests and
    /// server-side fixtures.
    pub fn new(name: impl Into<AttrValue>, size: u64, mime: impl Into<AttrValue>) -> Self {
        Self {
            name: name.into(),
            size,
            mime: mime.into(),
            #[cfg(target_arch = "wasm32")]
            raw: wasm_bindgen::JsValue::UNDEFINED,
        }
    }

    /// The browser's `File` object, to upload or read. With the `File`
    /// feature of `web-sys`: `file.js().clone().unchecked_into::<web_sys::File>()`.
    #[cfg(target_arch = "wasm32")]
    pub fn js(&self) -> &wasm_bindgen::JsValue {
        &self.raw
    }
}

/// Why a [`FileDrop`] turned a file away.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileRejection {
    /// Its name or media type does not match `accept`.
    Type,
    /// It is larger than `max_size`.
    TooLarge,
    /// More than one file arrived and `multiple` is off. The first is kept.
    TooMany,
}

/// A file that was offered but not passed to `on_files`.
#[derive(Debug, Clone, PartialEq)]
pub struct RejectedFile {
    pub file: SelectedFile,
    pub reason: FileRejection,
}

/// Whether a file matches an `accept` list: comma-separated extensions
/// (`.zip`), media types (`application/zip`) and wildcards (`image/*`).
fn accepts(accept: &str, file: &SelectedFile) -> bool {
    let name = file.name.to_ascii_lowercase();
    let mime = file.mime.to_ascii_lowercase();
    let mut patterns = accept.split(',').map(|p| p.trim().to_ascii_lowercase()).filter(|p| !p.is_empty()).peekable();

    patterns.peek().is_none()
        || patterns.any(|pattern| match pattern.strip_suffix("/*") {
            _ if pattern.starts_with('.') => name.ends_with(&pattern),
            Some(kind) => mime.split_once('/').is_some_and(|(found, _)| found == kind),
            None => mime == pattern,
        })
}

/// Splits what arrived into the files to hand on and those turned away.
/// The picker applies `accept` and `multiple` itself; a drop does not.
fn screen(
    files: Vec<SelectedFile>,
    accept: Option<&str>,
    max_size: Option<u64>,
    multiple: bool,
) -> (Vec<SelectedFile>, Vec<RejectedFile>) {
    let mut accepted = Vec::new();
    let mut rejected = Vec::new();

    for file in files {
        let reason = if accept.is_some_and(|accept| !accepts(accept, &file)) {
            Some(FileRejection::Type)
        } else if max_size.is_some_and(|max| file.size > max) {
            Some(FileRejection::TooLarge)
        } else if !multiple && !accepted.is_empty() {
            Some(FileRejection::TooMany)
        } else {
            None
        };

        match reason {
            Some(reason) => rejected.push(RejectedFile { file, reason }),
            None => accepted.push(file),
        }
    }

    (accepted, rejected)
}

/// Reads files out of browser events. Done through reflection so the crate
/// needs no extra `web-sys` features.
#[cfg(target_arch = "wasm32")]
mod dom {
    use js_sys::Reflect;
    use wasm_bindgen::JsValue;
    use yew::{DragEvent, Event};

    use super::SelectedFile;

    fn get(target: &JsValue, key: &str) -> JsValue {
        Reflect::get(target, &key.into()).unwrap_or(JsValue::UNDEFINED)
    }

    fn text(target: &JsValue, key: &str) -> yew::AttrValue {
        get(target, key).as_string().unwrap_or_default().into()
    }

    fn files(list: &JsValue) -> Vec<SelectedFile> {
        let length = get(list, "length").as_f64().unwrap_or(0.0) as u32;
        (0..length)
            .filter_map(|index| Reflect::get_u32(list, index).ok())
            .filter(JsValue::is_object)
            .map(|raw| SelectedFile {
                name: text(&raw, "name"),
                size: get(&raw, "size").as_f64().unwrap_or(0.0) as u64,
                mime: text(&raw, "type"),
                raw,
            })
            .collect()
    }

    /// The files picked in the `<input>` a `change` event came from. Unless
    /// it is to `keep` them for a form, the input is then emptied: choosing
    /// the same file twice in a row would otherwise raise no second event.
    pub fn chosen(event: &Event, keep: bool) -> Vec<SelectedFile> {
        let Some(input) = event.target() else {
            return Vec::new();
        };
        let found = files(&get(&input, "files"));
        if !keep {
            let _ = Reflect::set(&input, &"value".into(), &"".into());
        }
        found
    }

    pub fn dropped(event: &DragEvent) -> Vec<SelectedFile> {
        files(&get(&get(event.as_ref(), "dataTransfer"), "files"))
    }

    /// Whether a drag carries files, as opposed to text or a link.
    pub fn carries_files(event: &DragEvent) -> bool {
        let types = get(&get(event.as_ref(), "dataTransfer"), "types");
        js_sys::Array::is_array(&types) && js_sys::Array::from(&types).includes(&"Files".into(), 0)
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod dom {
    use yew::{DragEvent, Event};

    use super::SelectedFile;

    pub fn chosen(_: &Event, _: bool) -> Vec<SelectedFile> {
        Vec::new()
    }

    pub fn dropped(_: &DragEvent) -> Vec<SelectedFile> {
        Vec::new()
    }

    pub fn carries_files(_: &DragEvent) -> bool {
        false
    }
}

#[derive(Properties, PartialEq)]
pub struct FileDropProps {
    /// Fired with the files that were chosen or dropped and passed `accept`,
    /// `max_size` and `multiple`, in the order the browser listed them.
    #[prop_or_default]
    pub on_files: Callback<Vec<SelectedFile>>,
    /// Fired with the files that were turned away and why, so the caller can
    /// say so. Rejected files never reach `on_files`.
    #[prop_or_default]
    pub on_reject: Callback<Vec<RejectedFile>>,
    /// The name the files are submitted under in a native form. The input
    /// then keeps what was chosen, which is what the form sends.
    #[prop_or_default]
    pub name: Option<AttrValue>,
    /// What to drop, in the imperative. Defaults to "Drop a file here, or
    /// choose one" (or its plural with `multiple`).
    #[prop_or_default]
    pub label: Html,
    /// What the zone accepts and what happens to it afterwards.
    #[prop_or_default]
    pub hint: Html,
    /// Shown above the label. `None` removes it.
    #[prop_or(Some(icons::UPLOAD))]
    pub icon: Option<IconData>,
    /// The native `accept` list: extensions (`.zip`), media types
    /// (`application/zip`) or wildcards (`image/*`), comma separated. Applied
    /// to dropped files as well as the picker.
    #[prop_or_default]
    pub accept: Option<AttrValue>,
    /// Allows more than one file at a time.
    #[prop_or_default]
    pub multiple: bool,
    /// The largest file accepted, in bytes.
    #[prop_or_default]
    pub max_size: Option<u64>,
    /// `Small` is a single row for toolbars and dense forms.
    #[prop_or_default]
    pub size: Size,
    /// An upload is in flight: shows a spinner and `busy_label`, and takes no
    /// new files until it is cleared.
    #[prop_or_default]
    pub busy: bool,
    /// The label while `busy`.
    #[prop_or(AttrValue::Static("Uploading…"))]
    pub busy_label: AttrValue,
    /// Unavailable: not focusable, and drops are ignored.
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub required: bool,
    /// Marks the zone as having a problem. A surrounding `Field` with an
    /// `error` sets this for you.
    #[prop_or_default]
    pub invalid: bool,
    /// The id of the `<input>`. Ignored inside a `Field`, which assigns the
    /// id its label points at.
    #[prop_or_default]
    pub input_id: Option<AttrValue>,
    /// A reference to the `<input>`, for moving focus.
    #[prop_or_default]
    pub input_ref: NodeRef,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A zone that takes files by drag and drop, by click or from the keyboard.
/// It reports the files and keeps none: the caller lists, uploads or
/// discards them. With a `name` it is a form control instead, and inside
/// `ServerOnly` it then shows the names of the files chosen.
#[function_component]
pub fn FileDrop(props: &FileDropProps) -> Html {
    let control = use_field_control(props.input_id.clone(), props.invalid, props.required, props.disabled);
    let field = use_context::<FieldContext>();
    let base = use_id();
    let server_only = use_server_only();
    let over = use_state_eq(|| false);
    let blocked = control.disabled || props.busy;

    let label_id = format!("{base}-label");
    let hint_id = (!is_empty(&props.hint)).then(|| format!("{base}-hint"));
    // Inside a field the zone is named by the field's label and its own.
    let labelled_by = match &field {
        Some(field) => format!("{} {label_id}", field.label_id),
        None => label_id.clone(),
    };
    let described_by = [control.described_by.as_deref(), hint_id.as_deref()].into_iter().flatten().collect::<Vec<_>>();
    let described_by = (!described_by.is_empty()).then(|| described_by.join(" "));

    let deliver = {
        let rules = Rc::new((props.accept.clone(), props.max_size, props.multiple));
        let (on_files, on_reject) = (props.on_files.clone(), props.on_reject.clone());
        Callback::from(move |files: Vec<SelectedFile>| {
            let (accepted, rejected) = screen(files, rules.0.as_deref(), rules.1, rules.2);
            // Accepted files first, so a caller that clears its message on
            // success still shows the rejections from the same drop.
            if !accepted.is_empty() {
                on_files.emit(accepted);
            }
            if !rejected.is_empty() {
                on_reject.emit(rejected);
            }
        })
    };

    let named = props.name.is_some();
    let onchange = deliver.reform(move |event: Event| dom::chosen(&event, named));
    let ondragover = {
        let over = over.clone();
        Callback::from(move |event: DragEvent| {
            if dom::carries_files(&event) {
                // Marks the zone as a drop target; without it the browser
                // would open the file in place of the page.
                event.prevent_default();
                over.set(!blocked);
            }
        })
    };
    let ondragleave = {
        let over = over.clone();
        Callback::from(move |_: DragEvent| over.set(false))
    };
    let ondrop = {
        let over = over.clone();
        Callback::from(move |event: DragEvent| {
            event.prevent_default();
            over.set(false);
            if !blocked {
                deliver.emit(dom::dropped(&event));
            }
        })
    };

    let class = classes!(
        "yi-file-drop",
        format!("yi-file-drop--{}", props.size.as_str()),
        (*over).then_some("yi-file-drop--over"),
        control.invalid.then_some("yi-file-drop--invalid"),
        control.disabled.then_some("yi-file-drop--disabled"),
        props.busy.then_some("yi-file-drop--busy"),
        props.class.clone(),
    );
    let label = match props.multiple {
        _ if props.busy => html! { props.busy_label.clone() },
        _ if !is_empty(&props.label) => props.label.clone(),
        false => html! { "Drop a file here, or choose one" },
        true => html! { "Drop files here, or choose them" },
    };

    props.attrs.apply(html! {
        <label
            id={props.id.clone()}
            {class}
            aria-busy={props.busy.then_some("true")}
            data-yi-file-drop={server_only.then_some("")}
            ondragenter={ondragover.clone()}
            {ondragover}
            {ondragleave}
            {ondrop}
        >
            <input
                ref={props.input_ref.clone()}
                class="yi-file-drop__input"
                type="file"
                id={control.id.clone()}
                name={props.name.clone()}
                required={server_only && control.required}
                accept={props.accept.clone()}
                multiple={props.multiple}
                disabled={blocked}
                aria-required={control.required.then_some("true")}
                aria-invalid={control.aria_invalid()}
                aria-labelledby={labelled_by}
                aria-describedby={described_by}
                {onchange}
            />
            if props.busy {
                <Spinner class="yi-file-drop__icon" />
            } else if let Some(icon) = props.icon {
                <Icon {icon} class="yi-file-drop__icon" />
            }
            <span class="yi-file-drop__text">
                <span id={label_id} class="yi-file-drop__label">{ label }</span>
                if !is_empty(&props.hint) {
                    <span id={hint_id} class="yi-file-drop__hint">{ props.hint.clone() }</span>
                }
            </span>
        </label>
    })
}

#[cfg(test)]
mod tests;
