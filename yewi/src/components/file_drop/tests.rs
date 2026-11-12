use yew::prelude::*;
use yewi_test::render;

use super::*;
use crate::{Field, hooks::reset_ids};

fn file(name: &'static str, size: u64, mime: &'static str) -> SelectedFile {
    SelectedFile::new(name, size, mime)
}

fn names(files: &[SelectedFile]) -> Vec<&str> {
    files.iter().map(|file| file.name.as_str()).collect()
}

/// The value of `name` on the first `<tag ...>` in `html`.
fn attr<'a>(html: &'a str, tag: &str, name: &str) -> Option<&'a str> {
    let start = html.find(&format!("<{tag} "))?;
    let element = &html[start..start + html[start..].find('>')?];
    let value = &element[element.find(&format!(" {name}=\""))? + name.len() + 3..];
    Some(&value[..value.find('"')?])
}

fn render_fresh(view: fn() -> Html) -> String {
    reset_ids();
    render(view)
}

#[test]
fn accept_matches_extensions_media_types_and_wildcards() {
    let cases = [
        (".zip", "pack.ZIP", "", true),
        (".zip", "pack.tar", "application/zip", false),
        ("application/zip", "pack", "application/zip", true),
        ("image/*", "map.png", "image/png", true),
        ("image/*", "notes.txt", "text/plain", false),
        ("image/*", "unknown", "", false),
        (".kml, image/* ,application/pdf", "report.pdf", "application/pdf", true),
        (".kml, image/*", "route.KML", "", true),
        ("", "anything", "", true),
    ];
    for (accept, name, mime, expected) in cases {
        assert_eq!(accepts(accept, &file(name, 1, mime)), expected, "{accept:?} against {name:?} ({mime:?})");
    }
}

#[test]
fn screening_keeps_matching_files_in_order_and_explains_the_rest() {
    let files = vec![
        file("a.zip", 10, "application/zip"),
        file("b.txt", 10, "text/plain"),
        file("c.zip", 500, "application/zip"),
        file("d.zip", 20, "application/zip"),
    ];
    let (accepted, rejected) = screen(files, Some(".zip"), Some(100), true);

    assert_eq!(names(&accepted), ["a.zip", "d.zip"]);
    let reasons: Vec<_> = rejected.iter().map(|r| (r.file.name.as_str(), r.reason)).collect();
    assert_eq!(reasons, [("b.txt", FileRejection::Type), ("c.zip", FileRejection::TooLarge)]);
}

#[test]
fn a_single_file_zone_keeps_only_the_first_acceptable_file() {
    let files = vec![file("big.zip", 500, ""), file("a.zip", 1, ""), file("b.zip", 1, "")];
    let (accepted, rejected) = screen(files, None, Some(100), false);

    assert_eq!(names(&accepted), ["a.zip"]);
    assert_eq!(rejected.last().map(|r| r.reason), Some(FileRejection::TooMany));
}

#[test]
fn renders_a_native_file_input_named_by_its_label_and_described_by_its_hint() {
    let html = render_fresh(|| html! { <FileDrop label="Drop a package" hint="Zip files only" accept=".zip" multiple=true /> });

    assert_eq!(attr(&html, "input", "type"), Some("file"));
    assert_eq!(attr(&html, "input", "accept"), Some(".zip"));
    assert!(html.contains(" multiple"));
    let label = attr(&html, "input", "aria-labelledby").expect("input is labelled");
    assert!(html.contains(&format!(r#"id="{label}""#)) && html.contains("Drop a package"));
    let hint = attr(&html, "input", "aria-describedby").expect("input is described");
    assert!(html.contains(&format!(r#"id="{hint}""#)) && html.contains("Zip files only"));
}

#[test]
fn the_default_label_follows_multiple() {
    let one = render(|| html! { <FileDrop /> });
    let many = render(|| html! { <FileDrop multiple=true /> });
    assert!(one.contains("Drop a file"));
    assert!(many.contains("Drop files"));
}

#[test]
fn busy_and_disabled_block_the_input() {
    let busy = render(|| html! { <FileDrop busy=true busy_label="Sending" label="Drop here" /> });
    assert!(busy.contains(r#"aria-busy="true""#));
    assert!(busy.contains("Sending") && !busy.contains("Drop here"));
    assert!(busy.contains(" disabled"));

    let disabled = render(|| html! { <FileDrop disabled=true /> });
    assert!(disabled.contains(" disabled"));
    assert!(!disabled.contains("aria-busy"));
}

#[test]
fn a_field_supplies_the_id_name_description_and_state() {
    let html = render_fresh(|| {
        html! {
            <Field label="Package" error="Choose a package." required=true>
                <FileDrop hint="Zip files only" />
            </Field>
        }
    });

    assert_eq!(attr(&html, "input", "id"), attr(&html, "label", "for"));
    assert_eq!(attr(&html, "input", "aria-invalid"), Some("true"));
    assert_eq!(attr(&html, "input", "aria-required"), Some("true"));
    let field_label = attr(&html, "label", "id").unwrap();
    assert!(attr(&html, "input", "aria-labelledby").unwrap().starts_with(field_label));
    let error = attr(&html, "p", "id").unwrap();
    let described_by = attr(&html, "input", "aria-describedby").unwrap();
    assert!(described_by.split(' ').any(|id| id == error) && described_by.split(' ').count() == 2);
}

#[test]
fn id_class_and_attrs_reach_the_root() {
    let html = render(|| html! { <FileDrop id="upload" class="mine" attrs={crate::attrs! { "data-kind" => "pkg" }} /> });
    assert!(html.starts_with("<label"));
    assert_eq!(attr(&html, "label", "id"), Some("upload"));
    assert!(html.contains("mine") && html.contains(r#"data-kind="pkg""#));
}
