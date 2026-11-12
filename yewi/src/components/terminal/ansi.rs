//! A small ANSI parser: SGR colour and style sequences become styled spans,
//! every other escape sequence is dropped.

/// A colour set by an SGR sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Colour {
    /// One of the 16 palette colours, styled by the `--yi-terminal-*` tokens.
    Palette(u8),
    Rgb(u8, u8, u8),
}

/// The SGR state a run of text is drawn with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(super) struct Style {
    pub fg: Option<Colour>,
    pub bg: Option<Colour>,
    pub bold: bool,
    pub dim: bool,
    pub italic: bool,
    pub underline: bool,
    pub strike: bool,
    pub inverse: bool,
}

impl Style {
    pub fn is_plain(&self) -> bool {
        *self == Style::default()
    }
}

/// A run of text in one style. Line breaks are part of the text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Span {
    pub style: Style,
    pub text: String,
}

/// Parses terminal output into styled spans.
///
/// `\r\n` is a line break, and a lone `\r` restarts the current line, as a
/// progress bar redrawing itself would.
pub(super) fn parse(input: &str) -> Vec<Span> {
    // Finished lines, and the line being written (which a `\r` may restart).
    let mut spans: Vec<Span> = Vec::new();
    let mut line: Vec<Span> = Vec::new();
    let mut style = Style::default();
    let mut chars = input.chars().peekable();

    fn push(spans: &mut Vec<Span>, style: Style, text: &str) {
        match spans.last_mut() {
            Some(last) if last.style == style => last.text.push_str(text),
            _ => spans.push(Span { style, text: text.to_owned() }),
        }
    }

    while let Some(c) = chars.next() {
        match c {
            '\x1b' => match chars.peek() {
                Some('[') => {
                    chars.next();
                    let mut params = String::new();
                    let mut terminator = None;
                    for c in chars.by_ref() {
                        if ('\x40'..='\x7e').contains(&c) {
                            terminator = Some(c);
                            break;
                        }
                        params.push(c);
                    }
                    if terminator == Some('m') {
                        apply_sgr(&mut style, &params);
                    }
                }
                // Operating system commands (titles, hyperlinks) end at BEL or ESC \.
                Some(']') => {
                    chars.next();
                    while let Some(c) = chars.next() {
                        if c == '\x07' || (c == '\x1b' && chars.next_if_eq(&'\\').is_some()) {
                            break;
                        }
                    }
                }
                // Any other two-character escape.
                Some(_) => {
                    chars.next();
                }
                None => {}
            },
            '\r' => {
                if chars.peek() != Some(&'\n') {
                    line.clear();
                }
            }
            '\n' => {
                push(&mut line, style, "\n");
                for span in line.drain(..) {
                    push(&mut spans, span.style, &span.text);
                }
            }
            // Other control characters would render as nothing or as boxes.
            c if c.is_control() && c != '\t' => {}
            c => push(&mut line, style, c.encode_utf8(&mut [0; 4])),
        }
    }

    for span in line {
        push(&mut spans, span.style, &span.text);
    }
    spans
}

fn apply_sgr(style: &mut Style, params: &str) {
    // Parameters are separated by `;` (or `:`); an empty one means 0.
    let mut codes: Vec<u16> = vec![0];
    for byte in params.bytes() {
        match byte {
            b'0'..=b'9' => {
                let last = codes.last_mut().unwrap();
                *last = last.saturating_mul(10).saturating_add(u16::from(byte - b'0'));
            }
            b';' | b':' => codes.push(0),
            _ => return,
        }
    }

    let mut codes = codes.into_iter();
    while let Some(code) = codes.next() {
        match code {
            0 => *style = Style::default(),
            1 => style.bold = true,
            2 => style.dim = true,
            3 => style.italic = true,
            4 => style.underline = true,
            7 => style.inverse = true,
            9 => style.strike = true,
            22 => (style.bold, style.dim) = (false, false),
            23 => style.italic = false,
            24 => style.underline = false,
            27 => style.inverse = false,
            29 => style.strike = false,
            30..=37 => style.fg = Some(Colour::Palette((code - 30) as u8)),
            38 => style.fg = extended(&mut codes).or(style.fg),
            39 => style.fg = None,
            40..=47 => style.bg = Some(Colour::Palette((code - 40) as u8)),
            48 => style.bg = extended(&mut codes).or(style.bg),
            49 => style.bg = None,
            90..=97 => style.fg = Some(Colour::Palette((code - 90 + 8) as u8)),
            100..=107 => style.bg = Some(Colour::Palette((code - 100 + 8) as u8)),
            _ => {}
        }
    }
}

/// The colour that follows `38` or `48`: `5;n` (256 colours) or `2;r;g;b`.
fn extended(codes: &mut std::vec::IntoIter<u16>) -> Option<Colour> {
    let mut byte = || codes.next().map(|v| v.min(255) as u8);
    match byte()? {
        5 => Some(match byte()? {
            n @ 0..=15 => Colour::Palette(n),
            n @ 16..=231 => {
                let level = |v: u8| if v == 0 { 0 } else { 55 + v * 40 };
                let n = n - 16;
                Colour::Rgb(level(n / 36), level(n / 6 % 6), level(n % 6))
            }
            n => {
                let grey = 8 + (n - 232) * 10;
                Colour::Rgb(grey, grey, grey)
            }
        }),
        2 => Some(Colour::Rgb(byte()?, byte()?, byte()?)),
        _ => None,
    }
}
