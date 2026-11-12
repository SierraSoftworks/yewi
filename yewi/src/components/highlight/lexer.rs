//! The built-in lexers: one table-driven scanner for programming and data
//! languages, plus small ones for markup and diffs.
//!
//! These are not parsers. They recognise comments, strings, numbers, words
//! from a list and a handful of shapes (a call, a key, a macro), which is
//! what gives code its familiar colouring at a few kilobytes of wasm. Input
//! that confuses them is shown plain or miscoloured, never lost or reordered.

use super::{Language, TokenKind};

/// `(start, end, kind)` byte ranges into the source, in order and gap-free.
pub(super) type Spans = Vec<(usize, usize, Option<TokenKind>)>;

// Shapes a language opts into.
const CASELESS: u32 = 1 << 0; // keywords match in any case (SQL)
const CAPITAL_TYPES: u32 = 1 << 1; // `Name` is a type, `NAME` a constant
const CALLS: u32 = 1 << 2; // `name(` is a function
const BANG_MACRO: u32 = 1 << 3; // `name!` is a macro (Rust)
const KEY_COLON: u32 = 1 << 4; // `name:` and `"name":` are keys
const KEY_EQUALS: u32 = 1 << 5; // `name =` is a key
const DOLLAR_VAR: u32 = 1 << 6; // `$name` and `${name}` are variables
const AT_META: u32 = 1 << 7; // `@name` is a decorator
const AT_KEYWORD: u32 = 1 << 8; // `@name` is a keyword (CSS at-rules)
const HASH_ATTR: u32 = 1 << 9; // `#[...]` is an attribute (Rust)
const LIFETIMES: u32 = 1 << 10; // `'a` is a lifetime, `'a'` a character (Rust)
const RAW_STRINGS: u32 = 1 << 11; // `r#"..."#` (Rust)
const STRING_PREFIX: u32 = 1 << 12; // `f"..."`, `b"..."`
const SECTION: u32 = 1 << 13; // `[name]` on its own line is a heading (TOML)
const COMMANDS: u32 = 1 << 14; // the first word of a command is a function (shell)
const HEX_COLOUR: u32 = 1 << 15; // `#1a2b3c` is a value (CSS)
const TRIPLE: u32 = 1 << 16; // `"""..."""` spans lines
const DATES: u32 = 1 << 17; // `2026-01-15T12:00:00Z` is one value
const BLOCKS: u32 = 1 << 18; // `name "label" {` opens a block (HCL)

struct Syntax {
    line_comments: &'static [&'static str],
    block_comment: Option<(&'static str, &'static str)>,
    /// Characters that open a string closed by the same character.
    quotes: &'static str,
    /// The quotes whose strings may span lines.
    multiline: &'static str,
    /// Space-separated word lists.
    keywords: &'static str,
    more_keywords: &'static str,
    literals: &'static str,
    types: &'static str,
    /// Characters besides letters, digits and `_` that belong to a word.
    word_extra: &'static str,
    flags: u32,
}

const C_COMMENTS: &[&str] = &["//"];
const HASH_COMMENTS: &[&str] = &["#"];
const C_BLOCK: Option<(&str, &str)> = Some(("/*", "*/"));

const JS_KEYWORDS: &str = "as async await break case catch class const continue debugger default delete do else export \
    extends finally for from function get if import in instanceof let new of return set static super switch this throw \
    try typeof var void while with yield";
const TS_KEYWORDS: &str = "abstract declare enum implements infer interface is keyof namespace override private \
    protected public readonly satisfies type";

fn syntax(language: Language) -> Syntax {
    let base = Syntax {
        line_comments: &[],
        block_comment: None,
        quotes: "\"'",
        multiline: "",
        keywords: "",
        more_keywords: "",
        literals: "true false null",
        types: "",
        word_extra: "",
        flags: 0,
    };
    match language {
        Language::Rust => Syntax {
            line_comments: C_COMMENTS,
            block_comment: C_BLOCK,
            multiline: "\"",
            keywords: "as async await break const continue crate dyn else enum extern fn for if impl in let loop match \
                mod move mut pub ref return self Self static struct super trait type unsafe use where while",
            literals: "true false",
            types: "bool char f32 f64 i8 i16 i32 i64 i128 isize str u8 u16 u32 u64 u128 usize",
            flags: CAPITAL_TYPES | CALLS | BANG_MACRO | HASH_ATTR | LIFETIMES | RAW_STRINGS | STRING_PREFIX,
            ..base
        },
        Language::JavaScript | Language::TypeScript => Syntax {
            line_comments: C_COMMENTS,
            block_comment: C_BLOCK,
            quotes: "\"'`",
            multiline: "`",
            keywords: JS_KEYWORDS,
            more_keywords: if language == Language::TypeScript { TS_KEYWORDS } else { "" },
            literals: "true false null undefined NaN Infinity",
            types: if language == Language::TypeScript {
                "any bigint boolean never number object string symbol unknown"
            } else {
                ""
            },
            flags: CAPITAL_TYPES | CALLS | AT_META,
            ..base
        },
        Language::Python => Syntax {
            line_comments: HASH_COMMENTS,
            keywords: "and as assert async await break case class continue def del elif else except finally for from \
                global if import in is lambda match nonlocal not or pass raise return try while with yield",
            literals: "True False None",
            types: "bool bytes dict float int list object set str tuple",
            flags: CAPITAL_TYPES | CALLS | AT_META | STRING_PREFIX | TRIPLE,
            ..base
        },
        Language::Go => Syntax {
            line_comments: C_COMMENTS,
            block_comment: C_BLOCK,
            quotes: "\"'`",
            multiline: "`",
            keywords: "break case chan const continue default defer else fallthrough for func go goto if import \
                interface map package range return select struct switch type var",
            literals: "true false nil iota",
            types: "any bool byte complex64 complex128 error float32 float64 int int8 int16 int32 int64 rune string \
                uint uint8 uint16 uint32 uint64 uintptr",
            flags: CALLS,
            ..base
        },
        Language::Bash => Syntax {
            line_comments: HASH_COMMENTS,
            multiline: "\"'",
            keywords: "case declare do done elif else esac exit export fi for function if in local readonly return \
                select then until while",
            literals: "",
            word_extra: "-./",
            flags: DOLLAR_VAR | COMMANDS,
            ..base
        },
        Language::Sql => Syntax {
            line_comments: &["--"],
            block_comment: C_BLOCK,
            keywords: "add all alter and as asc begin between by case check column commit constraint create cross \
                default delete desc distinct drop else end exists foreign from full group having if in index inner \
                insert into is join key left like limit not offset on or order outer primary references returning \
                right rollback select set table then union unique update using values view when where with",
            types: "bigint boolean char date decimal double float int integer json jsonb numeric real serial smallint \
                text time timestamp timestamptz uuid varchar",
            flags: CASELESS | CALLS,
            ..base
        },
        Language::Css => Syntax {
            line_comments: C_COMMENTS,
            block_comment: C_BLOCK,
            literals: "",
            word_extra: "-",
            flags: AT_KEYWORD | DOLLAR_VAR | KEY_COLON | CALLS | HEX_COLOUR,
            ..base
        },
        Language::Json => {
            Syntax { line_comments: C_COMMENTS, block_comment: C_BLOCK, quotes: "\"", flags: KEY_COLON, ..base }
        }
        Language::Yaml => Syntax {
            line_comments: HASH_COMMENTS,
            literals: "true false null yes no on off",
            word_extra: "-./",
            flags: KEY_COLON | DATES,
            ..base
        },
        Language::Toml => Syntax {
            line_comments: HASH_COMMENTS,
            literals: "true false",
            word_extra: "-.",
            flags: KEY_EQUALS | SECTION | TRIPLE | DATES,
            ..base
        },
        Language::Hcl => Syntax {
            line_comments: &["#", "//"],
            block_comment: C_BLOCK,
            quotes: "\"",
            keywords: "for in if else",
            word_extra: "-",
            flags: KEY_EQUALS | BLOCKS | CALLS,
            ..base
        },
        // Lexed by `markup` and `diff`, never by `Scanner`.
        Language::Html | Language::Diff => base,
    }
}

pub(super) fn lex(source: &str, language: Language) -> Spans {
    let mut out = Spans::new();
    match language {
        Language::Html => markup(source, &mut out),
        Language::Diff => diff(source, &mut out),
        _ => Scanner::new(source, 0, source.len(), &syntax(language), &mut out).run(),
    }
    out
}

fn push(out: &mut Spans, start: usize, end: usize, kind: Option<TokenKind>) {
    if start >= end {
        return;
    }
    match out.last_mut() {
        Some(last) if last.2 == kind && last.1 == start => last.1 = end,
        _ => out.push((start, end, kind)),
    }
}

fn contains_word(list: &str, word: &str) -> bool {
    list.split_ascii_whitespace().any(|entry| entry == word)
}

/// Scans `bytes[i..end]`. Every index it stops at is a `char` boundary:
/// tokens end on an ASCII byte, or run to the end of a whole word.
struct Scanner<'a, 's> {
    bytes: &'a [u8],
    source: &'a str,
    i: usize,
    end: usize,
    syntax: &'s Syntax,
    out: &'s mut Spans,
    /// Shell: the next word is a command name.
    command: bool,
    /// Only whitespace so far on this line.
    line_start: bool,
}

impl<'a, 's> Scanner<'a, 's> {
    fn new(source: &'a str, start: usize, end: usize, syntax: &'s Syntax, out: &'s mut Spans) -> Self {
        Self { bytes: source.as_bytes(), source, i: start, end, syntax, out, command: true, line_start: true }
    }

    fn has(&self, flag: u32) -> bool {
        self.syntax.flags & flag != 0
    }

    fn at(&self, i: usize) -> u8 {
        if i < self.end { self.bytes[i] } else { 0 }
    }

    fn starts_with(&self, i: usize, text: &str) -> bool {
        self.bytes[i..self.end].starts_with(text.as_bytes())
    }

    fn is_word(&self, byte: u8) -> bool {
        byte.is_ascii_alphanumeric() || byte == b'_' || byte >= 0x80 || self.is_extra(byte)
    }

    fn is_extra(&self, byte: u8) -> bool {
        byte != 0 && self.syntax.word_extra.as_bytes().contains(&byte)
    }

    fn starts_word(&self, i: usize) -> bool {
        let byte = self.at(i);
        let letter = |b: u8| b.is_ascii_alphabetic() || b == b'_' || b >= 0x80;
        // `-webkit-x` and `./run.sh` are words; `-1` and a lone `-` are not.
        letter(byte) || (self.is_extra(byte) && (letter(self.at(i + 1)) || self.is_extra(self.at(i + 1))))
    }

    fn line_end(&self, from: usize) -> usize {
        self.bytes[from..self.end].iter().position(|&b| b == b'\n').map_or(self.end, |n| from + n)
    }

    fn find(&self, from: usize, text: &str) -> Option<usize> {
        self.source.get(from.min(self.end)..self.end)?.find(text).map(|n| from + n)
    }

    /// The first index at or after `i` that is not a space or tab.
    fn skip_blank(&self, mut i: usize) -> usize {
        while matches!(self.at(i), b' ' | b'\t') {
            i += 1;
        }
        i
    }

    fn emit(&mut self, end: usize, kind: Option<TokenKind>) {
        let end = end.min(self.end).max(self.i + 1);
        push(self.out, self.i, end, kind);
        self.i = end;
        self.line_start = false;
    }

    fn run(mut self) {
        while self.i < self.end {
            self.step();
        }
    }

    fn step(&mut self) {
        let (i, byte) = (self.i, self.bytes[self.i]);
        let previous = if i > 0 { self.bytes[i - 1] } else { b'\n' };

        if byte.is_ascii_whitespace() {
            push(self.out, i, i + 1, None);
            self.i += 1;
            // A trailing backslash continues the command onto the next line.
            if byte == b'\n' {
                self.line_start = true;
                self.command = previous != b'\\';
            }
            return;
        }

        if self.has(SECTION) && self.line_start && byte == b'[' {
            let line = self.line_end(i);
            let close = self.bytes[i..line].iter().rposition(|&b| b == b']').map_or(line, |n| i + n + 1);
            return self.emit(close, Some(TokenKind::Meta));
        }

        for marker in self.syntax.line_comments {
            // `#` and `//` also turn up inside words: `$#`, `a#b`, `http://`.
            let detached = match *marker {
                "#" => previous.is_ascii_whitespace(),
                "//" => previous != b':',
                _ => true,
            };
            if detached && self.starts_with(i, marker) {
                return self.emit(self.line_end(i), Some(TokenKind::Comment));
            }
        }
        if let Some((open, close)) = self.syntax.block_comment
            && self.starts_with(i, open)
        {
            let end = self.find(i + open.len(), close).map_or(self.end, |n| n + close.len());
            return self.emit(end, Some(TokenKind::Comment));
        }

        if self.has(HASH_ATTR) && byte == b'#' && (self.at(i + 1) == b'[' || self.starts_with(i + 1, "![")) {
            return self.emit(self.balanced(i, b'[', b']'), Some(TokenKind::Meta));
        }

        if self.syntax.quotes.as_bytes().contains(&byte) {
            // An apostrophe inside a word is not a quote.
            if byte == b'\'' && previous.is_ascii_alphanumeric() {
                return self.emit(i + 1, None);
            }
            return self.string(i);
        }

        if self.has(DOLLAR_VAR) && byte == b'$' {
            let next = self.at(i + 1);
            if next == b'{' {
                let end = self.find(i, "}").map_or(self.end, |n| n + 1);
                return self.emit(end, Some(TokenKind::Variable));
            }
            if next.is_ascii_alphanumeric() || next == b'_' {
                let mut j = i + 1;
                while self.at(j).is_ascii_alphanumeric() || self.at(j) == b'_' || (self.at(j) == b'-' && !self.has(COMMANDS))
                {
                    j += 1;
                }
                return self.emit(j, Some(TokenKind::Variable));
            }
            if b"?@#*!$".contains(&next) {
                return self.emit(i + 2, Some(TokenKind::Variable));
            }
        }

        if self.has(AT_META | AT_KEYWORD) && byte == b'@' && self.starts_word(i + 1) {
            let mut j = i + 1;
            while self.is_word(self.at(j)) || (self.at(j) == b'.' && self.has(AT_META)) {
                j += 1;
            }
            let kind = if self.has(AT_META) { TokenKind::Meta } else { TokenKind::Keyword };
            return self.emit(j, Some(kind));
        }

        if self.has(HEX_COLOUR) && byte == b'#' {
            let digits = self.bytes[i + 1..self.end].iter().take_while(|b| b.is_ascii_hexdigit()).count();
            if (3..=8).contains(&digits) && !self.is_word(self.at(i + 1 + digits)) {
                return self.emit(i + 1 + digits, Some(TokenKind::Number));
            }
        }

        if byte.is_ascii_digit() {
            let mut j = i + 1;
            loop {
                let b = self.at(j);
                let part = b.is_ascii_alphanumeric()
                    || b == b'_'
                    || (b == b'.' && self.at(j + 1).is_ascii_digit())
                    || (self.has(DATES) && matches!(b, b'-' | b':') && self.at(j + 1).is_ascii_digit());
                if !part {
                    break;
                }
                j += 1;
            }
            self.command = false;
            return self.emit(j, Some(TokenKind::Number));
        }

        if self.starts_word(i) {
            return self.word(i);
        }

        // Punctuation. In a shell these start a new command.
        if b"|;&(`{".contains(&byte) {
            self.command = true;
        } else if byte != b'$' {
            self.command = false;
        }
        self.emit(i + 1, None);
    }

    /// The index after the bracket that closes the one at or after `from`.
    fn balanced(&self, from: usize, open: u8, close: u8) -> usize {
        let mut depth = 0usize;
        for (j, &b) in self.bytes.iter().enumerate().take(self.end).skip(from) {
            if b == open {
                depth += 1;
            } else if b == close {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return j + 1;
                }
            } else if b == b'\n' && depth == 0 {
                break;
            }
        }
        self.end
    }

    /// The index after the string whose opening quote is at `open`.
    fn string_end(&self, open: usize) -> usize {
        let quote = self.bytes[open];
        let triple = [quote; 3];
        if self.has(TRIPLE) && self.bytes[open..self.end].starts_with(&triple) {
            let close = std::str::from_utf8(&triple).unwrap_or_default();
            return self.find(open + 3, close).map_or(self.end, |n| n + 3);
        }
        let multiline = self.syntax.multiline.as_bytes().contains(&quote);
        let mut j = open + 1;
        while j < self.end {
            match self.bytes[j] {
                b'\\' => j += 1,
                b if b == quote => return j + 1,
                b'\n' if !multiline => return j,
                _ => {}
            }
            j += 1;
        }
        self.end
    }

    /// A string, character or lifetime starting at `self.i`, with its opening
    /// quote at `open` (later than `self.i` after a prefix such as `b`).
    fn string(&mut self, open: usize) {
        if self.has(LIFETIMES) && self.bytes[open] == b'\'' {
            let next = self.source[open + 1..self.end].chars().next();
            let width = next.map_or(0, char::len_utf8);
            let is_char = next == Some('\\') || (width > 0 && self.at(open + 1 + width) == b'\'');
            if !is_char {
                let mut j = open + 1;
                while self.is_word(self.at(j)) {
                    j += 1;
                }
                let kind = (j > open + 1).then_some(TokenKind::Meta);
                return self.emit(j, kind);
            }
        }

        let end = self.string_end(open);
        let colon = self.skip_blank(end);
        let key = self.has(KEY_COLON) && self.at(colon) == b':';
        self.command = false;
        self.emit(end, Some(if key { TokenKind::Property } else { TokenKind::String }));
    }

    fn word(&mut self, start: usize) {
        let mut end = start + 1;
        while self.is_word(self.at(end)) {
            end += 1;
        }
        let (source, syntax) = (self.source, self.syntax);
        let word = &source[start..end];
        let next = self.at(end);

        if self.has(RAW_STRINGS) && matches!(word, "r" | "br") && matches!(next, b'"' | b'#') {
            let hashes = self.bytes[end..self.end].iter().take_while(|&&b| b == b'#').count();
            if self.at(end + hashes) == b'"' {
                let close = format!("\"{}", "#".repeat(hashes));
                let stop = self.find(end + hashes + 1, &close).map_or(self.end, |n| n + close.len());
                return self.emit(stop, Some(TokenKind::String));
            }
        }
        if self.has(STRING_PREFIX) && word.len() <= 2 && next != 0 && self.syntax.quotes.as_bytes().contains(&next) {
            return self.string(end);
        }

        let first_on_line = self.line_start;
        let command = std::mem::replace(&mut self.command, false);
        let after = self.skip_blank(end);
        let colon_key = self.has(KEY_COLON)
            && self.at(after) == b':'
            && (self.at(after + 1).is_ascii_whitespace() || after + 1 >= self.end);
        let equals_key =
            self.has(KEY_EQUALS) && self.at(after) == b'=' && !matches!(self.at(after + 1), b'=' | b'>');
        let lowered;
        let listed = if self.has(CASELESS) {
            lowered = word.to_ascii_lowercase();
            lowered.as_str()
        } else {
            word
        };
        let listed = |list: &str| contains_word(list, listed);

        let kind = if listed(syntax.keywords) || listed(syntax.more_keywords) {
            // After `if` or `then` comes a command; after `for` or `in`, a name.
            self.command = command && !matches!(word, "for" | "case" | "in" | "function" | "select");
            Some(TokenKind::Keyword)
        } else if listed(syntax.literals) {
            Some(TokenKind::Literal)
        } else if listed(syntax.types) {
            Some(TokenKind::Type)
        } else if self.has(COMMANDS) && command {
            match (next, word.as_bytes()[0]) {
                (b'=', _) => Some(TokenKind::Variable),
                (_, b'-') => None,
                _ => Some(TokenKind::Function),
            }
        } else if self.has(BANG_MACRO) && next == b'!' && self.at(end + 1) != b'=' {
            end += 1;
            Some(TokenKind::Meta)
        } else if self.has(CAPITAL_TYPES) && word.starts_with(|c: char| c.is_ascii_uppercase()) {
            let constant = word.len() > 1 && !word.contains(|c: char| c.is_ascii_lowercase());
            Some(if constant { TokenKind::Literal } else { TokenKind::Type })
        } else if self.has(CALLS) && next == b'(' {
            Some(TokenKind::Function)
        } else if colon_key || equals_key {
            Some(TokenKind::Property)
        } else if self.has(BLOCKS) && first_on_line && matches!(self.at(after), b'"' | b'{') {
            Some(TokenKind::Keyword)
        } else {
            None
        };
        self.emit(end, kind);
    }
}

/// HTML and XML: comments, tags, attributes and entities. The contents of
/// `<script>` and `<style>` are handed to the JavaScript and CSS lexers.
fn markup(source: &str, out: &mut Spans) {
    let bytes = source.as_bytes();
    let len = bytes.len();
    let find = |from: usize, text: &str| source.get(from..).and_then(|rest| rest.find(text)).map(|n| from + n);
    let name_end = |mut j: usize| {
        while j < len && !bytes[j].is_ascii_whitespace() && !b"<>=/\"'".contains(&bytes[j]) {
            j += 1;
        }
        j
    };
    let mut i = 0;

    while i < len {
        if source[i..].starts_with("<!--") {
            let end = find(i + 4, "-->").map_or(len, |n| n + 3);
            push(out, i, end, Some(TokenKind::Comment));
            i = end;
        } else if source[i..].starts_with("<!") || source[i..].starts_with("<?") {
            let end = find(i, ">").map_or(len, |n| n + 1);
            push(out, i, end, Some(TokenKind::Meta));
            i = end;
        } else if bytes[i] == b'<' && i + 1 < len && (bytes[i + 1].is_ascii_alphabetic() || bytes[i + 1] == b'/') {
            let name_start = if bytes[i + 1] == b'/' { i + 2 } else { i + 1 };
            let mut j = name_end(name_start);
            let name = &source[name_start..j];
            push(out, i, name_start, None);
            push(out, name_start, j, Some(TokenKind::Tag));

            // Attributes, up to the closing `>`.
            while j < len && bytes[j] != b'>' {
                let b = bytes[j];
                if b == b'"' || b == b'\'' {
                    let end = bytes[j + 1..].iter().position(|&c| c == b).map_or(len, |n| j + n + 2);
                    push(out, j, end, Some(TokenKind::String));
                    j = end;
                } else if b.is_ascii_whitespace() || b"=/<".contains(&b) {
                    push(out, j, j + 1, None);
                    j += 1;
                } else {
                    let end = name_end(j).max(j + 1);
                    // Yew's `html!` and JSX put expressions here; leave them plain.
                    let kind = (b != b'{' && b != b'}').then_some(TokenKind::Property);
                    let end = if kind.is_some() { end } else { j + 1 };
                    push(out, j, end, kind);
                    j = end;
                }
            }
            let opened = bytes[i + 1] != b'/' && (j == 0 || bytes[j - 1] != b'/');
            if j < len {
                push(out, j, j + 1, None);
                j += 1;
            }
            i = j;

            let embedded = match name {
                "script" => Some(Language::JavaScript),
                "style" => Some(Language::Css),
                _ => None,
            };
            if let Some(language) = embedded.filter(|_| opened) {
                let end = find(i, &format!("</{name}")).unwrap_or(len);
                Scanner::new(source, i, end, &syntax(language), out).run();
                i = end;
            }
        } else if bytes[i] == b'&' {
            let end = name_end(i + 1);
            let entity = end < len && bytes[end] == b';' && end > i + 1;
            let end = if entity { end + 1 } else { i + 1 };
            push(out, i, end, entity.then_some(TokenKind::Literal));
            i = end;
        } else {
            let end = bytes[i + 1..].iter().position(|&b| b == b'<' || b == b'&').map_or(len, |n| i + 1 + n);
            push(out, i, end, None);
            i = end;
        }
    }
}

/// Unified diffs, by the first character of each line.
fn diff(source: &str, out: &mut Spans) {
    let mut start = 0;
    for line in source.split_inclusive('\n') {
        let text = line.strip_suffix('\n').unwrap_or(line);
        let kind = if ["+++", "---", "@@", "diff ", "index "].iter().any(|p| text.starts_with(p)) {
            Some(TokenKind::Meta)
        } else if text.starts_with('+') {
            Some(TokenKind::Inserted)
        } else if text.starts_with('-') {
            Some(TokenKind::Deleted)
        } else {
            None
        };
        // The newline stays outside, so a coloured line does not bleed.
        out.push((start, start + text.len(), kind));
        if text.len() < line.len() {
            out.push((start + text.len(), start + line.len(), None));
        }
        start += line.len();
    }
    out.retain(|span| span.0 < span.1);
}
