//! Picks the lines of an example's source worth drawing the eye to.
//!
//! An example file is shown verbatim, so most of it is scaffolding: imports,
//! the function signature, layout wrappers. The lines that matter are the
//! ones its description talks about, and the description already names them
//! in backticks (`pressed`, `ButtonGroup`, `Orientation::Vertical`). Those
//! names are looked up in the source. When none is found, the opening tags of
//! the page's own components are marked instead.

/// Words too common in example code to single out a line.
const COMMON: &[&str] =
    &["true", "false", "None", "Some", "html", "class", "id", "Html", "String", "Callback", "Example"];

/// Above this share of the written lines, a highlight no longer singles anything out.
const MAX_SHARE: f64 = 0.5;

/// The lines of `source` to mark, as a `CodeBlock` `marked` list such as
/// `"8-10, 14"`. Empty when nothing stands out.
pub fn marked(source: &str, description: &str, components: &[&str]) -> String {
    let lines: Vec<&str> = source.trim_end().lines().collect();
    let written = lines.iter().filter(|line| !line.trim().is_empty()).count();

    let named = lines_for(source, &lines, &terms(description));
    let found = if share(&named, written) > 0.0 && share(&named, written) <= MAX_SHARE {
        named
    } else {
        // A hook is called, a component is opened as a tag.
        let own: Vec<String> = components
            .iter()
            .map(|name| if name.starts_with("use_") { (*name).to_owned() } else { format!("<{name}") })
            .collect();
        let tags = lines_for(source, &lines, &own);
        if share(&tags, written) <= MAX_SHARE { tags } else { Vec::new() }
    };
    ranges(&found)
}

fn share(marked: &[bool], written: usize) -> f64 {
    marked.iter().filter(|m| **m).count() as f64 / written.max(1) as f64
}

/// Whether a line can be marked: not blank, not an import or attribute, not
/// the signature, and not only closing brackets and tags.
fn is_code(line: &str) -> bool {
    let line = line.trim();
    !(line.is_empty()
        || line.starts_with("use ")
        || line.starts_with("#[")
        || line.starts_with("//")
        || line.starts_with("</")
        || line.starts_with("pub fn Example")
        || line.chars().all(|c| "{}()[]<>/;,".contains(c)))
}

/// The names a description puts in backticks, split into the identifiers and
/// paths that can be found in source: `NavTarget::route(..)` gives
/// `NavTarget::route`.
fn terms(description: &str) -> Vec<String> {
    let mut terms = Vec::new();
    for span in description.split('`').skip(1).step_by(2) {
        for word in span.split(|c: char| !(c.is_alphanumeric() || "_:-<".contains(c))) {
            let word = word.trim_matches(|c| c == ':' || c == '-').trim_end_matches('<');
            let name = word.trim_start_matches('<');
            if name.len() > 1 && name.chars().any(char::is_alphabetic) && !COMMON.contains(&name) {
                terms.push(word.to_owned());
            }
        }
    }
    terms
}

fn is_word(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Byte offsets where `term` appears in `text` as a whole word.
fn occurrences<'a>(text: &'a str, term: &'a str) -> impl Iterator<Item = usize> + 'a {
    text.match_indices(term).map(|(at, _)| at).filter(move |&at| {
        let before = text[..at].chars().next_back();
        let after = text[at + term.len()..].chars().next();
        (!term.starts_with(is_word) || !before.is_some_and(is_word)) && !after.is_some_and(is_word)
    })
}

/// Marks each code line a term appears on. A term that opens a tag marks the
/// whole opening tag, which may run over several lines.
fn lines_for(source: &str, lines: &[&str], terms: &[String]) -> Vec<bool> {
    let mut marked = vec![false; lines.len()];
    for term in terms {
        let mut start = 0;
        for (index, line) in lines.iter().enumerate() {
            if is_code(line) {
                for at in occurrences(line, term) {
                    let opens_tag = term.starts_with('<') || line[..at].ends_with('<');
                    let last = if opens_tag { index + tag_lines(&source[start + at..]) } else { index };
                    for mark in marked.iter_mut().take(last + 1).skip(index) {
                        *mark = true;
                    }
                }
            }
            start += line.len() + 1;
        }
    }
    for (mark, line) in marked.iter_mut().zip(lines) {
        *mark &= !line.trim().is_empty();
    }
    marked
}

/// How many line breaks an opening tag spans, reading from its name to the
/// `>` that ends it. Braces and strings are skipped, so a `>` in a closure
/// or a label does not end the tag.
fn tag_lines(rest: &str) -> usize {
    let (mut depth, mut quoted, mut breaks) = (0_usize, false, 0);
    for c in rest.chars() {
        match c {
            '\n' => breaks += 1,
            '"' if depth == 0 => quoted = !quoted,
            _ if quoted => {}
            '{' => depth += 1,
            '}' => depth = depth.saturating_sub(1),
            '>' if depth == 0 => return breaks,
            _ => {}
        }
    }
    0
}

/// `[false, true, true, false, true]` as `"2-3, 5"`.
fn ranges(marked: &[bool]) -> String {
    let mut out: Vec<String> = Vec::new();
    let mut index = 0;
    while index < marked.len() {
        if !marked[index] {
            index += 1;
            continue;
        }
        let from = index;
        while index + 1 < marked.len() && marked[index + 1] {
            index += 1;
        }
        out.push(if from == index { format!("{}", from + 1) } else { format!("{}-{}", from + 1, index + 1) });
        index += 1;
    }
    out.join(", ")
}

#[cfg(test)]
mod tests {
    use super::marked;

    const SOURCE: &str = r#"use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let on_change = Callback::noop();
    html! {
        <Stack>
            <Select
                label="Range"
                {on_change}
            />
            <Button pressed=true>{ "Pin" }</Button>
        </Stack>
    }
}
"#;

    #[test]
    fn marks_what_the_description_names() {
        for (description, expected) in [
            ("`pressed` makes a toggle.", "13"),
            ("Reports through `on_change`.", "6, 11"),
            ("A `Select` and a `Button`.", "9-13"),
            ("Give `Select::label(..)` a name.", "9-12"),
        ] {
            assert_eq!(marked(SOURCE, description, &["Select"]), expected, "{description}");
        }
    }

    #[test]
    fn falls_back_to_the_components_of_the_page() {
        assert_eq!(marked(SOURCE, "Nothing is named here.", &["Select"]), "9-12");
        assert_eq!(marked(SOURCE, "Names `something_else`.", &["Button"]), "13");
    }

    #[test]
    fn marks_nothing_rather_than_most_of_the_file() {
        let source = "html! {\n    <Pill>{ \"a\" }</Pill>\n    <Pill>{ \"b\" }</Pill>\n    <Pill>{ \"c\" }</Pill>\n}";
        assert_eq!(marked(source, "Each `Pill` is a state.", &["Pill"]), "");
        assert_eq!(marked(SOURCE, "No names.", &["Missing"]), "");
    }
}
