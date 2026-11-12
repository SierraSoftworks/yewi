//! The pure logic for a [`TagPicker`](super::TagPicker)'s list of values.

use yew::AttrValue;

use crate::SelectOption;

/// `values` with `value` removed if it is there, or added at the end if not.
pub(crate) fn toggle(values: &[AttrValue], value: &AttrValue) -> Vec<AttrValue> {
    let mut next: Vec<AttrValue> = values.iter().filter(|held| *held != value).cloned().collect();
    if next.len() == values.len() {
        next.push(value.clone());
    }
    next
}

/// The pieces of typed or pasted text that are values: split on commas and
/// line breaks, trimmed, with empty pieces dropped.
pub(crate) fn parse(text: &str) -> impl Iterator<Item = &str> {
    text.split([',', '\n']).map(str::trim).filter(|piece| !piece.is_empty())
}

/// `values` with typed text added, or `None` when it is already there. Text
/// that names an option, ignoring case, adds that option's value.
pub(crate) fn add(values: &[AttrValue], options: &[SelectOption], typed: &str) -> Option<Vec<AttrValue>> {
    let lower = typed.to_lowercase();
    let value = options
        .iter()
        .find(|option| !option.disabled && (option.value == typed || option.label.to_lowercase() == lower))
        .map_or_else(|| AttrValue::from(typed.to_owned()), |option| option.value.clone());
    (!values.contains(&value)).then(|| {
        let mut next = values.to_vec();
        next.push(value);
        next
    })
}
