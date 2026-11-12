//! The hierarchy a [`TreeSelect`](super::TreeSelect) picks from, the search
//! across it, and which rows are on show. Nothing here touches the browser.

use std::{collections::HashMap, rc::Rc};

use yew::{AttrValue, html::IntoPropValue};

use crate::IconData;

/// One entry of a hierarchy, placed by the names that lead to it.
///
/// ```ignore
/// TreeOption::new(["Europe", "Ireland", "Dublin"], "dub").detail("eu-west-1")
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TreeOption {
    /// The names that lead here, outermost first, ending in this entry's own.
    pub path: Vec<AttrValue>,
    /// What `on_change` reports and a form submits. `None` for a branch that
    /// only holds others: it can be opened but not chosen.
    pub value: Option<AttrValue>,
    /// Shown small beside the name and searched: a code, for those who know it.
    pub detail: Option<AttrValue>,
    /// Drawn before the name.
    pub icon: Option<IconData>,
    /// Handed to the select's `render_media` to draw a picture before the
    /// name, for entries an icon cannot stand for.
    pub media: Option<AttrValue>,
}

impl TreeOption {
    /// An entry that can be chosen.
    pub fn new<P: Into<AttrValue>>(path: impl IntoIterator<Item = P>, value: impl Into<AttrValue>) -> Self {
        Self { value: Some(value.into()), ..Self::branch(path) }
    }

    /// A branch that only holds other entries. Branches on the way to an
    /// entry are made for you; list one only to give it an icon or a detail.
    pub fn branch<P: Into<AttrValue>>(path: impl IntoIterator<Item = P>) -> Self {
        Self { path: path.into_iter().map(Into::into).collect(), ..Self::default() }
    }

    pub fn detail(mut self, detail: impl Into<AttrValue>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    pub fn icon(mut self, icon: IconData) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn media(mut self, media: impl Into<AttrValue>) -> Self {
        self.media = Some(media.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Node {
    pub label: AttrValue,
    pub value: Option<AttrValue>,
    pub detail: Option<AttrValue>,
    pub icon: Option<IconData>,
    pub media: Option<AttrValue>,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    /// Everything a search looks through, lowercased once: the names that
    /// lead here, this one's own, and its detail.
    haystack: String,
    label_lower: String,
    depth: usize,
}

#[derive(Debug, Default, PartialEq)]
pub(crate) struct Tree {
    nodes: Vec<Node>,
    roots: Vec<usize>,
    by_value: HashMap<AttrValue, usize>,
}

impl Tree {
    fn build(entries: impl IntoIterator<Item = TreeOption>) -> Self {
        let mut tree = Self::default();
        let mut known: HashMap<(Option<usize>, AttrValue), usize> = HashMap::new();

        for entry in entries {
            let mut parent = None;
            for step in &entry.path {
                let at = *known.entry((parent, step.clone())).or_insert_with(|| tree.push(step.clone(), parent));
                parent = Some(at);
            }
            let Some(at) = parent else { continue };
            if let Some(value) = &entry.value {
                tree.by_value.entry(value.clone()).or_insert(at);
            }
            let node = &mut tree.nodes[at];
            if let Some(detail) = &entry.detail {
                node.haystack.push(' ');
                node.haystack.push_str(&detail.to_lowercase());
            }
            node.value = entry.value;
            node.detail = entry.detail;
            node.icon = entry.icon;
            node.media = entry.media;
        }
        tree
    }

    /// Adds a step nothing has been said about yet, under `parent`.
    fn push(&mut self, label: AttrValue, parent: Option<usize>) -> usize {
        let at = self.nodes.len();
        let label_lower = label.to_lowercase();
        let (haystack, depth) = match parent.map(|parent| &self.nodes[parent]) {
            Some(above) => (format!("{} {label_lower}", above.haystack), above.depth + 1),
            None => (label_lower.clone(), 0),
        };
        self.nodes.push(Node {
            label,
            value: None,
            detail: None,
            icon: None,
            media: None,
            parent,
            children: Vec::new(),
            haystack,
            label_lower,
            depth,
        });
        match parent {
            Some(parent) => self.nodes[parent].children.push(at),
            None => self.roots.push(at),
        }
        at
    }

    pub fn node(&self, at: usize) -> Option<&Node> {
        self.nodes.get(at)
    }

    /// What is directly inside `branch`, or at the top for `None`.
    pub fn level(&self, branch: Option<usize>) -> &[usize] {
        match branch.and_then(|at| self.nodes.get(at)) {
            Some(node) => &node.children,
            None => &self.roots,
        }
    }

    /// The entry whose value is `value`.
    pub fn find(&self, value: &str) -> Option<usize> {
        self.by_value.get(value).copied()
    }

    /// The branches that lead to `at`, outermost first, without `at` itself.
    pub fn trail(&self, at: usize) -> Vec<usize> {
        let mut trail = Vec::new();
        let mut next = self.nodes.get(at).and_then(|node| node.parent);
        while let Some(parent) = next {
            trail.push(parent);
            next = self.nodes[parent].parent;
        }
        trail.reverse();
        trail
    }

    /// The entries that can be chosen and that hold every word of `query`,
    /// best first: a name that starts with what was typed, then one that
    /// contains it, then one reached only through the names above it or its
    /// detail; within each, the broader entry before the narrower. Returns
    /// at most `limit`, and how many matched in all.
    pub fn search(&self, query: &str, limit: usize) -> (Vec<usize>, usize) {
        let query = query.to_lowercase();
        let words: Vec<&str> = query.split_whitespace().collect();
        if words.is_empty() {
            return (Vec::new(), 0);
        }
        let mut found: Vec<(u8, usize, usize)> = self
            .nodes
            .iter()
            .enumerate()
            .filter(|(_, node)| node.value.is_some() && words.iter().all(|word| node.haystack.contains(word)))
            .map(|(at, node)| {
                let rank = if node.label_lower.starts_with(words[0]) {
                    0
                } else if words.iter().all(|word| node.label_lower.contains(word)) {
                    1
                } else {
                    2
                };
                (rank, node.depth, at)
            })
            .collect();
        let total = found.len();
        found.sort_unstable();
        (found.into_iter().take(limit).map(|(_, _, at)| at).collect(), total)
    }
}

/// A shared, cheaply cloned hierarchy of [`TreeOption`]s. Build it once (in
/// `use_memo`, or from a `Vec` or array) and pass it on every render.
///
/// Entries keep the order they were listed in. A step on the way to an entry
/// that was never listed itself becomes a branch: it can be opened, but not
/// chosen.
#[derive(Debug, Clone, Default)]
pub struct TreeOptions(Rc<Tree>);

impl TreeOptions {
    /// How many entries and branches the hierarchy holds.
    pub fn len(&self) -> usize {
        self.0.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.nodes.is_empty()
    }

    pub(crate) fn tree(&self) -> &Tree {
        &self.0
    }
}

impl PartialEq for TreeOptions {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0) || self.0 == other.0
    }
}

impl FromIterator<TreeOption> for TreeOptions {
    fn from_iter<I: IntoIterator<Item = TreeOption>>(iter: I) -> Self {
        Self(Rc::new(Tree::build(iter)))
    }
}

impl From<Vec<TreeOption>> for TreeOptions {
    fn from(options: Vec<TreeOption>) -> Self {
        options.into_iter().collect()
    }
}

impl<const N: usize> From<[TreeOption; N]> for TreeOptions {
    fn from(options: [TreeOption; N]) -> Self {
        options.into_iter().collect()
    }
}

impl IntoPropValue<TreeOptions> for Vec<TreeOption> {
    fn into_prop_value(self) -> TreeOptions {
        self.into()
    }
}

impl<const N: usize> IntoPropValue<TreeOptions> for [TreeOption; N] {
    fn into_prop_value(self) -> TreeOptions {
        self.into()
    }
}

impl IntoPropValue<TreeOptions> for Rc<TreeOptions> {
    fn into_prop_value(self) -> TreeOptions {
        (*self).clone()
    }
}

/// One row of the open list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Row {
    /// Go back to having nothing chosen.
    Clear,
    /// Use what was typed as it stands: the caller said it is well formed.
    Custom(AttrValue),
    /// An entry or branch of the tree.
    Node(usize),
}

/// The rows for `query` or, with nothing typed, for the inside of `branch`,
/// and how many entries matched in all.
///
/// Two modes, decided by whether anything has been typed. With nothing typed
/// the list is one level of the hierarchy, to walk down. With something typed
/// it is the best matches from the whole tree.
pub(crate) fn rows(
    tree: &Tree,
    query: &str,
    branch: Option<usize>,
    clearable: bool,
    custom: Option<AttrValue>,
    limit: usize,
) -> (Vec<Row>, usize) {
    let searching = !query.trim().is_empty();
    let (found, total) = if searching {
        tree.search(query, limit)
    } else {
        let level = tree.level(branch);
        (level.to_vec(), level.len())
    };
    // What was typed is only worth offering when the tree has no such value:
    // otherwise the same thing would be listed twice, once without its name.
    let custom = custom.filter(|value| searching && tree.find(value).is_none());
    let clear = (clearable && !searching && branch.is_none()).then_some(Row::Clear);
    (clear.into_iter().chain(found.into_iter().map(Row::Node)).chain(custom.map(Row::Custom)).collect(), total)
}
