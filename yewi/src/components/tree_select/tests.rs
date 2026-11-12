use yewi_test::render;

use super::{
    tree::{Row, Tree, rows},
    *,
};
use crate::{Field, attrs};

/// A slice of a symbol catalogue: `Unit` is never listed by itself.
fn catalogue() -> TreeOptions {
    let entry = |path: &[&'static str], value: &'static str| TreeOption::new(path.iter().copied(), value).detail(value);
    [
        entry(&["Ground track"], "G"),
        entry(&["Ground track", "Unit", "Combat"], "G-U-C"),
        entry(&["Ground track", "Unit", "Combat", "Infantry"], "G-U-C-I"),
        entry(&["Ground track", "Unit", "Combat", "Infantry", "Infantry mortar"], "G-U-C-I-O"),
        entry(&["Air track"], "A"),
        entry(&["Air track", "Military", "Rotary wing"], "A-M-H"),
    ]
    .into()
}

fn labels<'a>(tree: &'a Tree, found: &[usize]) -> Vec<&'a str> {
    found.iter().filter_map(|at| tree.node(*at)).map(|node| &*node.label).collect()
}

#[test]
fn shows_the_chosen_entry_as_a_collapsed_combobox() {
    let html = render(|| html! { <TreeSelect options={catalogue()} value="G-U-C-I" label="Symbol" /> });
    assert!(html.contains(r#"role="combobox""#));
    assert!(html.contains(r#"aria-expanded="false""#));
    assert!(html.contains(r#"aria-label="Symbol""#));
    assert!(html.contains("Infantry"));
    assert!(html.contains("G-U-C-I"));
    // The panel exists only while open.
    assert!(!html.contains(r#"role="listbox""#));
    assert!(!html.contains("Air track"));
}

#[test]
fn shows_the_placeholder_for_nothing_and_an_unlisted_value_as_it_stands() {
    let empty = render(|| html! { <TreeSelect options={catalogue()} placeholder="Choose a symbol" /> });
    assert!(empty.contains("Choose a symbol"));
    let unlisted = render(|| html! { <TreeSelect options={catalogue()} value="X-9" placeholder="Choose a symbol" /> });
    assert!(unlisted.contains("X-9"));
    assert!(!unlisted.contains("Choose a symbol"));
}

#[test]
fn render_media_draws_the_chosen_entrys_picture() {
    let options: TreeOptions = [TreeOption::new(["Sea", "Vessel"], "S").media("ship.svg")].into();
    let render_media = Callback::from(|media: AttrValue| html! { <img src={media} alt="" /> });
    let html = render(move || html! { <TreeSelect options={options.clone()} value="S" render_media={render_media.clone()} /> });
    assert!(html.contains(r#"src="ship.svg""#));
}

#[test]
fn name_submits_the_value_in_a_native_form() {
    let html = render(|| html! { <TreeSelect options={catalogue()} value="A-M-H" name="symbol" /> });
    assert!(html.contains(r#"type="hidden""#));
    assert!(html.contains(r#"name="symbol""#));
    assert!(html.contains(r#"value="A-M-H""#));
}

#[test]
fn a_field_labels_describes_and_invalidates_the_control() {
    let html = render(|| {
        html! { <Field label="Symbol" error="Choose a symbol" required=true><TreeSelect options={catalogue()} /></Field> }
    });
    assert!(html.contains(r#"aria-invalid="true""#));
    assert!(html.contains(r#"aria-required="true""#));
    assert!(html.contains("aria-describedby="));
    let id_at = html.find("for=\"").expect("the label points at a control") + 5;
    let id = &html[id_at..id_at + html[id_at..].find('"').unwrap()];
    assert!(html.contains(&format!(r#"id="{id}""#)));
}

#[test]
fn disabled_uses_the_native_attribute() {
    let html = render(|| html! { <TreeSelect options={catalogue()} disabled=true /> });
    assert!(html.contains(" disabled"));
}

#[test]
fn class_and_attrs_reach_the_root_and_id_the_control() {
    let html = render(|| {
        html! { <TreeSelect options={catalogue()} id="symbol" class="mine" attrs={attrs! { "data-test" => "x" }} /> }
    });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains("mine"));
    assert!(root.contains(r#"data-test="x""#));
    assert!(html.contains(r#"id="symbol""#));
}

#[test]
fn a_step_nobody_listed_can_be_opened_but_not_chosen() {
    let options = catalogue();
    let tree = options.tree();
    assert_eq!(labels(tree, tree.level(None)), ["Ground track", "Air track"]);

    let ground = tree.find("G").expect("the ground track");
    let unit = tree.node(tree.level(Some(ground))[0]).expect("the branch");
    assert_eq!((&*unit.label, unit.value.as_deref()), ("Unit", None));
    assert_eq!(unit.children.len(), 1);

    let infantry = tree.find("G-U-C-I").expect("infantry");
    assert_eq!(labels(tree, &tree.trail(infantry)), ["Ground track", "Unit", "Combat"]);
    assert_eq!(tree.find("G-X"), None);
}

#[test]
fn a_search_prefers_the_name_typed_and_the_broader_of_two() {
    let options = catalogue();
    let tree = options.tree();
    let cases = [
        // Starts with it, broader first.
        ("inf", vec!["Infantry", "Infantry mortar"]),
        // Every word, anywhere on the way down.
        ("air rotary", vec!["Rotary wing"]),
        // By its detail, in either case.
        ("g-u-c-i-o", vec!["Infantry mortar"]),
        // A branch that was never an entry is not an answer, but leads to some.
        ("unit", vec!["Combat", "Infantry", "Infantry mortar"]),
        ("   ", vec![]),
        ("submarine", vec![]),
    ];
    for (query, expected) in cases {
        assert_eq!(labels(tree, &tree.search(query, 10).0), expected, "{query:?}");
    }
    // The cap limits what is shown, not what is counted.
    let (shown, total) = tree.search("track", 2);
    assert_eq!((shown.len(), total), (2, 6));
}

#[test]
fn nothing_typed_lists_one_level_and_something_typed_the_whole_tree() {
    let options = catalogue();
    let tree = options.tree();
    let ground = tree.find("G");

    assert_eq!(rows(tree, "", None, false, None, 10).0.len(), 2);
    assert_eq!(rows(tree, "  ", ground, false, None, 10).0.len(), 1);
    assert_eq!(rows(tree, "mortar", None, false, None, 10).0, [Row::Node(tree.find("G-U-C-I-O").unwrap())]);
}

#[test]
fn clearing_is_offered_at_the_top_and_typed_text_only_when_it_is_new() {
    let options = catalogue();
    let tree = options.tree();

    assert_eq!(rows(tree, "", None, true, None, 10).0[0], Row::Clear);
    assert!(!rows(tree, "", tree.find("G"), true, None, 10).0.contains(&Row::Clear));
    assert!(!rows(tree, "air", None, true, None, 10).0.contains(&Row::Clear));

    let typed = |value: &'static str| rows(tree, value, None, false, Some(value.into()), 10).0;
    assert_eq!(typed("G-X").last(), Some(&Row::Custom("G-X".into())));
    assert!(!typed("A-M-H").iter().any(|row| matches!(row, Row::Custom(_))));
}
