//! The option model shared by [`Select`](super::Select) and
//! [`Combobox`](super::Combobox), and the pure logic for moving through it.

use std::{ops::Deref, rc::Rc};

use yew::{AttrValue, html::IntoPropValue};

use crate::IconData;

/// One choice in a [`Select`](super::Select) or [`Combobox`](super::Combobox).
///
/// ```ignore
/// SelectOption::new("eu-west-1", "Ireland").description("eu-west-1").group("Europe")
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct SelectOption {
    /// What `on_change` reports and a form submits. Unique within the list.
    pub value: AttrValue,
    /// The text shown, searched and announced.
    pub label: AttrValue,
    /// A muted second line, for an identifier or a short explanation.
    pub description: Option<AttrValue>,
    /// Drawn before the label, in the list and on the control.
    pub icon: Option<IconData>,
    /// The heading this option sits under. Consecutive options with the same
    /// group share one heading.
    pub group: Option<AttrValue>,
    /// Shown, but cannot be chosen.
    pub disabled: bool,
}

impl SelectOption {
    pub fn new(value: impl Into<AttrValue>, label: impl Into<AttrValue>) -> Self {
        Self { value: value.into(), label: label.into(), description: None, icon: None, group: None, disabled: false }
    }

    /// An option whose value is also its label.
    pub fn plain(value: impl Into<AttrValue>) -> Self {
        let value = value.into();
        Self::new(value.clone(), value)
    }

    pub fn description(mut self, description: impl Into<AttrValue>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn icon(mut self, icon: IconData) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn group(mut self, group: impl Into<AttrValue>) -> Self {
        self.group = Some(group.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// A shared, cheaply cloned list of [`SelectOption`]s. Build it once (in
/// `use_memo`, or from a `Vec` or array) and pass it on every render.
#[derive(Debug, Clone, Default)]
pub struct SelectOptions(Rc<[SelectOption]>);

impl Deref for SelectOptions {
    type Target = [SelectOption];

    fn deref(&self) -> &[SelectOption] {
        &self.0
    }
}

impl PartialEq for SelectOptions {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0) || self.0 == other.0
    }
}

impl FromIterator<SelectOption> for SelectOptions {
    fn from_iter<I: IntoIterator<Item = SelectOption>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl From<Vec<SelectOption>> for SelectOptions {
    fn from(options: Vec<SelectOption>) -> Self {
        Self(options.into())
    }
}

impl<const N: usize> From<[SelectOption; N]> for SelectOptions {
    fn from(options: [SelectOption; N]) -> Self {
        Self(options.into())
    }
}

impl From<Rc<[SelectOption]>> for SelectOptions {
    fn from(options: Rc<[SelectOption]>) -> Self {
        Self(options)
    }
}

impl IntoPropValue<SelectOptions> for Vec<SelectOption> {
    fn into_prop_value(self) -> SelectOptions {
        self.into()
    }
}

impl<const N: usize> IntoPropValue<SelectOptions> for [SelectOption; N] {
    fn into_prop_value(self) -> SelectOptions {
        self.into()
    }
}

impl IntoPropValue<SelectOptions> for Rc<[SelectOption]> {
    fn into_prop_value(self) -> SelectOptions {
        self.into()
    }
}

impl IntoPropValue<SelectOptions> for Rc<SelectOptions> {
    fn into_prop_value(self) -> SelectOptions {
        (*self).clone()
    }
}

/// The option `delta` choosable steps from `from`, within `order` (the
/// options on show, as indices). Stops at the ends; with no starting point it
/// enters from the matching end.
pub(crate) fn step(options: &[SelectOption], order: &[usize], from: Option<usize>, delta: isize) -> Option<usize> {
    let enabled = |index: &usize| !options[*index].disabled;
    let Some(position) = from.and_then(|from| order.iter().position(|index| *index == from)) else {
        return edge(options, order, delta < 0);
    };
    let found = if delta >= 0 {
        order[position + 1..].iter().copied().filter(enabled).take(delta.unsigned_abs()).last()
    } else {
        order[..position].iter().rev().copied().filter(enabled).take(delta.unsigned_abs()).last()
    };
    found.or(from)
}

/// The first (or last) choosable option on show.
pub(crate) fn edge(options: &[SelectOption], order: &[usize], last: bool) -> Option<usize> {
    let mut enabled = order.iter().copied().filter(|index| !options[*index].disabled);
    if last { enabled.next_back() } else { enabled.next() }
}

/// The option a typed prefix lands on. Repeating one letter cycles through
/// the options starting with it; a longer prefix stays on a match.
pub(crate) fn typeahead(options: &[SelectOption], typed: &str, from: Option<usize>) -> Option<usize> {
    let typed = typed.to_lowercase();
    let first = typed.chars().next()?;
    let repeated = typed.chars().all(|c| c == first);
    let needle = if repeated { first.to_string() } else { typed };
    let advance = repeated || needle.chars().count() == 1;
    let start = from.map_or(0, |from| if advance { from + 1 } else { from });
    let count = options.len();
    (0..count)
        .map(|offset| (start + offset) % count)
        .find(|index| !options[*index].disabled && options[*index].label.to_lowercase().starts_with(&needle))
}

/// The options matching `query`, in order: every whitespace-separated term
/// must appear in the label or the description, ignoring case. Returns at
/// most `limit` indices, and how many matched in all.
pub(crate) fn filter(options: &[SelectOption], query: &str, limit: usize) -> (Vec<usize>, usize) {
    let query = query.to_lowercase();
    let terms: Vec<&str> = query.split_whitespace().collect();
    let mut shown = Vec::new();
    let mut total = 0;
    for (index, option) in options.iter().enumerate() {
        let matches = terms.is_empty() || {
            let label = option.label.to_lowercase();
            let description = option.description.as_ref().map(|d| d.to_lowercase());
            terms.iter().all(|term| label.contains(term) || description.as_ref().is_some_and(|d| d.contains(term)))
        };
        if matches {
            total += 1;
            if shown.len() < limit {
                shown.push(index);
            }
        }
    }
    (shown, total)
}
