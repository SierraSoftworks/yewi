//! Line diffing and unified-diff parsing: pure functions, no rendering.

use std::{collections::HashMap, ops::Range};

use yew::AttrValue;

/// What one row of a diff is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiffLineKind {
    /// Present on both sides.
    Context,
    /// Only in the new text.
    Added,
    /// Only in the old text.
    Removed,
    /// A hunk header: `@@ -1,3 +1,4 @@`.
    Hunk,
    /// A file header: `--- a/file` or `+++ b/file`.
    File,
    /// Anything else a patch carries, such as `\ No newline at end of file`.
    Meta,
}

/// One row of a diff, as [`diff_lines`] and [`parse_unified_diff`] produce it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffLine {
    pub kind: DiffLineKind,
    /// The line without its `+`, `-` or space marker. Header and meta rows
    /// keep their whole text.
    pub text: AttrValue,
    /// 1-based line number in the old text.
    pub old: Option<u32>,
    /// 1-based line number in the new text.
    pub new: Option<u32>,
}

impl DiffLine {
    fn new(kind: DiffLineKind, text: &str, old: Option<u32>, new: Option<u32>) -> Self {
        Self { kind, text: AttrValue::from(text.to_owned()), old, new }
    }

    pub(super) fn is_change(&self) -> bool {
        matches!(self.kind, DiffLineKind::Added | DiffLineKind::Removed)
    }
}

/// Beyond this many edits the shortest script is not worth its memory; the
/// differing middle is then shown as removed followed by added.
const MAX_EDITS: usize = 1500;

/// Compares two texts line by line. Every line of both is returned, in
/// order, as context, removed or added; [`Diff`](super::Diff) folds the long
/// unchanged runs when drawing.
pub fn diff_lines(before: &str, after: &str) -> Vec<DiffLine> {
    let a: Vec<&str> = before.lines().collect();
    let b: Vec<&str> = after.lines().collect();

    let prefix = a.iter().zip(&b).take_while(|(x, y)| x == y).count();
    let suffix = a[prefix..].iter().rev().zip(b[prefix..].iter().rev()).take_while(|(x, y)| x == y).count();
    let (mid_a, mid_b) = (&a[prefix..a.len() - suffix], &b[prefix..b.len() - suffix]);

    let mut out = Vec::with_capacity(a.len().max(b.len()) + mid_a.len().min(mid_b.len()));
    let (mut old, mut new) = (0_u32, 0_u32);
    let mut push = |edit: Edit, text: &str| {
        let (kind, o, n) = match edit {
            Edit::Keep => (DiffLineKind::Context, true, true),
            Edit::Remove => (DiffLineKind::Removed, true, false),
            Edit::Add => (DiffLineKind::Added, false, true),
        };
        old += u32::from(o);
        new += u32::from(n);
        out.push(DiffLine::new(kind, text, o.then_some(old), n.then_some(new)));
    };

    a[..prefix].iter().for_each(|line| push(Edit::Keep, line));
    let (mut i, mut j) = (0, 0);
    for edit in edit_script(mid_a, mid_b) {
        match edit {
            Edit::Keep => {
                push(edit, mid_a[i]);
                i += 1;
                j += 1;
            }
            Edit::Remove => {
                push(edit, mid_a[i]);
                i += 1;
            }
            Edit::Add => {
                push(edit, mid_b[j]);
                j += 1;
            }
        }
    }
    a[a.len() - suffix..].iter().for_each(|line| push(Edit::Keep, line));
    out
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Edit {
    Keep,
    Remove,
    Add,
}

/// The shortest edit script from `a` to `b` (Myers, O(ND)), with removals
/// ordered before additions inside each changed block.
fn edit_script<'a>(a: &[&'a str], b: &[&'a str]) -> Vec<Edit> {
    // Compare interned ids rather than strings.
    let mut ids: HashMap<&'a str, u32> = HashMap::new();
    let mut intern = |line: &&'a str| {
        let next = ids.len() as u32;
        *ids.entry(*line).or_insert(next)
    };
    let a: Vec<u32> = a.iter().map(&mut intern).collect();
    let b: Vec<u32> = b.iter().map(&mut intern).collect();

    let mut script = shortest_path(&a, &b).unwrap_or_else(|| {
        let mut all = vec![Edit::Remove; a.len()];
        all.resize(a.len() + b.len(), Edit::Add);
        all
    });

    // Within a run of changes, show what was removed before what replaced it.
    for block in script.chunk_by_mut(|x, y| *x != Edit::Keep && *y != Edit::Keep) {
        block.sort_by_key(|edit| *edit == Edit::Add);
    }
    script
}

fn shortest_path(a: &[u32], b: &[u32]) -> Option<Vec<Edit>> {
    let (n, m) = (a.len() as isize, b.len() as isize);
    let cap = ((n + m) as usize).min(MAX_EDITS) as isize;
    let off = cap + 1;
    // `v[off + k]` is the furthest x reached on diagonal k = x - y.
    let mut v = vec![0_isize; (2 * cap + 3) as usize];
    // `trace[d]` is `v` for diagonals -(d+1)..=(d+1) as round d began.
    let mut trace: Vec<Vec<isize>> = Vec::new();

    let step = |v: &[isize], off: isize, k: isize, d: isize| -> bool {
        k == -d || (k != d && v[(off + k - 1) as usize] < v[(off + k + 1) as usize])
    };

    let mut found = None;
    'search: for d in 0..=cap {
        trace.push(v[(off - d - 1) as usize..=(off + d + 1) as usize].to_vec());
        for k in (-d..=d).step_by(2) {
            let mut x = if step(&v, off, k, d) { v[(off + k + 1) as usize] } else { v[(off + k - 1) as usize] + 1 };
            let mut y = x - k;
            while x < n && y < m && a[x as usize] == b[y as usize] {
                x += 1;
                y += 1;
            }
            v[(off + k) as usize] = x;
            if x >= n && y >= m {
                found = Some(d);
                break 'search;
            }
        }
    }

    let mut script = Vec::with_capacity((n + m) as usize);
    let (mut x, mut y) = (n, m);
    for d in (0..=found?).rev() {
        let v = &trace[d as usize];
        let off = d + 1;
        let k = x - y;
        let down = step(v, off, k, d);
        let prev_k = if down { k + 1 } else { k - 1 };
        let prev_x = if d == 0 { 0 } else { v[(off + prev_k) as usize] };
        let prev_y = if d == 0 { 0 } else { prev_x - prev_k };
        while x > prev_x && y > prev_y {
            script.push(Edit::Keep);
            x -= 1;
            y -= 1;
        }
        if d > 0 {
            script.push(if down { Edit::Add } else { Edit::Remove });
        }
        (x, y) = (prev_x, prev_y);
    }
    script.reverse();
    Some(script)
}

/// Reads a unified diff (`diff -u`, `git diff`) into rows. Lenient: text
/// with no hunk headers is still classified by its `+` and `-` markers.
pub fn parse_unified_diff(unified: &str) -> Vec<DiffLine> {
    let mut out = Vec::new();
    // Lines of the current hunk still to come, and the next line numbers.
    let (mut left_old, mut left_new) = (0_u32, 0_u32);
    let (mut old, mut new) = (None::<u32>, None::<u32>);

    for line in unified.lines() {
        let in_hunk = left_old > 0 || left_new > 0;
        let marker = line.chars().next();
        let body = line.get(1..).unwrap_or_default();
        let bump = |n: &mut Option<u32>| {
            let current = *n;
            *n = n.map(|v| v + 1);
            current
        };

        if !in_hunk && (line.starts_with("--- ") || line.starts_with("+++ ")) {
            out.push(DiffLine::new(DiffLineKind::File, line, None, None));
            (old, new) = (None, None);
        } else if !in_hunk && line.starts_with("@@") {
            let ((o, lo), (n, ln)) = hunk_ranges(line).unwrap_or(((0, 0), (0, 0)));
            (left_old, left_new) = (lo, ln);
            (old, new) = (Some(o), Some(n));
            out.push(DiffLine::new(DiffLineKind::Hunk, line, None, None));
        } else if marker == Some('+') {
            left_new = left_new.saturating_sub(1);
            out.push(DiffLine::new(DiffLineKind::Added, body, None, bump(&mut new)));
        } else if marker == Some('-') {
            left_old = left_old.saturating_sub(1);
            out.push(DiffLine::new(DiffLineKind::Removed, body, bump(&mut old), None));
        } else if marker == Some('\\') || (!in_hunk && marker != Some(' ')) {
            out.push(DiffLine::new(DiffLineKind::Meta, line, None, None));
        } else {
            // A context line; some tools strip the lone space of an empty one.
            left_old = left_old.saturating_sub(1);
            left_new = left_new.saturating_sub(1);
            let text = if marker == Some(' ') { body } else { line };
            out.push(DiffLine::new(DiffLineKind::Context, text, bump(&mut old), bump(&mut new)));
        }
    }
    out
}

/// `((old start, old count), (new start, new count))` of `@@ -a,b +c,d @@`.
fn hunk_ranges(header: &str) -> Option<((u32, u32), (u32, u32))> {
    let mut parts = header.split_whitespace().skip(1);
    let range = |part: &str, sign: char| -> Option<(u32, u32)> {
        let part = part.strip_prefix(sign)?;
        let (start, count) = part.split_once(',').unwrap_or((part, "1"));
        Some((start.parse().ok()?, count.parse().ok()?))
    };
    Some((range(parts.next()?, '-')?, range(parts.next()?, '+')?))
}

/// The file a patch header names: `--- a/src/x.rs` gives `src/x.rs`.
pub(super) fn file_name(header: &str) -> &str {
    let name = header.get(4..).unwrap_or_default();
    let name = name.split('\t').next().unwrap_or_default().trim();
    name.strip_prefix("a/").or_else(|| name.strip_prefix("b/")).unwrap_or(name)
}

/// The byte ranges of `old` and `new` that differ, when the two lines share
/// enough of a prefix and suffix to be read as one edited line.
pub(super) fn changed_spans(old: &str, new: &str) -> Option<(Range<usize>, Range<usize>)> {
    if old == new {
        return None;
    }
    let prefix: usize = old.chars().zip(new.chars()).take_while(|(a, b)| a == b).map(|(a, _)| a.len_utf8()).sum();
    let (rest_old, rest_new) = (&old[prefix..], &new[prefix..]);
    let suffix: usize =
        rest_old.chars().rev().zip(rest_new.chars().rev()).take_while(|(a, b)| a == b).map(|(a, _)| a.len_utf8()).sum();

    let solid = |text: &str| text.chars().filter(|c| !c.is_whitespace()).count();
    let shared = solid(&old[..prefix]) + solid(&old[old.len() - suffix..]);
    if shared == 0 || shared * 2 < solid(old).max(solid(new)) {
        return None;
    }
    Some((prefix..old.len() - suffix, prefix..new.len() - suffix))
}
