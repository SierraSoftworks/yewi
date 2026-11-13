#![cfg(feature = "markdown")]
//! Markdown rendered with yewi's own components.
//!
//! [`Markdown`] parses CommonMark (plus tables, strikethrough and task
//! lists) with `pulldown-cmark` and builds the document from [`Heading`],
//! [`Link`], [`InlineCode`], [`Kbd`], [`SimpleTable`] and [`CodeBlock`], so
//! it looks like the rest of the app. Nothing in the source reaches the page
//! as markup: raw HTML is shown as text or dropped, and links and images
//! with a scheme outside a short allowlist are not rendered as such.

use pulldown_cmark::{Alignment, CodeBlockKind, Event, Options, Parser, Tag};
use yew::prelude::*;

use crate::{Attrs, CodeBlock, Heading, HeadingLevel, InlineCode, Kbd, Link, NavTarget, SimpleTable};

/// What happens to raw HTML in the source. Neither choice renders it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum MarkdownHtml {
    /// Shown as the text that was written, tags included.
    #[default]
    Escape,
    /// Left out of the document.
    Omit,
}

/// A link in the document that was activated.
#[derive(Debug, Clone, PartialEq)]
pub struct MarkdownLinkClick {
    /// The link's destination, as written in the source.
    pub href: AttrValue,
    /// The click. Call `prevent_default` to handle the navigation yourself.
    pub event: MouseEvent,
}

#[derive(Properties, PartialEq)]
pub struct MarkdownProps {
    /// The Markdown text. It may come from an untrusted author.
    pub source: AttrValue,
    /// The level a `#` heading renders at; deeper headings follow from it
    /// and stop at `<h6>`. Set it to the level below the heading the
    /// document sits under, so the page keeps one outline and embedded
    /// documents get proportionate headings.
    #[prop_or(HeadingLevel::H1)]
    pub top_heading: HeadingLevel,
    /// What happens to raw HTML in the source. `<kbd>` and `<br>` are
    /// understood either way.
    #[prop_or_default]
    pub html: MarkdownHtml,
    /// Shows images. When off, an image is replaced by its alternative text.
    /// Turn it off for untrusted authors: loading a remote image tells its
    /// host who is reading.
    #[prop_or(true)]
    pub images: bool,
    /// Opens links to other sites in a new tab and marks them with an arrow.
    /// Links within the site are unaffected.
    #[prop_or_default]
    pub external_links: bool,
    /// Wraps long lines in code blocks instead of scrolling them sideways.
    #[prop_or_default]
    pub wrap_code: bool,
    /// Fired when a link is activated, before the browser follows it. Use it
    /// to route links within the app on the client.
    #[prop_or_default]
    pub on_link_click: Callback<MarkdownLinkClick>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A Markdown document drawn with yewi's typography, tables and code blocks.
///
/// The text is parsed again only when `source` or an option changes. It
/// inherits the size and colour of where it is placed.
#[function_component]
pub fn Markdown(props: &MarkdownProps) -> Html {
    let options = RenderOptions {
        top_heading: props.top_heading,
        html: props.html,
        images: props.images,
        external_links: props.external_links,
        wrap_code: props.wrap_code,
        on_link_click: props.on_link_click.clone(),
    };
    let body = use_memo((props.source.clone(), options), |(source, options)| render(source, options));

    props.attrs.apply(html! {
        <div id={props.id.clone()} class={classes!("yi-markdown", props.class.clone())}>
            { (*body).clone() }
        </div>
    })
}

#[derive(PartialEq)]
struct RenderOptions {
    top_heading: HeadingLevel,
    html: MarkdownHtml,
    images: bool,
    external_links: bool,
    wrap_code: bool,
    on_link_click: Callback<MarkdownLinkClick>,
}

/// Where a URL may be used.
#[derive(Clone, Copy, PartialEq)]
enum UrlUse {
    Link,
    Image,
}

/// The URL if it is safe to use, cleaned the way a browser would read it.
///
/// Relative URLs pass. A URL with a scheme passes only for `http`, `https`
/// and, for links, `mailto` and `tel`; so `javascript:`, `data:` and the
/// like never reach an attribute.
fn safe_url(url: &str, usage: UrlUse) -> Option<String> {
    // Browsers drop tabs and line breaks anywhere in a URL and ignore leading
    // control characters and spaces, so judge what is left.
    let cleaned: String = url.chars().filter(|c| !matches!(c, '\t' | '\n' | '\r')).collect();
    let cleaned = cleaned.trim_matches(|c: char| c.is_ascii_control() || c == ' ');
    let scheme = match cleaned.find([':', '/', '?', '#']) {
        Some(end) if cleaned.as_bytes()[end] == b':' => cleaned[..end].to_ascii_lowercase(),
        _ => return Some(cleaned.to_owned()),
    };
    let allowed = match usage {
        UrlUse::Link => matches!(scheme.as_str(), "http" | "https" | "mailto" | "tel"),
        UrlUse::Image => matches!(scheme.as_str(), "http" | "https"),
    };
    allowed.then(|| cleaned.to_owned())
}

/// Whether a safe URL leaves the site: it names a host of its own.
fn is_external(url: &str) -> bool {
    let lower = url.to_ascii_lowercase();
    lower.starts_with("http:") || lower.starts_with("https:") || lower.starts_with("//")
}

/// An element being built: what it is and the content collected so far.
struct Frame {
    kind: Kind,
    children: Vec<Html>,
    /// Plain text, for the kinds that are built from text alone.
    text: String,
}

enum Kind {
    Root,
    Paragraph,
    Heading(HeadingLevel),
    BlockQuote,
    CodeBlock(String),
    HtmlBlock,
    List(Option<u64>),
    Item { task: bool },
    Table(Vec<Alignment>),
    TableHead,
    TableRow,
    TableCell,
    Emphasis,
    Strong,
    Strikethrough,
    Superscript,
    Subscript,
    Link { href: String, title: String },
    Image { src: String, title: String },
    /// `<kbd>` written as inline HTML.
    Kbd,
    /// A construct that is not enabled; its content passes through.
    Other,
}

struct Renderer<'a> {
    options: &'a RenderOptions,
    stack: Vec<Frame>,
    /// Inside a table head, where cells are column headers.
    in_head: bool,
    /// The column of the cell being built.
    column: usize,
}

fn render(source: &str, options: &RenderOptions) -> Html {
    let mut parser_options = Options::empty();
    parser_options.insert(Options::ENABLE_TABLES);
    parser_options.insert(Options::ENABLE_STRIKETHROUGH);
    parser_options.insert(Options::ENABLE_TASKLISTS);

    let mut renderer = Renderer { options, stack: vec![Frame::new(Kind::Root)], in_head: false, column: 0 };
    for event in Parser::new_ext(source, parser_options) {
        renderer.event(event);
    }
    renderer.finish()
}

impl Frame {
    fn new(kind: Kind) -> Self {
        Frame { kind, children: Vec::new(), text: String::new() }
    }
}

fn level(heading: pulldown_cmark::HeadingLevel, top: HeadingLevel) -> HeadingLevel {
    let top = HeadingLevel::ALL.iter().position(|level| *level == top).unwrap_or(0);
    HeadingLevel::ALL[(top + heading as usize - 1).min(5)]
}

impl Renderer<'_> {
    fn top(&mut self) -> &mut Frame {
        self.stack.last_mut().expect("the root frame is never popped")
    }

    /// Adds text to the element being built. Images and code blocks keep it
    /// as plain text; everything else takes a text node.
    fn text(&mut self, text: &str) {
        if let Some(image) = self.stack.iter_mut().rev().find(|frame| matches!(frame.kind, Kind::Image { .. })) {
            image.text.push_str(text);
            return;
        }
        let top = self.top();
        match top.kind {
            Kind::CodeBlock(_) | Kind::HtmlBlock => top.text.push_str(text),
            _ => top.children.push(html! { { text.to_owned() } }),
        }
    }

    fn push(&mut self, node: Html) {
        // Inside an image only the alternative text is kept.
        if !self.stack.iter().any(|frame| matches!(frame.kind, Kind::Image { .. })) {
            self.top().children.push(node);
        }
    }

    fn raw_html(&mut self, raw: &str) {
        if self.options.html == MarkdownHtml::Escape {
            self.text(raw);
        }
    }

    fn inline_html(&mut self, raw: &str) {
        let tag: String = raw.chars().filter(|c| !c.is_whitespace()).collect::<String>().to_ascii_lowercase();
        match tag.as_str() {
            "<kbd>" => self.stack.push(Frame::new(Kind::Kbd)),
            "</kbd>" if matches!(self.top().kind, Kind::Kbd) => self.close(),
            "<br>" | "<br/>" => self.push(html! { <br /> }),
            _ => self.raw_html(raw),
        }
    }

    /// A `<kbd>` that was never closed inside its parent is not a key after
    /// all: its content goes back to the parent as it was written.
    fn abandon_kbd(&mut self) {
        while matches!(self.top().kind, Kind::Kbd) {
            let frame = self.stack.pop().expect("checked above");
            self.raw_html("<kbd>");
            self.top().children.extend(frame.children);
        }
    }

    fn event(&mut self, event: Event<'_>) {
        match event {
            Event::Start(tag) => self.start(tag),
            Event::End(_) => {
                self.abandon_kbd();
                self.close();
            }
            Event::Text(text) => self.text(&text),
            Event::Code(code) => {
                if self.stack.iter().any(|frame| matches!(frame.kind, Kind::Image { .. })) {
                    self.text(&code);
                } else {
                    self.push(html! { <InlineCode>{ code.to_string() }</InlineCode> });
                }
            }
            Event::InlineMath(text) | Event::DisplayMath(text) => self.text(&text),
            Event::Html(raw) => self.raw_html(&raw),
            Event::InlineHtml(raw) => self.inline_html(&raw),
            Event::FootnoteReference(name) => self.text(&format!("[^{name}]")),
            Event::SoftBreak => self.text("\n"),
            Event::HardBreak => self.push(html! { <br /> }),
            Event::Rule => self.push(html! { <hr class="yi-divider" /> }),
            Event::TaskListMarker(checked) => {
                if let Some(item) = self.stack.iter_mut().rev().find(|frame| matches!(frame.kind, Kind::Item { .. })) {
                    item.kind = Kind::Item { task: true };
                }
                self.push(html! {
                    <input class="yi-markdown__task-box" type="checkbox"
                        {checked}
                        disabled=true
                        aria-label={if checked { "Completed" } else { "Not completed" }}
                    />
                });
            }
        }
    }

    fn start(&mut self, tag: Tag<'_>) {
        let kind = match tag {
            Tag::Paragraph => Kind::Paragraph,
            Tag::Heading { level: heading, .. } => Kind::Heading(level(heading, self.options.top_heading)),
            Tag::BlockQuote(_) => Kind::BlockQuote,
            Tag::CodeBlock(CodeBlockKind::Fenced(info)) => {
                // The language is the first word; the rest is free-form.
                Kind::CodeBlock(info.split([' ', ',', '\t']).next().unwrap_or_default().to_owned())
            }
            Tag::CodeBlock(CodeBlockKind::Indented) => Kind::CodeBlock(String::new()),
            Tag::HtmlBlock => Kind::HtmlBlock,
            Tag::List(start) => Kind::List(start),
            Tag::Item => Kind::Item { task: false },
            Tag::Table(alignments) => Kind::Table(alignments),
            Tag::TableHead => {
                self.in_head = true;
                self.column = 0;
                Kind::TableHead
            }
            Tag::TableRow => {
                self.column = 0;
                Kind::TableRow
            }
            Tag::TableCell => Kind::TableCell,
            Tag::Emphasis => Kind::Emphasis,
            Tag::Strong => Kind::Strong,
            Tag::Strikethrough => Kind::Strikethrough,
            Tag::Superscript => Kind::Superscript,
            Tag::Subscript => Kind::Subscript,
            Tag::Link { dest_url, title, .. } => Kind::Link { href: dest_url.into_string(), title: title.into_string() },
            Tag::Image { dest_url, title, .. } => Kind::Image { src: dest_url.into_string(), title: title.into_string() },
            _ => Kind::Other,
        };
        self.stack.push(Frame::new(kind));
    }

    /// The alignment class of the cell being built, from the table's
    /// delimiter row.
    fn cell_class(&self) -> Classes {
        let alignment = self.stack.iter().rev().find_map(|frame| match &frame.kind {
            Kind::Table(alignments) => alignments.get(self.column).copied(),
            _ => None,
        });
        let align = match alignment {
            Some(Alignment::Right) => Some("yi-table__cell--end"),
            Some(Alignment::Center) => Some("yi-table__cell--center"),
            _ => None,
        };
        classes!(align)
    }

    /// Finishes the element being built and hands it to its parent.
    fn close(&mut self) {
        if self.stack.len() < 2 {
            return;
        }
        let Frame { kind, children: mut nodes, text } = self.stack.pop().expect("checked above");
        // A table is its head followed by its rows, which need a body.
        let rows = match kind {
            Kind::Table(_) if !nodes.is_empty() => nodes.split_off(1),
            _ => Vec::new(),
        };
        let children: Html = nodes.into_iter().collect();
        let node = match kind {
            Kind::Root | Kind::Other => children,
            Kind::Paragraph => html! { <p>{ children }</p> },
            Kind::Heading(level) => html! { <Heading {level}>{ children }</Heading> },
            Kind::BlockQuote => html! { <blockquote class="yi-markdown__quote">{ children }</blockquote> },
            Kind::CodeBlock(language) => html! {
                <CodeBlock code={text} {language} wrap={self.options.wrap_code} />
            },
            Kind::HtmlBlock => match text.trim_end() {
                "" => Html::default(),
                text => html! { <p class="yi-markdown__html">{ text.to_owned() }</p> },
            },
            Kind::List(None) => html! { <ul>{ children }</ul> },
            Kind::List(Some(start)) => html! {
                <ol start={(start != 1).then(|| start.to_string())}>{ children }</ol>
            },
            Kind::Item { task } => html! { <li class={task.then_some("yi-markdown__task")}>{ children }</li> },
            Kind::Table(_) => html! {
                <SimpleTable wrap=true>
                    { children }
                    <tbody>{ for rows }</tbody>
                </SimpleTable>
            },
            Kind::TableHead => {
                self.in_head = false;
                html! { <thead><tr class="yi-table__row">{ children }</tr></thead> }
            }
            Kind::TableRow => html! { <tr class="yi-table__row">{ children }</tr> },
            Kind::TableCell => {
                let class = self.cell_class();
                self.column += 1;
                if self.in_head {
                    html! { <th scope="col" {class}>{ children }</th> }
                } else {
                    html! { <td {class}>{ children }</td> }
                }
            }
            Kind::Emphasis => html! { <em>{ children }</em> },
            Kind::Strong => html! { <strong class="yi-text">{ children }</strong> },
            Kind::Strikethrough => html! { <del>{ children }</del> },
            Kind::Superscript => html! { <sup>{ children }</sup> },
            Kind::Subscript => html! { <sub>{ children }</sub> },
            Kind::Kbd => html! { <Kbd>{ children }</Kbd> },
            Kind::Link { href, title } => match safe_url(&href, UrlUse::Link) {
                Some(href) => {
                    let external = self.options.external_links && is_external(&href);
                    let href = AttrValue::from(href);
                    let on_click = {
                        let on_link_click = self.options.on_link_click.clone();
                        let href = href.clone();
                        Callback::from(move |event| on_link_click.emit(MarkdownLinkClick { href: href.clone(), event }))
                    };
                    let attrs = if title.is_empty() { Attrs::default() } else { crate::attrs! { "title" => title } };
                    let href = NavTarget::href(href);
                    let href = if external { href.external() } else { href };
                    html! { <Link {href} {on_click} {attrs}>{ children }</Link> }
                }
                // A link that cannot be followed safely keeps its text.
                None => children,
            },
            Kind::Image { src, title } => match safe_url(&src, UrlUse::Image).filter(|_| self.options.images) {
                Some(src) => html! {
                    <img
                        class="yi-markdown__image"
                        {src}
                        alt={text}
                        title={(!title.is_empty()).then_some(title)}
                        loading="lazy"
                        referrerpolicy="no-referrer"
                    />
                },
                None => html! { { text } },
            },
        };
        // An image inside an image contributes only to the outer one's text.
        self.push(node);
    }

    fn finish(mut self) -> Html {
        self.abandon_kbd();
        while self.stack.len() > 1 {
            self.close();
        }
        self.stack.pop().map(|root| root.children.into_iter().collect()).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests;
