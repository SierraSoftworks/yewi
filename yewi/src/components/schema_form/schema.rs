//! Reading a JSON Schema well enough to draw a form from it, and editing the
//! document the form holds. No browser in it, so it is tested natively.
//!
//! This is not a validator. It answers one question: which control does each
//! part of the document want? It understands the shapes generators such as
//! `schemars` derive from a settings struct; anything else is
//! [`Kind::Unknown`], which the form draws as a JSON box for that one value.

use yew::AttrValue;

use crate::Json;

/// How deep anything here goes before giving up: `$ref`s followed from one
/// node, levels of defaults built and levels drawn. A schema may contain
/// itself, and one that requires itself has no last level.
pub(super) const MAX_DEPTH: usize = 16;

// --- The document -----------------------------------------------------

pub(super) fn get<'a>(node: &'a Json, key: &str) -> Option<&'a Json> {
    match node {
        Json::Object(entries) => entries.iter().find(|(name, _)| name == key).map(|(_, value)| value),
        _ => None,
    }
}

pub(super) fn text(node: &Json) -> Option<&str> {
    match node {
        Json::String(text) => Some(text),
        _ => None,
    }
}

pub(super) fn number(node: &Json) -> Option<f64> {
    match node {
        Json::Number(text) => text.parse().ok(),
        _ => None,
    }
}

pub(super) fn items(node: &Json) -> Option<&[Json]> {
    match node {
        Json::Array(items) => Some(items),
        _ => None,
    }
}

fn flag(node: &Json, key: &str) -> bool {
    get(node, key) == Some(&Json::Bool(true))
}

/// A number as JSON writes it. `None` for what JSON cannot hold.
pub(super) fn json_number(value: f64, integer: bool) -> Option<Json> {
    let value = if integer { value.round() } else { value };
    let text = if value == 0.0 { "0".to_owned() } else { value.to_string() };
    value.is_finite().then(|| Json::Number(text.into()))
}

/// `pointer` extended by one step, as a JSON Pointer (RFC 6901).
pub(super) fn child(pointer: &str, step: &str) -> AttrValue {
    format!("{pointer}/{}", step.replace('~', "~0").replace('/', "~1")).into()
}

fn steps(pointer: &str) -> Vec<String> {
    pointer.split('/').skip(1).map(|step| step.replace("~1", "/").replace("~0", "~")).collect()
}

/// The value at a JSON Pointer.
pub(super) fn at<'a>(doc: &'a Json, pointer: &str) -> Option<&'a Json> {
    steps(pointer).iter().try_fold(doc, |node, step| match node {
        Json::Array(items) => items.get(step.parse::<usize>().ok()?),
        node => get(node, step),
    })
}

/// `doc` with the value at `pointer` replaced, creating the objects on the
/// way. `None` removes the key (or the list item) rather than storing a null,
/// so that an emptied field is one the reader's own default applies to. An
/// index one past the end of a list appends.
pub(super) fn set_at(doc: &Json, pointer: &str, value: Option<Json>) -> Json {
    set(Some(doc), &steps(pointer), value).unwrap_or(Json::Null)
}

fn set(node: Option<&Json>, path: &[String], value: Option<Json>) -> Option<Json> {
    let Some((step, rest)) = path.split_first() else {
        return value;
    };
    match node {
        Some(Json::Array(items)) => {
            let Ok(index) = step.parse::<usize>() else {
                return node.cloned();
            };
            let mut next = items.to_vec();
            match (set(items.get(index), rest, value), index < next.len()) {
                (Some(item), true) => next[index] = item,
                (Some(item), false) => next.push(item),
                (None, true) => drop(next.remove(index)),
                (None, false) => {}
            }
            Some(Json::Array(next.into()))
        }
        _ => {
            let mut next = match node {
                Some(Json::Object(entries)) => entries.to_vec(),
                _ => Vec::new(),
            };
            let position = next.iter().position(|(name, _)| name == step);
            match (set(position.map(|at| &next[at].1), rest, value), position) {
                (Some(entry), Some(at)) => next[at].1 = entry,
                (Some(entry), None) => next.push((step.clone().into(), entry)),
                (None, Some(at)) => drop(next.remove(at)),
                (None, None) => {}
            }
            Some(Json::Object(next.into()))
        }
    }
}

// --- The schema -------------------------------------------------------

/// How a string is entered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum TextKind {
    Plain,
    /// `format: password` or `writeOnly`.
    Secret,
    /// `format: multiline` (or `textarea`).
    Multiline,
    Email,
    Url,
}

/// What kind of control a schema node wants.
#[derive(Debug, PartialEq)]
pub(super) enum Kind<'a> {
    Object(Vec<Property<'a>>),
    /// A tagged union: a picker, then the chosen variant's fields.
    Variants(Vec<Variant<'a>>),
    /// One of a fixed set of values.
    Choice(Vec<&'a Json>),
    Text(TextKind),
    Integer,
    Number,
    Boolean,
    /// A list of strings: from the given choices, or free-form.
    Tags(Option<Vec<&'a Json>>),
    /// A list whose items all follow one schema.
    List(&'a Json),
    Unknown,
}

#[derive(Debug, PartialEq)]
pub(super) struct Property<'a> {
    pub key: &'a AttrValue,
    pub schema: &'a Json,
    pub required: bool,
}

/// One alternative of a tagged union.
#[derive(Debug, PartialEq)]
pub(super) struct Variant<'a> {
    pub label: String,
    pub schema: &'a Json,
    /// The property whose constant says which variant an object is.
    pub tag: Option<(&'a AttrValue, &'a Json)>,
}

impl Variant<'_> {
    /// This variant's own fields: everything but the tag, which the picker
    /// sets.
    pub fn fields(&self) -> Vec<Property<'_>> {
        properties(self.schema).into_iter().filter(|property| self.tag.is_none_or(|(tag, _)| tag != property.key)).collect()
    }
}

/// Follows local `$ref`s, and the single-entry `allOf` older generators wrap
/// one in to hang a description on it.
pub(super) fn resolve<'a>(root: &'a Json, mut node: &'a Json) -> &'a Json {
    for _ in 0..MAX_DEPTH {
        let next = match (get(node, "$ref").and_then(text), get(node, "allOf").and_then(items)) {
            (Some(target), _) => target.strip_prefix('#').and_then(|pointer| at(root, pointer)),
            (None, Some([only])) => Some(only),
            _ => None,
        };
        match next {
            Some(next) => node = next,
            None => break,
        }
    }
    node
}

/// `node`, resolved, with "or null" taken off: whether a value may be absent
/// is the parent's `required`, not something the control needs to draw.
fn unwrapped<'a>(root: &'a Json, node: &'a Json) -> &'a Json {
    let node = resolve(root, node);
    let is_null = |entry: &Json| get(entry, "type").and_then(text) == Some("null");
    match alternatives(node) {
        Some([only, null]) | Some([null, only]) if is_null(null) && !is_null(only) => resolve(root, only),
        _ => node,
    }
}

fn alternatives(node: &Json) -> Option<&[Json]> {
    get(node, "oneOf").or_else(|| get(node, "anyOf")).and_then(items)
}

/// The one non-null type a node names, if it names one.
fn type_of(node: &Json) -> Option<&str> {
    match get(node, "type")? {
        Json::String(name) => Some(name),
        Json::Array(names) => names.iter().filter_map(text).find(|name| *name != "null"),
        _ => None,
    }
}

fn properties(node: &Json) -> Vec<Property<'_>> {
    let required: Vec<&str> =
        get(node, "required").and_then(items).map(|keys| keys.iter().filter_map(text).collect()).unwrap_or_default();
    match get(node, "properties") {
        Some(Json::Object(entries)) => entries
            .iter()
            .map(|(key, schema)| Property { key, schema, required: required.contains(&key.as_str()) })
            .collect(),
        _ => Vec::new(),
    }
}

/// The constant a variant pins `key` to.
fn constant<'a>(variant: &'a Json, key: &str) -> Option<&'a Json> {
    get(get(get(variant, "properties")?, key)?, "const")
}

/// The variants of a tagged union, when every alternative is an object.
fn variants<'a>(root: &'a Json, entries: &'a [Json]) -> Option<Vec<Variant<'a>>> {
    let resolved: Vec<&Json> = entries.iter().map(|entry| resolve(root, entry)).collect();
    if resolved.is_empty() || resolved.iter().any(|entry| type_of(entry) != Some("object")) {
        return None;
    }

    // The tag is the property every alternative pins to a constant.
    let tag = properties(resolved[0])
        .into_iter()
        .map(|property| property.key)
        .find(|key| resolved.iter().all(|entry| constant(entry, key).is_some()));

    Some(
        resolved
            .into_iter()
            .enumerate()
            .map(|(index, schema)| {
                let tag = tag.and_then(|key| Some((key, constant(schema, key)?)));
                let label = get(schema, "title")
                    .and_then(text)
                    .map(str::to_owned)
                    .or_else(|| tag.and_then(|(_, value)| text(value)).map(humanise))
                    .unwrap_or_else(|| format!("Option {}", index + 1));
                Variant { label, schema, tag }
            })
            .collect(),
    )
}

/// What kind of control `node` wants.
pub(super) fn kind<'a>(root: &'a Json, node: &'a Json) -> Kind<'a> {
    let node = unwrapped(root, node);

    if let Some(values) = get(node, "enum").and_then(items) {
        return Kind::Choice(values.iter().filter(|value| **value != Json::Null).collect());
    }
    if let Some(entries) = alternatives(node) {
        let constants: Vec<&Json> = entries.iter().filter_map(|entry| get(entry, "const")).collect();
        return match variants(root, entries) {
            _ if !entries.is_empty() && constants.len() == entries.len() => Kind::Choice(constants),
            Some(variants) => Kind::Variants(variants),
            None => Kind::Unknown,
        };
    }

    match type_of(node) {
        Some("object") if get(node, "properties").is_some() => Kind::Object(properties(node)),
        Some("string") => Kind::Text(match get(node, "format").and_then(text) {
            Some("password") => TextKind::Secret,
            _ if flag(node, "writeOnly") => TextKind::Secret,
            Some("multiline" | "textarea") => TextKind::Multiline,
            Some("email") => TextKind::Email,
            Some("uri" | "url") => TextKind::Url,
            _ => TextKind::Plain,
        }),
        Some("integer") => Kind::Integer,
        Some("number") => Kind::Number,
        Some("boolean") => Kind::Boolean,
        Some("array") => match get(node, "items") {
            Some(item) => match kind(root, item) {
                Kind::Choice(values) if values.iter().all(|value| text(value).is_some()) => Kind::Tags(Some(values)),
                Kind::Text(TextKind::Plain) => Kind::Tags(None),
                _ => Kind::List(item),
            },
            None => Kind::Unknown,
        },
        _ => Kind::Unknown,
    }
}

/// What to call one value of a [`Kind::Choice`]: the `title` beside its
/// `const` when the schema offers one, otherwise the value itself.
pub(super) fn choice_label(root: &Json, node: &Json, value: &Json) -> String {
    alternatives(unwrapped(root, node))
        .and_then(|entries| entries.iter().find(|entry| get(entry, "const") == Some(value)))
        .and_then(|entry| get(entry, "title"))
        .or(Some(value))
        .and_then(text)
        .map_or_else(|| value.to_string(), str::to_owned)
}

/// A keyword of `node`, looked for on the node itself before what it refers
/// to: a `description` beside a `$ref` is about this use of the type.
pub(super) fn keyword<'a>(root: &'a Json, node: &'a Json, name: &str) -> Option<&'a Json> {
    get(node, name).or_else(|| get(unwrapped(root, node), name))
}

/// The first paragraph of the `description`, unwrapped. Generators carry a
/// whole doc comment across, and what follows its summary is written for
/// somebody reading the source, not a form.
pub(super) fn summary(root: &Json, node: &Json) -> Option<String> {
    let description = keyword(root, node, "description").and_then(text)?;
    let summary = description.split("\n\n").next().unwrap_or_default();
    Some(summary.split_whitespace().collect::<Vec<_>>().join(" ")).filter(|summary| !summary.is_empty())
}

/// The value a newly added part starts with: the schema's own `default`
/// where it has one, and otherwise the emptiest thing of the right shape.
pub(super) fn default_for(root: &Json, node: &Json) -> Json {
    default_at(root, node, 0)
}

fn default_at(root: &Json, node: &Json, depth: usize) -> Json {
    if depth > MAX_DEPTH {
        return Json::Null;
    }
    if let Some(given) = keyword(root, node, "default").or_else(|| keyword(root, node, "const")) {
        return given.clone();
    }
    match kind(root, node) {
        Kind::Object(properties) => Json::Object(
            properties
                .iter()
                .filter(|property| property.required)
                .map(|property| (property.key.clone(), default_at(root, property.schema, depth + 1)))
                .collect(),
        ),
        Kind::Variants(variants) => {
            variants.first().map_or(Json::Null, |first| default_at(root, first.schema, depth + 1))
        }
        Kind::Choice(values) => values.first().copied().cloned().unwrap_or(Json::Null),
        Kind::Text(_) => Json::String(AttrValue::Static("")),
        Kind::Integer | Kind::Number => {
            keyword(root, node, "minimum").cloned().unwrap_or(Json::Number(AttrValue::Static("0")))
        }
        Kind::Boolean => Json::Bool(false),
        Kind::Tags(_) | Kind::List(_) => Json::Array([].into()),
        Kind::Unknown => Json::Null,
    }
}

/// Which variant `value` is: by its tag, or failing that by its keys.
pub(super) fn variant_of(variants: &[Variant<'_>], value: &Json) -> Option<usize> {
    variants.iter().position(|variant| match variant.tag {
        Some((key, constant)) => get(value, key) == Some(constant),
        None => properties(variant.schema)
            .iter()
            .filter(|property| property.required)
            .all(|property| get(value, property.key).is_some()),
    })
}

/// A fresh value of `variant`, keeping whatever `previous` held under the
/// same name and type, so that switching a circle to a box and back does not
/// cost somebody the coordinates they had typed.
pub(super) fn switched(root: &Json, variant: &Variant<'_>, previous: Option<&Json>) -> Json {
    let fresh = default_for(root, variant.schema);
    let (Json::Object(entries), Some(previous)) = (&fresh, previous) else {
        return fresh;
    };
    let kept = entries.iter().map(|(key, new)| {
        let carried = get(previous, key)
            .filter(|_| variant.tag.is_none_or(|(tag, _)| tag != key))
            .filter(|old| std::mem::discriminant(*old) == std::mem::discriminant(new));
        (key.clone(), carried.unwrap_or(new).clone())
    });
    Json::Object(kept.collect())
}

/// `radius_km` as a label: "Radius km".
pub(super) fn humanise(key: &str) -> String {
    let spaced = key.replace(['_', '-'], " ");
    let mut letters = spaced.chars();
    match letters.next() {
        Some(first) => first.to_uppercase().chain(letters).collect(),
        None => spaced,
    }
}
