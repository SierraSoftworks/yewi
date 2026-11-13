#![cfg(feature = "schema-form")]
//! [`SchemaForm`]: a form drawn from a JSON Schema.
//!
//! One set of controls for every document an app edits from a schema: a
//! plugin's configuration, a workflow's settings, a user's preferences. The
//! caller hands over a schema and a document and gets the control each part
//! calls for, laid out as labelled [`Field`]s with the schema's descriptions
//! as hints.
//!
//! It is controlled: it shows `value` and reports the whole new document
//! through `on_change`. It does not validate; the server that owns the schema
//! does, and its findings come back through `errors`.

use std::rc::Rc;

use yew::prelude::*;

use crate::{
    Attrs, Button, Field, FieldLayout, Icon, IconButton, Json, NumberInput, SecretInput, Select, SelectOption, SelectOptions,
    Size, TagPicker, TextArea, TextInput, TextInputKind, ToggleSwitch, ButtonVariant, attrs, icons, use_id,
};

mod schema;

use schema::{Kind, Property, TextKind, Variant};

/// How a nested part of the document (an object, a tagged union, a list) is
/// set apart from the fields around it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum SchemaGroups {
    /// A tonal box around each group. The default: the clearest separation.
    #[default]
    Boxed,
    /// A title, then the fields indented behind a rule. Costs less width
    /// where groups nest deeply.
    Ruled,
    /// A title only, with the fields at the same indent as their neighbours,
    /// for shallow schemas that should read as one flat form.
    Flat,
}

impl SchemaGroups {
    pub const ALL: [SchemaGroups; 3] = [SchemaGroups::Boxed, SchemaGroups::Ruled, SchemaGroups::Flat];

    const fn as_str(self) -> &'static str {
        match self {
            SchemaGroups::Ruled => "ruled",
            SchemaGroups::Boxed => "boxed",
            SchemaGroups::Flat => "flat",
        }
    }
}

/// Problems with the document, each addressed to the value it is about.
///
/// The address is a JSON Pointer (`/area/lat`, `/sensors/0`), which is what
/// schema validators report. A dotted path (`area.lat`) is accepted too. A
/// problem addressed to a group shows under the group's title.
///
/// ```ignore
/// let errors = SchemaErrors::from([("/days", "Must be at most 5")]);
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SchemaErrors(Rc<[(AttrValue, AttrValue)]>);

impl SchemaErrors {
    /// The message for exactly this pointer.
    pub fn get(&self, pointer: &str) -> Option<&AttrValue> {
        self.0.iter().find(|(at, _)| at == pointer).map(|(_, message)| message)
    }

    /// Whether there are no problems.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<P: Into<AttrValue>, M: Into<AttrValue>> FromIterator<(P, M)> for SchemaErrors {
    fn from_iter<I: IntoIterator<Item = (P, M)>>(errors: I) -> Self {
        let pointer = |path: AttrValue| match path.is_empty() || path.starts_with('/') {
            true => path,
            false => path.split('.').fold(String::new(), |pointer, step| schema::child(&pointer, step).to_string()).into(),
        };
        Self(errors.into_iter().map(|(path, message)| (pointer(path.into()), message.into())).collect())
    }
}

impl<P: Into<AttrValue>, M: Into<AttrValue>, const N: usize> From<[(P, M); N]> for SchemaErrors {
    fn from(errors: [(P, M); N]) -> Self {
        errors.into_iter().collect()
    }
}

impl<P: Into<AttrValue>, M: Into<AttrValue>> From<Vec<(P, M)>> for SchemaErrors {
    fn from(errors: Vec<(P, M)>) -> Self {
        errors.into_iter().collect()
    }
}

/// The words the form itself supplies, for translation.
#[derive(Debug, Clone, PartialEq)]
pub struct SchemaFormLabels {
    /// The button that adds an item to a list.
    pub add: AttrValue,
    /// The verb in an item's remove button, followed by the item's name.
    pub remove: AttrValue,
    /// The verb in the switch that includes an optional group, followed by
    /// the group's name.
    pub include: AttrValue,
    /// A list item's name, followed by its number.
    pub item: AttrValue,
    /// An optional choice that has not been made.
    pub not_set: AttrValue,
    /// A required choice that has not been made.
    pub choose: AttrValue,
    /// An optional yes-or-no, answered yes.
    pub yes: AttrValue,
    /// An optional yes-or-no, answered no.
    pub no: AttrValue,
}

impl Default for SchemaFormLabels {
    fn default() -> Self {
        Self {
            add: "Add".into(),
            remove: "Remove".into(),
            include: "Include".into(),
            item: "Item".into(),
            not_set: "Not set".into(),
            choose: "Choose one".into(),
            yes: "Yes".into(),
            no: "No".into(),
        }
    }
}

/// One value of the document, as handed to [`SchemaFormProps::render`] so the
/// app can draw its own control for it.
#[derive(Debug, Clone, PartialEq)]
pub struct SchemaControl {
    /// Where the value sits in the document, as a JSON Pointer.
    pub pointer: AttrValue,
    /// This value's schema, as written (a `$ref` is not followed).
    pub schema: Json,
    /// The schema's `format`, the usual way to ask for a custom control.
    pub format: Option<AttrValue>,
    /// The value now. `None` when the document does not have it.
    pub value: Option<Json>,
    /// Report a new value. `None` removes it from the document.
    pub on_change: Callback<Option<Json>>,
    pub required: bool,
    pub disabled: bool,
}

#[derive(Properties, PartialEq)]
pub struct SchemaFormProps {
    /// The JSON Schema. Parse it once (in `use_memo`) and pass it on every
    /// render.
    pub schema: Json,
    /// The document so far. `Json::Null` is an empty one.
    #[prop_or(Json::Null)]
    pub value: Json,
    /// Fired with the whole new document on every edit. A cleared optional
    /// value is removed from the document rather than stored as `null`.
    #[prop_or_default]
    pub on_change: Callback<Json>,
    /// Problems to show against the values they are about, usually from the
    /// server that refused the document.
    #[prop_or_default]
    pub errors: SchemaErrors,
    /// Draws an app's own control for a value: return `Some` to replace the
    /// built-in control, `None` to keep it. The label, hint and error are
    /// still drawn around it. Called for every value and group below the
    /// root; memoise it (`use_callback`), or every field re-renders on every
    /// edit.
    #[prop_or_default]
    pub render: Option<Callback<SchemaControl, Option<Html>>>,
    /// Where each field's label sits.
    #[prop_or_default]
    pub layout: FieldLayout,
    /// How nested groups are set apart.
    #[prop_or_default]
    pub groups: SchemaGroups,
    /// The form's own words, for translation.
    #[prop_or_default]
    pub labels: SchemaFormLabels,
    /// Makes every control unavailable, while a save is in flight.
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// What every node of one form shares.
#[derive(PartialEq)]
struct Shared {
    root: Json,
    errors: SchemaErrors,
    edit: Callback<(AttrValue, Option<Json>)>,
    render: Option<Callback<SchemaControl, Option<Html>>>,
    layout: FieldLayout,
    labels: SchemaFormLabels,
    disabled: bool,
}

/// A form drawn from a JSON Schema, editing one JSON document.
///
/// - Strings, numbers, booleans, choices (`enum`, `oneOf` of constants),
///   secrets (`format: password`), lists of strings (as tags), nested
///   objects, tagged unions and lists each get the matching yewi control.
///   `title` is the label, `description` the hint, `default`, `minimum`,
///   `maximum` and `examples` are honoured, and local `$ref`s are followed.
/// - A part of the schema it cannot read becomes a JSON box for that one
///   value, so an unusual schema costs a nice control, never the ability to
///   edit.
/// - It renders a `<div>`, not a `<form>`: put it inside your own form or
///   card next to the buttons that save it.
/// - Only the fields on the path of an edit re-render, so large schemas stay
///   responsive.
#[function_component]
pub fn SchemaForm(props: &SchemaFormProps) -> Html {
    // One stable callback for every node: it reads the latest document and
    // listener when an edit arrives, so it never changes and unedited
    // subtrees are skipped.
    let latest = use_mut_ref(|| (Json::Null, Callback::<Json>::noop()));
    *latest.borrow_mut() = (props.value.clone(), props.on_change.clone());
    let edit = use_callback((), move |(pointer, value): (AttrValue, Option<Json>), _| {
        let (document, on_change) = latest.borrow().clone();
        on_change.emit(schema::set_at(&document, &pointer, value));
    });

    let shared = Rc::new(Shared {
        root: props.schema.clone(),
        errors: props.errors.clone(),
        edit,
        render: props.render.clone(),
        layout: props.layout,
        labels: props.labels.clone(),
        disabled: props.disabled,
    });
    let class = classes!("yi-schema-form", format!("yi-schema-form--{}", props.groups.as_str()), props.class.clone());
    let value = Some(props.value.clone()).filter(|value| *value != Json::Null);

    props.attrs.apply(html! {
        <div id={props.id.clone()} {class}>
            if let Some(error) = props.errors.get("") {
                <p class="yi-field__message yi-field__message--error" role="alert">
                    <Icon icon={icons::CIRCLE_ALERT} />
                    <span>{ error }</span>
                </p>
            }
            <SchemaNode {shared} schema={props.schema.clone()} pointer="" label="" required=true {value} />
        </div>
    })
}

#[derive(Properties, PartialEq)]
struct SchemaNodeProps {
    shared: Rc<Shared>,
    /// This node's own schema.
    schema: Json,
    /// Where this node's value sits in the document.
    pointer: AttrValue,
    label: AttrValue,
    required: bool,
    value: Option<Json>,
    /// A list item: it carries its own remove button and cannot be emptied.
    #[prop_or_default]
    item: bool,
    #[prop_or_default]
    depth: usize,
}

impl SchemaNodeProps {
    fn on_change(&self) -> Callback<Option<Json>> {
        let pointer = self.pointer.clone();
        self.shared.edit.reform(move |value| (pointer.clone(), value))
    }

    fn error(&self) -> Option<Html> {
        self.shared.errors.get(&self.pointer).map(|message| html! { message })
    }

    fn hint(&self) -> Option<Html> {
        schema::summary(&self.shared.root, &self.schema).map(|summary| html! { summary })
    }

    fn keyword(&self, name: &str) -> Option<&Json> {
        schema::keyword(&self.shared.root, &self.schema, name)
    }

    fn disabled(&self) -> bool {
        self.shared.disabled || self.keyword("readOnly") == Some(&Json::Bool(true))
    }

    /// A child node for `step` below this one.
    fn child(&self, step: &str, schema: &Json, label: String, required: bool, value: Option<&Json>, item: bool) -> Html {
        html! {
            <SchemaNode
                key={step.to_owned()}
                shared={self.shared.clone()}
                schema={schema.clone()}
                pointer={schema::child(&self.pointer, step)}
                {label}
                {required}
                value={value.cloned()}
                {item}
                depth={self.depth + 1}
            />
        }
    }

    /// A node per property, each writing its own key of this node's object.
    fn fields(&self, properties: &[Property<'_>]) -> Html {
        let root = &self.shared.root;
        properties
            .iter()
            .map(|property| {
                let label = schema::keyword(root, property.schema, "title")
                    .and_then(schema::text)
                    .map_or_else(|| schema::humanise(property.key), str::to_owned);
                let value = self.value.as_ref().and_then(|value| schema::get(value, property.key));
                self.child(property.key, property.schema, label, property.required, value, false)
            })
            .collect()
    }
}

/// One node of the form, drawn from one node of the schema. Recursive: an
/// object draws a node per property, a tagged union a picker and then the
/// chosen variant's properties, a list a node per item; everything else is a
/// single labelled control.
#[function_component]
fn SchemaNode(props: &SchemaNodeProps) -> Html {
    let base = use_id();
    let shared = &props.shared;
    let root = &shared.root;

    let custom = shared.render.as_ref().filter(|_| props.depth > 0).and_then(|render| {
        render.emit(SchemaControl {
            pointer: props.pointer.clone(),
            schema: props.schema.clone(),
            format: props.keyword("format").and_then(schema::text).map(|format| format.to_owned().into()),
            value: props.value.clone(),
            on_change: props.on_change(),
            required: props.required,
            disabled: props.disabled(),
        })
    });
    if let Some(control) = custom {
        return field(props, control);
    }

    // A schema may contain itself. Past this depth the rest is one JSON box.
    let kind = if props.depth > schema::MAX_DEPTH { Kind::Unknown } else { schema::kind(root, &props.schema) };
    match kind {
        // The document itself has no frame: the card around it is the frame.
        Kind::Object(properties) if props.depth == 0 => props.fields(&properties),
        Kind::Object(properties) => group(props, &base, Body::Fields(props.fields(&properties))),
        Kind::Variants(variants) => group(props, &base, Body::Fields(variant_fields(props, &variants))),
        Kind::List(item) => group(props, &base, Body::List(list(props, item))),
        leaf => field(props, control(props, &leaf)),
    }
}

/// The labelled row around a single control.
fn field(props: &SchemaNodeProps, control: Html) -> Html {
    let labels = &props.shared.labels;
    let row = html! {
        <Field
            label={html! { &props.label }}
            hint={props.hint().unwrap_or_default()}
            error={props.error().unwrap_or_default()}
            required={props.required && !props.item}
            disabled={props.disabled()}
            layout={props.shared.layout}
            hide_label={props.item}
            attrs={attrs! { "data-yi-pointer" => props.pointer.clone() }}
        >
            { control }
        </Field>
    };
    if !props.item {
        return row;
    }
    html! {
        <div class="yi-schema-form__item">
            { row }
            { remove_button(props, labels) }
        </div>
    }
}

fn remove_button(props: &SchemaNodeProps, labels: &SchemaFormLabels) -> Html {
    let on_click = props.on_change().reform(|_: MouseEvent| None);
    html! {
        <IconButton
            key="remove"
            icon={icons::X}
            label={format!("{} {}", labels.remove, props.label)}
            variant={ButtonVariant::Subtle}
            disabled={props.disabled()}
            {on_click}
        />
    }
}

/// What a group holds.
enum Body {
    Fields(Html),
    List(Html),
}

/// The frame every group shares: a title, the switch that includes an
/// optional group, the hint, and what is wrong with the group as a whole.
fn group(props: &SchemaNodeProps, base: &str, body: Body) -> Html {
    let labels = &props.shared.labels;
    let label_id = format!("{base}-label");
    // A list needs no switch: it is there once it has an item.
    let optional = !props.required && !props.item && !matches!(body, Body::List(_));
    let present = props.value.is_some() || !optional;
    let (Body::Fields(body) | Body::List(body)) = body;

    let toggle = {
        let (root, node, on_change) = (props.shared.root.clone(), props.schema.clone(), props.on_change());
        Callback::from(move |on: bool| on_change.emit(on.then(|| schema::default_for(&root, &node))))
    };

    html! {
        <div
            class="yi-schema-form__group"
            role="group"
            aria-labelledby={label_id.clone()}
            data-yi-pointer={props.pointer.clone()}
        >
            <div key="head" class="yi-schema-form__head">
                <span id={label_id} class="yi-schema-form__title">{ &props.label }</span>
                if optional {
                    <ToggleSwitch
                        checked={props.value.is_some()}
                        label={format!("{} {}", labels.include, props.label)}
                        size={Size::Small}
                        disabled={props.disabled()}
                        on_change={toggle}
                    />
                }
                if props.item {
                    { remove_button(props, labels) }
                }
            </div>
            if let Some(hint) = props.hint() {
                <p key="hint" class="yi-field__message">{ hint }</p>
            }
            if let Some(error) = props.error() {
                <p key="error" class="yi-field__message yi-field__message--error" role="alert">
                    <Icon icon={icons::CIRCLE_ALERT} />
                    <span>{ error }</span>
                </p>
            }
            <div key="body" class="yi-schema-form__body" hidden={!present}>
                if present {
                    { body }
                }
            </div>
        </div>
    }
}

/// The picker of a tagged union, then the chosen variant's own fields.
fn variant_fields(props: &SchemaNodeProps, variants: &[Variant<'_>]) -> Html {
    let labels = &props.shared.labels;
    let chosen = props.value.as_ref().and_then(|value| schema::variant_of(variants, value));
    let options: SelectOptions =
        variants.iter().enumerate().map(|(index, variant)| SelectOption::new(index.to_string(), variant.label.clone())).collect::<Vec<_>>().into();
    let picker =
        variants.first().and_then(|variant| variant.tag).map_or_else(|| "Type".to_owned(), |(tag, _)| schema::humanise(tag));

    // The closure outlives this render, so it reads the schema again from
    // what it owns rather than borrowing `variants`.
    let on_change = {
        let (root, node) = (props.shared.root.clone(), props.schema.clone());
        let (previous, on_change) = (props.value.clone(), props.on_change());
        Callback::from(move |picked: AttrValue| {
            if let (Kind::Variants(variants), Ok(index)) = (schema::kind(&root, &node), picked.parse::<usize>())
                && let Some(variant) = variants.get(index)
            {
                on_change.emit(Some(schema::switched(&root, variant, previous.as_ref())));
            }
        })
    };

    html! {
        <>
            <Field key="~variant" label={picker} required=true disabled={props.disabled()} layout={props.shared.layout}>
                <Select
                    {options}
                    value={chosen.map(|index| AttrValue::from(index.to_string()))}
                    placeholder={labels.choose.clone()}
                    {on_change}
                />
            </Field>
            if let Some(variant) = chosen.and_then(|index| variants.get(index)) {
                { props.fields(&variant.fields()) }
            }
        </>
    }
}

/// A node per item, then the button that adds one.
fn list(props: &SchemaNodeProps, item: &Json) -> Html {
    let labels = &props.shared.labels;
    let items = props.value.as_ref().and_then(schema::items).unwrap_or_default();
    let name = schema::keyword(&props.shared.root, item, "title").and_then(schema::text).unwrap_or(&labels.item);

    let add = {
        let fresh = schema::default_for(&props.shared.root, item);
        let (edit, pointer) = (props.shared.edit.clone(), schema::child(&props.pointer, &items.len().to_string()));
        Callback::from(move |_: MouseEvent| edit.emit((pointer.clone(), Some(fresh.clone()))))
    };

    html! {
        <>
            { for items.iter().enumerate().map(|(index, value)| {
                props.child(&index.to_string(), item, format!("{name} {}", index + 1), true, Some(value), true)
            }) }
            <Button
                key="~add"
                class="yi-schema-form__add"
                size={Size::Small}
                icon={icons::PLUS}
                disabled={props.disabled()}
                on_click={add}
            >
                { &labels.add }
            </Button>
        </>
    }
}

/// The single control a leaf wants.
fn control(props: &SchemaNodeProps, kind: &Kind<'_>) -> Html {
    let labels = &props.shared.labels;
    let root = &props.shared.root;
    let value = props.value.as_ref();
    let on_change = props.on_change();
    // A list item is removed with its button; emptying it keeps it.
    let keep_empty = props.required || props.item;
    let placeholder = props
        .keyword("examples")
        .and_then(schema::items)
        .and_then(|examples| examples.first())
        // A value left out takes the default, so show what that is.
        .or_else(|| props.keyword("default").filter(|default| !default.is_container()))
        .map(|example| AttrValue::from(schema::text(example).map_or_else(|| example.to_string(), str::to_owned)));

    match kind {
        Kind::Text(flavour) => {
            let text = AttrValue::from(value.and_then(schema::text).unwrap_or_default().to_owned());
            let on_input = on_change.reform(move |text: String| (keep_empty || !text.is_empty()).then(|| Json::from(text)));
            let max_length = props.keyword("maxLength").and_then(schema::number).map(|length| length as u32);
            match flavour {
                TextKind::Secret => html! { <SecretInput value={text} {on_input} {placeholder} {max_length} /> },
                TextKind::Multiline => html! { <TextArea value={text} {on_input} {placeholder} {max_length} /> },
                flavour => {
                    let kind = match flavour {
                        TextKind::Email => TextInputKind::Email,
                        TextKind::Url => TextInputKind::Url,
                        _ => TextInputKind::Text,
                    };
                    html! { <TextInput value={text} {on_input} {kind} {placeholder} {max_length} /> }
                }
            }
        }
        Kind::Integer | Kind::Number => {
            let integer = matches!(kind, Kind::Integer);
            let bound = |name: &str| props.keyword(name).and_then(schema::number);
            let on_input = Callback::from(move |number: Option<f64>| {
                match number.and_then(|number| schema::json_number(number, integer)) {
                    None if keep_empty => {}
                    next => on_change.emit(next),
                }
            });
            html! {
                <NumberInput
                    value={value.and_then(schema::number)}
                    min={bound("minimum")}
                    max={bound("maximum")}
                    step={bound("multipleOf").unwrap_or(1.0)}
                    stepper={integer}
                    {placeholder}
                    {on_input}
                />
            }
        }
        // A switch can only say yes or no. Where leaving the question open is
        // an answer too (the reader then applies its own default), offer it.
        Kind::Boolean if keep_empty || props.keyword("default").is_some() => {
            let default = props.keyword("default") == Some(&Json::Bool(true));
            let checked = value.map_or(default, |value| *value == Json::Bool(true));
            html! { <ToggleSwitch {checked} on_change={on_change.reform(|on: bool| Some(Json::Bool(on)))} /> }
        }
        Kind::Boolean => {
            let options = SelectOptions::from([
                SelectOption::new("", labels.not_set.clone()),
                SelectOption::new("true", labels.yes.clone()),
                SelectOption::new("false", labels.no.clone()),
            ]);
            let chosen = match value {
                Some(Json::Bool(on)) => Some(AttrValue::Static(if *on { "true" } else { "false" })),
                _ => None,
            };
            let on_change = on_change.reform(|picked: AttrValue| picked.parse::<bool>().ok().map(Json::Bool));
            html! { <Select {options} value={chosen} placeholder={labels.not_set.clone()} {on_change} /> }
        }
        Kind::Choice(values) => {
            // Picked by position, so that a choice between numbers stays a
            // number on its way back out of the list.
            let offered: Vec<Json> = values.iter().map(|value| (*value).clone()).collect();
            let mut options: Vec<SelectOption> = offered
                .iter()
                .enumerate()
                .map(|(index, value)| SelectOption::new(index.to_string(), schema::choice_label(root, &props.schema, value)))
                .collect();
            if !keep_empty {
                options.insert(0, SelectOption::new("", labels.not_set.clone()));
            }
            let chosen = value
                .and_then(|value| offered.iter().position(|offered| offered == value))
                .map(|index| AttrValue::from(index.to_string()));
            let on_change =
                on_change.reform(move |picked: AttrValue| picked.parse::<usize>().ok().and_then(|at| offered.get(at).cloned()));
            let placeholder = if keep_empty { &labels.choose } else { &labels.not_set };
            html! { <Select {options} value={chosen} placeholder={placeholder.clone()} {on_change} /> }
        }
        Kind::Tags(choices) => {
            let values: Vec<AttrValue> = value
                .and_then(schema::items)
                .unwrap_or_default()
                .iter()
                .filter_map(|item| schema::text(item).map(|text| AttrValue::from(text.to_owned())))
                .collect();
            let options: SelectOptions = choices
                .iter()
                .flatten()
                .filter_map(|choice| schema::text(choice).map(|text| SelectOption::plain(text.to_owned())))
                .collect::<Vec<_>>()
                .into();
            let on_change = on_change.reform(move |values: Vec<AttrValue>| {
                (keep_empty || !values.is_empty()).then(|| Json::Array(values.into_iter().map(Json::String).collect()))
            });
            html! { <TagPicker {values} {options} allow_custom={choices.is_none()} {placeholder} {on_change} /> }
        }
        _ => html! { <SchemaJsonBox value={props.value.clone()} {on_change} keep_empty={props.item} /> },
    }
}

#[derive(Properties, PartialEq)]
struct SchemaJsonBoxProps {
    value: Option<Json>,
    on_change: Callback<Option<Json>>,
    keep_empty: bool,
}

/// One value edited as JSON, for a part of a schema the form cannot draw.
///
/// It keeps the text somebody typed as text beside the value it parses to,
/// and reports only what parses: a box re-rendered from the parsed value
/// would reformat itself under the cursor.
#[function_component]
fn SchemaJsonBox(props: &SchemaJsonBoxProps) -> Html {
    let pretty = |value: &Option<Json>| value.as_ref().map(Json::to_pretty).unwrap_or_default();
    let draft = use_mut_ref(|| pretty(&props.value));
    let update = use_force_update();

    // The value changed from outside rather than by typing here, which is
    // the one time the text should follow it.
    let parsed = Json::parse(&draft.borrow()).ok();
    if parsed != props.value && (parsed.is_some() || draft.borrow().trim().is_empty()) {
        *draft.borrow_mut() = pretty(&props.value);
    }

    let on_input = {
        let (draft, on_change, keep_empty) = (draft.clone(), props.on_change.clone(), props.keep_empty);
        Callback::from(move |text: String| {
            match Json::parse(&text) {
                _ if text.trim().is_empty() && !keep_empty => on_change.emit(None),
                Ok(value) => on_change.emit(Some(value)),
                // Half-typed. Left alone until it is JSON.
                Err(_) => {}
            }
            *draft.borrow_mut() = text;
            update.force_update();
        })
    };
    let text = draft.borrow().clone();
    let invalid = !text.trim().is_empty() && Json::parse(&text).is_err();

    html! { <TextArea value={text} rows={4} monospace=true {invalid} {on_input} /> }
}

#[cfg(test)]
mod tests;
