use yewi_test::render;

use super::*;
use crate::attrs;

const BEFORE: &str = "a\nb\nc\nd\ne\nf\ng\nh\ni\nj\nk\nl\n";
const AFTER: &str = "a\nb\nc\nd\ne\nF\ng\nh\ni\nj\nk\nl\nm\n";
const PATCH: &str = "--- a/job.hcl\n+++ b/job.hcl\n@@ -1,3 +1,3 @@\n datacenter = \"home\"\n-count = 1\n+count = 2\n --- not a header\n\\ No newline at end of file\n";

/// Rebuilds one side of a diff from its rows.
fn side(lines: &[DiffLine], drop: DiffLineKind) -> String {
    lines.iter().filter(|l| l.kind != drop).map(|l| format!("{}\n", l.text)).collect()
}

fn kinds(lines: &[DiffLine]) -> Vec<DiffLineKind> {
    lines.iter().map(|l| l.kind).collect()
}

#[test]
fn a_computed_diff_reproduces_both_texts() {
    let cases = [
        ("", ""),
        ("", "a\nb\n"),
        ("a\nb\n", ""),
        ("a\nb\nc\n", "a\nb\nc\n"),
        ("a\nb\nc\n", "a\nx\nc\n"),
        ("a\nb\nc\nd\n", "d\nc\nb\na\n"),
        ("x\na\nb\nc\n", "a\nb\nc\ny\n"),
        ("a\na\nb\na\n", "a\nb\na\na\nc\n"),
        (BEFORE, AFTER),
    ];
    for (before, after) in cases {
        let lines = diff_lines(before, after);
        assert_eq!(side(&lines, DiffLineKind::Added), before, "old side of {before:?} -> {after:?}");
        assert_eq!(side(&lines, DiffLineKind::Removed), after, "new side of {before:?} -> {after:?}");
    }
}

#[test]
fn a_computed_diff_is_minimal_and_numbered() {
    let lines = diff_lines(BEFORE, AFTER);
    let changes: Vec<_> = lines.iter().filter(|l| l.kind != DiffLineKind::Context).collect();
    assert_eq!(changes.len(), 3);
    assert_eq!((changes[0].kind, &*changes[0].text, changes[0].old), (DiffLineKind::Removed, "f", Some(6)));
    assert_eq!((changes[1].kind, &*changes[1].text, changes[1].new), (DiffLineKind::Added, "F", Some(6)));
    assert_eq!((changes[2].kind, changes[2].new), (DiffLineKind::Added, Some(13)));
}

#[test]
fn texts_too_different_for_a_shortest_script_still_diff() {
    let before: String = (0..2000).map(|i| format!("old {i}\n")).collect();
    let after: String = (0..2000).map(|i| format!("new {i}\n")).collect();
    let lines = diff_lines(&before, &after);
    assert_eq!(side(&lines, DiffLineKind::Added), before);
    assert_eq!(side(&lines, DiffLineKind::Removed), after);
}

#[test]
fn a_unified_diff_is_classified_and_numbered() {
    use DiffLineKind::*;
    let lines = parse_unified_diff(PATCH);
    assert_eq!(kinds(&lines), [File, File, Hunk, Context, Removed, Added, Context, Meta]);
    assert_eq!(&*lines[4].text, "count = 1");
    assert_eq!((lines[4].old, lines[4].new), (Some(2), None));
    assert_eq!((lines[5].old, lines[5].new), (None, Some(2)));
    // Inside a hunk, a line that looks like a file header is content.
    assert_eq!(&*lines[6].text, "--- not a header");
}

#[test]
fn a_patch_without_hunk_headers_is_read_by_its_markers() {
    use DiffLineKind::*;
    assert_eq!(kinds(&parse_unified_diff(" same\n-old\n+new\n")), [Context, Removed, Added]);
}

#[test]
fn renders_added_and_removed_lines_semantically_with_stats() {
    let html = render(|| html! { <Diff title="letters.txt" before={BEFORE} after={AFTER} /> });
    assert!(html.starts_with("<figure"));
    assert!(html.contains("letters.txt"));
    assert!(html.contains("<del") && html.contains("<ins"));
    assert!(html.contains("2 lines added, 1 line removed"));
}

#[test]
fn unchanged_runs_fold_to_the_requested_context() {
    let html = render(|| html! { <Diff before={BEFORE} after={AFTER} context={1} /> });
    assert!(html.contains("Show 4 unchanged lines"));
    assert!(html.contains(">e<") && !html.contains(">d<"));

    let html = render(|| html! { <Diff before={BEFORE} after={AFTER} context={100} /> });
    assert!(!html.contains("unchanged lines"));
    assert!(html.contains(">a<"));
}

#[test]
fn identical_texts_say_so() {
    let html = render(|| html! { <Diff before={BEFORE} after={BEFORE} /> });
    assert!(html.contains("No changes"));
    assert!(!html.contains("<ins") && !html.contains("<del"));
}

#[test]
fn a_unified_diff_takes_its_title_from_the_file_header() {
    let html = render(|| html! { <Diff unified={PATCH} /> });
    assert!(html.contains("job.hcl"));
    assert!(!html.contains("+++"));
    assert!(html.contains("@@ -1,3 +1,3 @@"));
    assert!(html.contains("1 line added, 1 line removed"));
}

#[test]
fn an_edited_line_marks_only_what_changed() {
    let html = render(|| html! { <Diff unified={PATCH} /> });
    assert!(html.contains("<mark"));
    assert!(html.contains(">1</mark>") && html.contains(">2</mark>"));
}

#[test]
fn render_line_draws_the_text_of_every_line() {
    let render_line = Callback::from(|text: AttrValue| html! { <b>{ text.to_uppercase() }</b> });
    let html = render(move || html! { <Diff unified={PATCH} render_line={render_line.clone()} /> });
    assert!(html.contains("<b>COUNT = 2</b>"));
    assert!(!html.contains("<mark"));
}

#[test]
fn every_mode_renders_every_line() {
    for mode in DiffMode::ALL {
        let html = render(move || html! { <Diff unified={PATCH} {mode} /> });
        for text in ["count = 1", "count = 2", "No newline at end of file"] {
            assert!(html.replace("<mark class=\"yi-diff__changed\">", "").replace("</mark>", "").contains(text), "{mode:?}: {text}");
        }
    }
}

#[test]
fn line_numbers_can_be_hidden() {
    let numbered = render(|| html! { <Diff unified={PATCH} /> });
    let plain = render(|| html! { <Diff unified={PATCH} line_numbers={false} /> });
    assert!(numbered.contains("data-n="));
    assert!(!plain.contains("data-n="));
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html = render(|| {
        html! { <Diff id="plan" class="mine" attrs={attrs! { "data-step" => "3" }} unified={PATCH} /> }
    });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="plan""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"data-step="3""#));
}
