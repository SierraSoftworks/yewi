//! Server-render snapshots of every example, with nothing to write per
//! component. They catch unintended markup changes without a browser.
//!
//! A changed snapshot fails the test and writes `<name>.snap.new` beside the
//! old one. Review the difference, then accept with `cargo insta accept`, or
//! re-run with `INSTA_UPDATE=always`.

use yewi_site::pages::PAGES;

#[test]
fn examples_render_as_recorded() {
    let mut settings = insta::Settings::clone_current();
    settings.set_prepend_module_to_snapshot(false);
    settings.set_omit_expression(true);
    let _guard = settings.bind_to_scope();

    for page in PAGES {
        for example in page.def.examples {
            yewi::hooks::reset_ids();
            let html = yewi_test::render(example.render);
            // One element per line keeps the recorded files reviewable.
            let readable = html.replace("><", ">\n<");
            insta::assert_snapshot!(format!("{}__{}", page.slug, example.slug()), readable);
        }
    }
}

#[test]
fn every_page_has_examples_with_unique_names() {
    for page in PAGES {
        assert!(!page.def.examples.is_empty(), "{} has no examples", page.slug);
        let mut slugs: Vec<_> = page.def.examples.iter().map(|e| e.slug()).collect();
        slugs.sort();
        slugs.dedup();
        assert_eq!(slugs.len(), page.def.examples.len(), "{} has duplicate example names", page.slug);
    }
}

#[test]
fn pages_only_document_components_that_exist() {
    for page in PAGES {
        for module in page.def.api.unwrap_or_default() {
            assert!(
                yewi_site::api::API.iter().any(|api| api.module == *module),
                "{} documents `{module}`, which is not a directory under yewi/src/components",
                page.slug
            );
        }
    }
}

#[test]
fn generic_components_document_their_props() {
    for name in ["Table", "VirtualList", "VirtualTable"] {
        let component = yewi_site::api::API.iter().flat_map(|api| api.components).find(|c| c.name == name);
        assert!(component.is_some_and(|c| !c.props.is_empty()), "{name} has no documented props");
    }
}
