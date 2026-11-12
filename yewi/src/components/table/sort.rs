//! Sort state and the orderings columns are built from.

use std::cmp::Ordering;

use yew::{AttrValue, html::IntoPropValue};

/// Which way a column is sorted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum SortDirection {
    /// Smallest first: A to Z, oldest to newest.
    #[default]
    Ascending,
    /// Largest first.
    Descending,
}

impl SortDirection {
    /// The other direction.
    pub const fn reversed(self) -> Self {
        match self {
            SortDirection::Ascending => SortDirection::Descending,
            SortDirection::Descending => SortDirection::Ascending,
        }
    }

    /// The `aria-sort` value.
    pub const fn as_str(self) -> &'static str {
        match self {
            SortDirection::Ascending => "ascending",
            SortDirection::Descending => "descending",
        }
    }

    /// Applies the direction to an ascending comparison.
    pub fn apply(self, ordering: Ordering) -> Ordering {
        match self {
            SortDirection::Ascending => ordering,
            SortDirection::Descending => ordering.reverse(),
        }
    }
}

/// One column of a table's sort order.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sort {
    /// The key of the sorted [`Column`](crate::Column).
    pub column: AttrValue,
    pub direction: SortDirection,
}

impl Sort {
    pub fn new(column: impl Into<AttrValue>, direction: SortDirection) -> Self {
        Self { column: column.into(), direction }
    }

    /// `column`, smallest first.
    pub fn asc(column: impl Into<AttrValue>) -> Self {
        Self::new(column, SortDirection::Ascending)
    }

    /// `column`, largest first.
    pub fn desc(column: impl Into<AttrValue>) -> Self {
        Self::new(column, SortDirection::Descending)
    }
}

impl IntoPropValue<Vec<Sort>> for Sort {
    fn into_prop_value(self) -> Vec<Sort> {
        vec![self]
    }
}

impl IntoPropValue<Option<Vec<Sort>>> for Sort {
    fn into_prop_value(self) -> Option<Vec<Sort>> {
        Some(vec![self])
    }
}

/// What activating a sortable header asks for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SortClick {
    /// The direction the column should take next: ascending first, then the
    /// reverse of what it has.
    pub direction: SortDirection,
    /// Shift was held: add this column to the existing sort instead of
    /// replacing it.
    pub extend: bool,
}

/// The sort order after `column` is activated. A column already in the order
/// reverses; a new one starts ascending. With `extend` the rest of the order
/// is kept and a new column becomes the last tie-break; without it the column
/// becomes the only sort.
pub fn next_sort(current: &[Sort], column: &AttrValue, extend: bool) -> Vec<Sort> {
    let existing = current.iter().position(|sort| &sort.column == column);
    let direction = existing.map_or(SortDirection::Ascending, |i| current[i].direction.reversed());
    let next = Sort { column: column.clone(), direction };
    if !extend {
        return vec![next];
    }
    let mut order = current.to_vec();
    match existing {
        Some(i) => order[i] = next,
        None => order.push(next),
    }
    order
}

/// Orders text the way a person would: ignoring case, and comparing runs of
/// digits by value, so `web-2` sorts before `web-10`. Allocates nothing.
pub fn natural_cmp(a: &str, b: &str) -> Ordering {
    let (mut left, mut right) = (a.chars().peekable(), b.chars().peekable());
    loop {
        match (left.peek().copied(), right.peek().copied()) {
            (None, None) => return a.cmp(b),
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(x), Some(y)) if x.is_ascii_digit() && y.is_ascii_digit() => {
                let (x, y) = (digits(&mut left), digits(&mut right));
                let by_value = x.len().cmp(&y.len()).then_with(|| x.cmp(&y));
                if by_value != Ordering::Equal {
                    return by_value;
                }
            }
            (Some(x), Some(y)) => {
                let by_letter = x.to_lowercase().cmp(y.to_lowercase());
                if by_letter != Ordering::Equal {
                    return by_letter;
                }
                left.next();
                right.next();
            }
        }
    }
}

/// Takes a run of digits without its leading zeros. Runs are short, so the
/// small buffer never spills in practice.
fn digits(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> DigitRun {
    let mut run = DigitRun::default();
    while let Some(digit) = chars.peek().copied().filter(char::is_ascii_digit) {
        chars.next();
        run.push(digit as u8);
    }
    run
}

/// A digit run compared by length, then digit by digit.
#[derive(Default, PartialEq, Eq, PartialOrd, Ord)]
struct DigitRun {
    inline: [u8; 24],
    len: usize,
    spilled: Vec<u8>,
}

impl DigitRun {
    fn push(&mut self, digit: u8) {
        if self.len == 0 && digit == b'0' {
            return;
        }
        if self.len < self.inline.len() {
            self.inline[self.len] = digit;
        } else {
            self.spilled.push(digit);
        }
        self.len += 1;
    }

    fn len(&self) -> usize {
        self.len
    }
}
