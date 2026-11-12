use yewi_test::render;

use super::*;
use crate::attrs;

fn markdown(source: &'static str) -> String {
    render(move || html! { <Markdown {source} /> })
}

#[test]
fn renders_the_document_structure() {
    let html = markdown("# Title\n\nSome *stressed* and **strong** text with `code`.\n\n- one\n- two\n\n1. first\n\n> quoted\n\n---\n");
    for expected in ["<h1", "Title", "<em>stressed</em>", "strong</strong>", "<code", "<ul>", "<ol>", "<blockquote", "<hr"] {
        assert!(html.contains(expected), "missing {expected} in {html}");
    }
}

#[test]
fn headings_start_at_the_top_heading_and_stop_at_h6() {
    let html = render(|| html! { <Markdown source={"# a\n\n## b\n\n#### c\n"} top_heading={HeadingLevel::H4} /> });
    assert!(html.contains("<h4") && html.contains("<h5") && html.contains("<h6"));
    assert!(!html.contains("<h1") && !html.contains("<h2"));
}

#[test]
fn fenced_code_becomes_a_code_block_named_by_its_language() {
    let html = markdown("```rust title=x\nfn main() {}\n```\n");
    assert!(html.contains("yi-code-block"));
    assert!(html.contains("main"));
    assert!(html.contains("Rust") || html.contains("rust"));
    assert!(!html.contains("title=x"));
}

#[test]
fn tables_have_column_headers_and_body_rows() {
    let html = markdown("| Name | Count |\n|:--|--:|\n| a | 1 |\n| b | 2 |\n");
    assert!(html.contains("yi-table"));
    assert_eq!(html.matches(r#"scope="col""#).count(), 2);
    assert!(html.contains("<tbody>"));
    assert_eq!(html.matches("<td").count(), 4);
    assert!(html.contains("yi-table__cell--end"));
}

#[test]
fn task_lists_show_their_state_read_only() {
    let html = markdown("- [x] done\n- [ ] open\n");
    assert_eq!(html.matches(r#"type="checkbox""#).count(), 2);
    assert_eq!(html.matches(" disabled").count(), 2);
    assert_eq!(html.matches(" checked").count(), 1);
}

#[test]
fn kbd_and_line_breaks_are_understood_as_inline_html() {
    let html = markdown("Press <kbd>Ctrl</kbd> then<br>wait. An open <kbd> alone is text.");
    assert!(html.contains("<kbd"));
    assert!(html.contains("Ctrl</kbd>"));
    assert!(html.contains("<br"));
    assert!(html.contains("&lt;kbd&gt; alone"));
}

#[test]
fn raw_html_is_never_rendered() {
    for source in [
        "<script>alert(1)</script>",
        "text <img src=x onerror=alert(1)> more",
        "<div onclick=\"alert(1)\">\n\nhi\n\n</div>",
        "<iframe src=\"https://example.com\"></iframe>",
        "<kbd onclick=alert(1)>x</kbd>",
    ] {
        let escaped = markdown(source);
        assert!(escaped.contains("&lt;"), "{source} should be shown as text: {escaped}");
        let omitted = render(move || html! { <Markdown {source} html={MarkdownHtml::Omit} /> });
        for html in [escaped, omitted] {
            for tag in ["<script", "<img", "<iframe", "<div onclick", "<kbd onclick"] {
                assert!(!html.contains(tag), "{source} rendered {tag}: {html}");
            }
        }
    }
}

#[test]
fn unsafe_urls_never_reach_an_attribute() {
    for url in [
        "javascript:alert(1)",
        "JaVaScRiPt:alert(1)",
        " javascript:alert(1)",
        "java\tscript:alert(1)",
        "java&#10;script:alert(1)",
        "&#1;javascript:alert(1)",
        "data:text/html,<script>alert(1)</script>",
        "vbscript:msgbox(1)",
        "file:///etc/passwd",
    ] {
        let source: &'static str = format!("[link](<{url}>) ![image](<{url}>)").leak();
        let html = markdown(source);
        assert!(!html.contains("href="), "{url} became a link: {html}");
        assert!(!html.contains("src="), "{url} became an image: {html}");
        assert!(html.contains("link") && html.contains("image"), "the text is kept: {html}");
    }
}

#[test]
fn safe_urls_are_kept() {
    for (url, usage) in [
        ("https://example.com/a?b#c", UrlUse::Image),
        ("/relative/path", UrlUse::Image),
        ("../up", UrlUse::Link),
        ("#anchor", UrlUse::Link),
        ("//cdn.example.com/x.png", UrlUse::Image),
        ("mailto:ops@example.com", UrlUse::Link),
        ("tel:+123", UrlUse::Link),
        ("docs/a:b", UrlUse::Link),
    ] {
        assert_eq!(safe_url(url, usage).as_deref(), Some(url));
    }
    assert_eq!(safe_url("data:image/png;base64,AAAA", UrlUse::Image), None);
    assert_eq!(safe_url("mailto:ops@example.com", UrlUse::Image), None);
}

#[test]
fn external_links_open_a_new_tab_only_when_asked_and_only_off_site() {
    let source = "[out](https://example.com) [in](/settings)";
    let plain = markdown(source);
    assert!(!plain.contains("_blank"));
    let html = render(move || html! { <Markdown {source} external_links=true /> });
    assert_eq!(html.matches(r#"target="_blank""#).count(), 1);
    assert!(html.contains("noopener"));
}

#[test]
fn images_can_be_reduced_to_their_alternative_text() {
    let source = "![A *fine* diagram](https://example.com/d.png \"Diagram\")";
    let shown = markdown(source);
    assert!(shown.contains(r#"alt="A fine diagram""#));
    assert!(shown.contains(r#"src="https://example.com/d.png""#));
    let hidden = render(move || html! { <Markdown {source} images=false /> });
    assert!(!hidden.contains("<img"));
    assert!(hidden.contains("A fine diagram"));
}

#[test]
fn text_in_the_source_is_escaped() {
    let html = markdown("a & b < c, `<Styles />`");
    assert!(html.contains("&amp;") && html.contains("&lt; c") && html.contains("&lt;Styles /&gt;"));
}

#[test]
fn empty_and_unbalanced_sources_render_without_panicking() {
    for source in ["", "   \n", "<kbd>", "*a <kbd>b* c</kbd>", "![a ![b](c)](d)", "| a |\n|--|\n", "- [ ]", "```"] {
        let html = markdown(source);
        assert!(html.contains("yi-markdown"));
    }
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html = render(|| html! { <Markdown source="x" id="m" class="mine" attrs={attrs! { "lang" => "en" }} /> });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="m""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"lang="en""#));
}
