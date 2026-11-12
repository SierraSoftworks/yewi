//! What to suggest at the caret. Pure: no browser, no state.

use std::rc::Rc;

use yew::AttrValue;

/// A property a query can be scoped to, and the values worth suggesting for
/// it.
#[derive(Debug, Clone, PartialEq)]
pub struct QueryField {
    /// The name as it is typed in a query.
    pub name: AttrValue,
    /// A short explanation shown beside the name.
    pub description: Option<AttrValue>,
    /// The values offered once a term is scoped to this field. Shown in the
    /// order given, so sort them the way they should be offered.
    pub values: Rc<[AttrValue]>,
}

impl QueryField {
    pub fn new(name: impl Into<AttrValue>) -> Self {
        Self { name: name.into(), description: None, values: Rc::from([]) }
    }

    /// Adds the explanation shown beside the name.
    pub fn description(mut self, description: impl Into<AttrValue>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the values to suggest.
    pub fn values<V: Into<AttrValue>>(mut self, values: impl IntoIterator<Item = V>) -> Self {
        self.values = values.into_iter().map(Into::into).collect();
        self
    }
}

/// The shape of the query language, which decides where fields and values
/// can appear and how a completed one is written.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum QuerySyntax {
    /// Space-separated search terms, each optionally scoped with
    /// `field:value`. A value with spaces is quoted.
    #[default]
    Terms,
    /// A boolean expression such as `browser == "Chrome" && country != "DE"`.
    /// Fields are offered at the start and after `&&`, `||`, `!` and `(`;
    /// values after a field and an operator.
    Expression,
}

impl QuerySyntax {
    pub const ALL: [QuerySyntax; 2] = [QuerySyntax::Terms, QuerySyntax::Expression];
}

/// One entry in the suggestion list.
#[derive(Debug, Clone, PartialEq)]
pub struct QuerySuggestion {
    /// What the entry shows: a field (`status:`) or a value.
    pub label: AttrValue,
    /// The field's description. Values have none.
    pub description: Option<AttrValue>,
    /// The text that replaces the token at the caret.
    pub insert: String,
}

/// The suggestions for the token at the caret, and the byte range of the
/// text they replace.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct QueryCompletion {
    pub start: usize,
    pub end: usize,
    pub items: Vec<QuerySuggestion>,
}

impl QueryCompletion {
    /// The query after accepting `items[index]`, and the byte offset the
    /// caret moves to.
    pub fn apply(&self, text: &str, index: usize) -> Option<(String, usize)> {
        let item = self.items.get(index)?;
        let (head, tail) = (text.get(..self.start)?, text.get(self.end..)?);
        // A finished term ends with a space: reuse the one already there.
        let next = tail.chars().next().filter(|c| c.is_whitespace());
        let reuse = next.filter(|_| item.insert.ends_with(' '));
        let insert = if reuse.is_some() { &item.insert[..item.insert.len() - 1] } else { item.insert.as_str() };
        let caret = head.len() + insert.len() + reuse.map_or(0, char::len_utf8);
        Some((format!("{head}{insert}{tail}"), caret))
    }
}

/// Works out what to suggest for `text` with the caret at byte offset
/// `caret`, offering at most `max` entries.
pub fn complete_query(
    text: &str,
    caret: usize,
    fields: &[QueryField],
    syntax: QuerySyntax,
    max: usize,
) -> QueryCompletion {
    let caret = floor_boundary(text, caret);
    if fields.is_empty() || max == 0 {
        return QueryCompletion { start: caret, end: caret, items: Vec::new() };
    }
    match syntax {
        QuerySyntax::Terms => complete_terms(text, caret, fields, max),
        QuerySyntax::Expression => complete_expression(text, caret, fields, max),
    }
}

fn floor_boundary(text: &str, index: usize) -> usize {
    let mut index = index.min(text.len());
    while !text.is_char_boundary(index) {
        index -= 1;
    }
    index
}

fn find_field<'a>(fields: &'a [QueryField], name: &str) -> Option<&'a QueryField> {
    fields.iter().find(|field| field.name.eq_ignore_ascii_case(name))
}

fn field_items<'a>(
    fields: &'a [QueryField],
    partial: &str,
    max: usize,
    render: impl Fn(&QueryField) -> (String, String) + 'a,
) -> Vec<QuerySuggestion> {
    let needle = partial.to_lowercase();
    fields
        .iter()
        .filter(|field| field.name.to_lowercase().starts_with(&needle))
        .take(max)
        .map(|field| {
            let (label, insert) = render(field);
            QuerySuggestion { label: label.into(), description: field.description.clone(), insert }
        })
        .filter(|item: &QuerySuggestion| item.label != partial)
        .collect()
}

/// The field's values containing `partial`, those starting with it first.
fn value_items(
    field: &QueryField,
    partial: &str,
    max: usize,
    render: impl Fn(&str) -> String,
) -> Vec<QuerySuggestion> {
    let needle = partial.to_lowercase();
    let item = |value: &AttrValue| QuerySuggestion { label: value.clone(), description: None, insert: render(value) };
    let mut items: Vec<QuerySuggestion> =
        field.values.iter().filter(|value| value.to_lowercase().starts_with(&needle)).take(max).map(item).collect();
    if items.len() < max && !needle.is_empty() {
        let rest = field.values.iter().filter(|value| {
            let value = value.to_lowercase();
            !value.starts_with(&needle) && value.contains(&needle)
        });
        let room = max - items.len();
            items.extend(rest.take(room).map(item));
    }
    // Nothing left to complete.
    if matches!(items.as_slice(), [only] if only.label == partial) {
        items.clear();
    }
    items
}

fn quoted(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for c in value.chars() {
        if matches!(c, '"' | '\\') {
            out.push('\\');
        }
        out.push(c);
    }
    out.push('"');
    out
}

/// The partial value typed so far, without its opening (and any closing) quote.
fn unquoted(token: &str) -> &str {
    let Some(quote) = token.chars().next().filter(|c| matches!(c, '"' | '\'')) else {
        return token;
    };
    let inner = &token[1..];
    inner.strip_suffix(quote).unwrap_or(inner)
}

// ---------------------------------------------------------------- terms

/// Where the whitespace-separated token around `caret` starts and ends.
/// Whitespace inside quotes does not end a token.
fn term_bounds(text: &str, caret: usize) -> (usize, usize) {
    let (mut start, mut quote) = (0, None::<char>);
    for (at, c) in text[..caret].char_indices() {
        match quote {
            Some(open) if c == open => quote = None,
            Some(_) => {}
            None if matches!(c, '"' | '\'') => quote = Some(c),
            None if c.is_whitespace() => start = at + c.len_utf8(),
            None => {}
        }
    }
    let mut end = text.len();
    for (at, c) in text[caret..].char_indices() {
        match quote {
            Some(open) if c == open => quote = None,
            Some(_) => {}
            None if matches!(c, '"' | '\'') => quote = Some(c),
            None if c.is_whitespace() => {
                end = caret + at;
                break;
            }
            None => {}
        }
    }
    (start, end)
}

fn complete_terms(text: &str, caret: usize, fields: &[QueryField], max: usize) -> QueryCompletion {
    let (start, end) = term_bounds(text, caret);
    let typed = &text[start..caret];
    let items = match typed.split_once(':') {
        Some((name, partial)) => find_field(fields, name).map_or_else(Vec::new, |field| {
            value_items(field, unquoted(partial), max, |value| {
                let needs_quotes = value.is_empty() || value.contains(|c: char| c.is_whitespace() || c == '"');
                let value = if needs_quotes { quoted(value) } else { value.to_owned() };
                format!("{}:{value} ", field.name)
            })
        }),
        // A scope is not a finished term, so no space follows it: accepting
        // one goes straight on to its values.
        None => field_items(fields, typed, max, |field| (format!("{}:", field.name), format!("{}:", field.name))),
    };
    QueryCompletion { start, end, items }
}

// ----------------------------------------------------------- expression

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Word,
    Text,
    Open,
    Close,
    Operator,
}

#[derive(Debug, Clone, Copy)]
struct Token<'a> {
    kind: Kind,
    start: usize,
    text: &'a str,
    /// For `Text`: whether the closing quote was seen.
    closed: bool,
}

impl Token<'_> {
    fn end(&self) -> usize {
        self.start + self.text.len()
    }

    /// A string still being typed: its closing quote is missing.
    fn is_open_text(&self) -> bool {
        self.kind == Kind::Text && !self.closed
    }

    /// Whether a new operand follows this token: `&&`, `||`, `!`, `(`, `,`
    /// and their spelled-out forms.
    fn joins(&self) -> bool {
        match self.kind {
            Kind::Open => true,
            Kind::Operator => self.text.chars().all(|c| matches!(c, '&' | '|')) || self.text == "!",
            Kind::Word => ["and", "or", "not"].iter().any(|word| self.text.eq_ignore_ascii_case(word)),
            Kind::Text | Kind::Close => false,
        }
    }
}

fn is_word(c: char) -> bool {
    c.is_alphanumeric() || matches!(c, '_' | '.' | '$' | '@')
}

fn lex(text: &str) -> Vec<Token<'_>> {
    let mut tokens = Vec::new();
    let mut chars = text.char_indices().peekable();
    while let Some(&(start, c)) = chars.peek() {
        let mut closed = false;
        let kind = if c.is_whitespace() {
            chars.next();
            continue;
        } else if matches!(c, '"' | '\'') {
            chars.next();
            let mut escaped = false;
            for (_, next) in chars.by_ref() {
                if escaped {
                    escaped = false;
                } else if next == '\\' {
                    escaped = true;
                } else if next == c {
                    closed = true;
                    break;
                }
            }
            Kind::Text
        } else if matches!(c, '(' | '[' | ',') {
            chars.next();
            Kind::Open
        } else if matches!(c, ')' | ']') {
            chars.next();
            Kind::Close
        } else if is_word(c) {
            while chars.next_if(|&(_, next)| is_word(next)).is_some() {}
            Kind::Word
        } else {
            let other = |c: char| !c.is_whitespace() && !is_word(c) && !matches!(c, '"' | '\'' | '(' | ')' | '[' | ']' | ',');
            while chars.next_if(|&(_, next)| other(next)).is_some() {}
            Kind::Operator
        };
        let end = chars.peek().map_or(text.len(), |&(at, _)| at);
        tokens.push(Token { kind, start, text: &text[start..end], closed });
    }
    tokens
}

fn complete_expression(text: &str, caret: usize, fields: &[QueryField], max: usize) -> QueryCompletion {
    let mut before = lex(&text[..caret]);
    // The token being typed, if the caret sits at the end of one.
    let current = before.pop_if(|token| token.end() == caret && (token.kind == Kind::Word || token.is_open_text()));
    let start = current.map_or(caret, |token| token.start);
    let typed = current.map_or("", |token| token.text);
    let none = QueryCompletion { start, end: caret, items: Vec::new() };

    let previous = before.last();
    let wants_field = previous.is_none_or(Token::joins);
    if wants_field {
        if current.is_some_and(|token| token.kind == Kind::Text) {
            return none;
        }
        let rest = text[caret..].find(|c| !is_word(c)).unwrap_or(text.len() - caret);
        let items = field_items(fields, typed, max, |field| (field.name.to_string(), format!("{} ", field.name)));
        return QueryCompletion { start, end: caret + rest, items };
    }

    // A value follows `field <operator>`, where the operator is a symbol
    // (`==`, `!=`, `~`) or a word (`like`, `contains`).
    let [.., subject, operator] = before.as_slice() else { return none };
    let is_operator = matches!(operator.kind, Kind::Operator | Kind::Word) && !operator.joins();
    let Some(field) = (is_operator && subject.kind == Kind::Word).then(|| find_field(fields, subject.text)).flatten()
    else {
        return none;
    };
    let after = &text[caret..];
    let rest = match typed.chars().next().filter(|c| matches!(c, '"' | '\'')) {
        // Swallow the closing quote an editor may have paired.
        Some(quote) => usize::from(after.starts_with(quote)),
        None => after.find(|c| !is_word(c)).unwrap_or(after.len()),
    };
    let items = value_items(field, unquoted(typed), max, |value| {
        let literal = value.parse::<f64>().is_ok() || matches!(value, "true" | "false" | "null");
        format!("{} ", if literal { value.to_owned() } else { quoted(value) })
    });
    QueryCompletion { start, end: caret + rest, items }
}
