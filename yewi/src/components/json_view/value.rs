//! [`Json`]: a parsed JSON document that is cheap to clone, compare and hand
//! down a component tree.

use std::{fmt, rc::Rc, str::FromStr};

use yew::AttrValue;

/// Nesting deeper than this is refused rather than risking the stack.
const MAX_DEPTH: usize = 256;

/// A JSON value.
///
/// Arrays and objects are reference counted, so a clone is one pointer and
/// comparing a value with itself does not walk it. That is what lets
/// [`JsonView`](super::JsonView) pass subtrees to its nodes and re-render
/// one of them without touching the rest of a large document.
///
/// - Build one with [`Json::parse`] (or `str::parse`), or from a
///   `serde_json::Value` with the `serde_json` feature.
/// - Objects keep their keys in document order.
/// - Numbers keep their source text, so large integers and exact decimals
///   are shown as they were written.
#[derive(Debug, Clone)]
pub enum Json {
    Null,
    Bool(bool),
    /// The number as written, for example `1.50` or `18446744073709551616`.
    Number(AttrValue),
    String(AttrValue),
    Array(Rc<[Json]>),
    /// Entries in document order.
    Object(Rc<[(AttrValue, Json)]>),
}

impl PartialEq for Json {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Json::Null, Json::Null) => true,
            (Json::Bool(a), Json::Bool(b)) => a == b,
            (Json::Number(a), Json::Number(b)) | (Json::String(a), Json::String(b)) => a == b,
            (Json::Array(a), Json::Array(b)) => Rc::ptr_eq(a, b) || a == b,
            (Json::Object(a), Json::Object(b)) => Rc::ptr_eq(a, b) || a == b,
            _ => false,
        }
    }
}

impl Json {
    /// Parses a JSON document (RFC 8259).
    pub fn parse(text: &str) -> Result<Json, JsonError> {
        let mut parser = Parser { bytes: text.as_bytes(), at: 0 };
        parser.whitespace();
        let value = parser.value(0)?;
        parser.whitespace();
        if parser.at < parser.bytes.len() {
            return Err(parser.error("unexpected text after the document"));
        }
        Ok(value)
    }

    /// Whether this is an array or an object.
    pub const fn is_container(&self) -> bool {
        matches!(self, Json::Array(_) | Json::Object(_))
    }

    /// The number of items in an array or entries in an object; 0 otherwise.
    pub fn len(&self) -> usize {
        match self {
            Json::Array(items) => items.len(),
            Json::Object(entries) => entries.len(),
            _ => 0,
        }
    }

    /// Whether [`len`](Self::len) is 0.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The document indented by two spaces, as `JsonView` shows and copies it.
    pub fn to_pretty(&self) -> String {
        let mut out = String::new();
        self.write(&mut out, Some(0));
        out
    }

    fn write(&self, out: &mut String, indent: Option<usize>) {
        let inner = indent.map(|depth| depth + 1);
        let line = |out: &mut String, depth: Option<usize>| {
            if let Some(depth) = depth {
                out.push('\n');
                (0..depth).for_each(|_| out.push_str("  "));
            }
        };
        match self {
            Json::Null => out.push_str("null"),
            Json::Bool(value) => out.push_str(if *value { "true" } else { "false" }),
            Json::Number(text) => out.push_str(text),
            Json::String(text) => quote(text, out),
            Json::Array(items) if items.is_empty() => out.push_str("[]"),
            Json::Object(entries) if entries.is_empty() => out.push_str("{}"),
            Json::Array(items) => {
                out.push('[');
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    line(out, inner);
                    item.write(out, inner);
                }
                line(out, indent);
                out.push(']');
            }
            Json::Object(entries) => {
                out.push('{');
                for (i, (key, value)) in entries.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    line(out, inner);
                    quote(key, out);
                    out.push(':');
                    if indent.is_some() {
                        out.push(' ');
                    }
                    value.write(out, inner);
                }
                line(out, indent);
                out.push('}');
            }
        }
    }
}

/// The compact form, with no whitespace.
impl fmt::Display for Json {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        self.write(&mut out, None);
        f.write_str(&out)
    }
}

impl FromStr for Json {
    type Err = JsonError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Json::parse(text)
    }
}

/// Lets a memoised document be passed straight to a prop:
/// `<JsonView value={use_memo(.., |_| Json::parse(..))} />`.
impl yew::html::IntoPropValue<Json> for Rc<Json> {
    fn into_prop_value(self) -> Json {
        Rc::unwrap_or_clone(self)
    }
}

impl From<bool> for Json {
    fn from(value: bool) -> Self {
        Json::Bool(value)
    }
}

impl From<&str> for Json {
    fn from(value: &str) -> Self {
        Json::String(AttrValue::from(value.to_owned()))
    }
}

impl From<String> for Json {
    fn from(value: String) -> Self {
        Json::String(AttrValue::from(value))
    }
}

#[cfg(feature = "serde_json")]
impl From<&serde_json::Value> for Json {
    fn from(value: &serde_json::Value) -> Self {
        use serde_json::Value;
        match value {
            Value::Null => Json::Null,
            Value::Bool(value) => Json::Bool(*value),
            Value::Number(number) => Json::Number(number.to_string().into()),
            Value::String(text) => Json::String(text.clone().into()),
            Value::Array(items) => Json::Array(items.iter().map(Json::from).collect()),
            Value::Object(map) => {
                Json::Object(map.iter().map(|(key, value)| (AttrValue::from(key.clone()), Json::from(value))).collect())
            }
        }
    }
}

#[cfg(feature = "serde_json")]
impl From<serde_json::Value> for Json {
    fn from(value: serde_json::Value) -> Self {
        Json::from(&value)
    }
}

/// Appends `text` as a JSON string literal.
pub(super) fn quote(text: &str, out: &mut String) {
    use fmt::Write as _;

    out.push('"');
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c < ' ' => write!(out, "\\u{:04x}", c as u32).unwrap(),
            c => out.push(c),
        }
    }
    out.push('"');
}

/// Why a document could not be parsed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonError {
    /// The byte offset in the text where parsing stopped.
    pub offset: usize,
    /// What was wrong there.
    pub message: &'static str,
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at byte {}", self.message, self.offset)
    }
}

impl std::error::Error for JsonError {}

struct Parser<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl Parser<'_> {
    fn error(&self, message: &'static str) -> JsonError {
        JsonError { offset: self.at, message }
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.at).copied()
    }

    fn whitespace(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\n' | b'\r' | b'\t')) {
            self.at += 1;
        }
    }

    fn literal(&mut self, word: &str, value: Json) -> Result<Json, JsonError> {
        if self.bytes[self.at..].starts_with(word.as_bytes()) {
            self.at += word.len();
            Ok(value)
        } else {
            Err(self.error("expected a value"))
        }
    }

    fn value(&mut self, depth: usize) -> Result<Json, JsonError> {
        if depth > MAX_DEPTH {
            return Err(self.error("nested too deeply"));
        }
        match self.peek() {
            Some(b'n') => self.literal("null", Json::Null),
            Some(b't') => self.literal("true", Json::Bool(true)),
            Some(b'f') => self.literal("false", Json::Bool(false)),
            Some(b'"') => self.string().map(Json::String),
            Some(b'-' | b'0'..=b'9') => self.number(),
            Some(b'[') => {
                let mut items = Vec::new();
                self.members(b']', |parser| {
                    items.push(parser.value(depth + 1)?);
                    Ok(())
                })?;
                Ok(Json::Array(items.into()))
            }
            Some(b'{') => {
                let mut entries = Vec::new();
                self.members(b'}', |parser| {
                    if parser.peek() != Some(b'"') {
                        return Err(parser.error("expected a key"));
                    }
                    let key = parser.string()?;
                    parser.whitespace();
                    if parser.peek() != Some(b':') {
                        return Err(parser.error("expected ':'"));
                    }
                    parser.at += 1;
                    parser.whitespace();
                    entries.push((key, parser.value(depth + 1)?));
                    Ok(())
                })?;
                Ok(Json::Object(entries.into()))
            }
            Some(_) => Err(self.error("expected a value")),
            None => Err(self.error("unexpected end of the document")),
        }
    }

    /// The comma-separated members of an array or object, up to `close`.
    fn members(
        &mut self,
        close: u8,
        mut member: impl FnMut(&mut Self) -> Result<(), JsonError>,
    ) -> Result<(), JsonError> {
        self.at += 1;
        self.whitespace();
        if self.peek() == Some(close) {
            self.at += 1;
            return Ok(());
        }
        loop {
            member(self)?;
            self.whitespace();
            match self.peek() {
                Some(b',') => {
                    self.at += 1;
                    self.whitespace();
                }
                Some(byte) if byte == close => {
                    self.at += 1;
                    return Ok(());
                }
                Some(_) => return Err(self.error("expected ',' or the end of the container")),
                None => return Err(self.error("unexpected end of the document")),
            }
        }
    }

    fn number(&mut self) -> Result<Json, JsonError> {
        let start = self.at;
        let digits = |parser: &mut Self| {
            let from = parser.at;
            while matches!(parser.peek(), Some(b'0'..=b'9')) {
                parser.at += 1;
            }
            parser.at - from
        };
        if self.peek() == Some(b'-') {
            self.at += 1;
        }
        let leading_zero = self.peek() == Some(b'0');
        match digits(self) {
            0 => return Err(self.error("expected a digit")),
            1 => {}
            _ if leading_zero => return Err(JsonError { offset: start, message: "a number cannot start with 0" }),
            _ => {}
        }
        if self.peek() == Some(b'.') {
            self.at += 1;
            if digits(self) == 0 {
                return Err(self.error("expected a digit"));
            }
        }
        if matches!(self.peek(), Some(b'e' | b'E')) {
            self.at += 1;
            if matches!(self.peek(), Some(b'+' | b'-')) {
                self.at += 1;
            }
            if digits(self) == 0 {
                return Err(self.error("expected a digit"));
            }
        }
        // Only ASCII was consumed, so the slice is valid UTF-8.
        let text = std::str::from_utf8(&self.bytes[start..self.at]).unwrap_or_default();
        Ok(Json::Number(AttrValue::from(text.to_owned())))
    }

    fn string(&mut self) -> Result<AttrValue, JsonError> {
        self.at += 1;
        let mut out = String::new();
        loop {
            // Copy the run up to the next quote, escape or control byte whole.
            // Those are ASCII, so the run ends on a character boundary.
            let start = self.at;
            while matches!(self.peek(), Some(byte) if byte != b'"' && byte != b'\\' && byte >= b' ') {
                self.at += 1;
            }
            out.push_str(std::str::from_utf8(&self.bytes[start..self.at]).unwrap_or_default());
            match self.peek() {
                Some(b'"') => {
                    self.at += 1;
                    return Ok(AttrValue::from(out));
                }
                Some(b'\\') => {
                    self.at += 1;
                    out.push(self.escape()?);
                }
                Some(_) => return Err(self.error("control character in a string")),
                None => return Err(self.error("unterminated string")),
            }
        }
    }

    fn escape(&mut self) -> Result<char, JsonError> {
        let c = match self.peek() {
            Some(b'"') => '"',
            Some(b'\\') => '\\',
            Some(b'/') => '/',
            Some(b'b') => '\u{8}',
            Some(b'f') => '\u{c}',
            Some(b'n') => '\n',
            Some(b'r') => '\r',
            Some(b't') => '\t',
            Some(b'u') => {
                self.at += 1;
                let high = self.hex4()?;
                let code = match high {
                    0xD800..=0xDBFF if self.bytes[self.at..].starts_with(b"\\u") => {
                        self.at += 2;
                        let low = self.hex4()?;
                        if !(0xDC00..=0xDFFF).contains(&low) {
                            return Err(self.error("invalid surrogate pair"));
                        }
                        0x10000 + ((high - 0xD800) << 10) + (low - 0xDC00)
                    }
                    code => code,
                };
                // A lone surrogate has no character; show the replacement.
                return Ok(char::from_u32(code).unwrap_or(char::REPLACEMENT_CHARACTER));
            }
            _ => return Err(self.error("invalid escape")),
        };
        self.at += 1;
        Ok(c)
    }

    fn hex4(&mut self) -> Result<u32, JsonError> {
        let digits = self.bytes.get(self.at..self.at + 4).ok_or_else(|| self.error("invalid escape"))?;
        let mut code = 0;
        for &digit in digits {
            code = code * 16 + (digit as char).to_digit(16).ok_or_else(|| self.error("invalid escape"))?;
        }
        self.at += 4;
        Ok(code)
    }
}
