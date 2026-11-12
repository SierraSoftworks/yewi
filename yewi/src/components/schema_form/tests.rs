use yewi_test::render;

use super::{
    schema::{self, Kind, TextKind},
    *,
};

fn json(text: &str) -> Json {
    Json::parse(text).expect("valid JSON")
}

/// What `schemars` derives for a feed plugin's configuration: an optional,
/// internally tagged `Area` behind a `$ref`.
fn feed() -> Json {
    json(
        r##"{
        "type": "object",
        "properties": {
            "area": {
                "description": "Where the feed\n  is looking.\n\nSee `Area` for the shapes.",
                "anyOf": [{ "$ref": "#/$defs/Area" }, { "type": "null" }]
            },
            "api_key": { "type": "string", "format": "password" },
            "days": { "title": "Days back", "type": "integer", "minimum": 1, "maximum": 5 },
            "sensors": { "type": "array", "items": { "enum": ["modis", "viirs"] } },
            "tags": { "type": "array", "items": { "type": "string" } },
            "stations": { "type": "array", "items": { "type": "integer" } },
            "symbols": { "oneOf": [{ "const": "2525d", "title": "MIL-STD-2525D" }, { "const": "app6" }] },
            "labels": { "type": "object", "additionalProperties": { "type": "string" } }
        },
        "required": ["api_key"],
        "$defs": { "Area": { "oneOf": [
            {
                "type": "object",
                "properties": {
                    "kind": { "type": "string", "const": "bbox" },
                    "south": { "type": "number" }, "north": { "type": "number" }
                },
                "required": ["kind", "south", "north"]
            },
            {
                "type": "object",
                "properties": {
                    "kind": { "type": "string", "const": "circle" },
                    "lat": { "type": "number", "minimum": -90 },
                    "radius_km": { "type": "number", "default": 50.0 }
                },
                "required": ["kind", "lat", "radius_km"]
            }
        ] } }
    }"##,
    )
}

fn property<'a>(root: &'a Json, key: &str) -> &'a Json {
    schema::get(schema::get(root, "properties").unwrap(), key).unwrap()
}

#[test]
fn each_shape_a_settings_struct_derives_gets_the_control_it_wants() {
    let root = feed();
    let kind = |key: &str| schema::kind(&root, property(&root, key));

    assert!(matches!(schema::kind(&root, &root), Kind::Object(fields) if fields.len() == 8));
    assert_eq!(kind("api_key"), Kind::Text(TextKind::Secret));
    assert_eq!(kind("days"), Kind::Integer);
    assert!(matches!(kind("sensors"), Kind::Tags(Some(choices)) if choices.len() == 2));
    assert_eq!(kind("tags"), Kind::Tags(None));
    assert!(matches!(kind("stations"), Kind::List(_)));
    assert!(matches!(kind("symbols"), Kind::Choice(values) if values.len() == 2));
    assert_eq!(kind("labels"), Kind::Unknown, "a map is a JSON box");

    let Kind::Variants(variants) = kind("area") else {
        panic!("an optional tagged enum behind a $ref is still a tagged enum");
    };
    assert_eq!(variants.iter().map(|v| v.label.as_str()).collect::<Vec<_>>(), ["Bbox", "Circle"]);
    let fields = variants[1].fields();
    assert_eq!(fields.iter().map(|p| p.key.as_str()).collect::<Vec<_>>(), ["lat", "radius_km"], "the tag is the picker's");
    assert_eq!(schema::summary(&root, property(&root, "area")).as_deref(), Some("Where the feed is looking."));
}

#[test]
fn string_formats_choose_the_text_control() {
    let cases = [
        (r#"{"type":"string"}"#, TextKind::Plain),
        (r#"{"type":"string","writeOnly":true}"#, TextKind::Secret),
        (r#"{"type":"string","format":"multiline"}"#, TextKind::Multiline),
        (r#"{"type":["string","null"],"format":"email"}"#, TextKind::Email),
        (r#"{"type":"string","format":"uri"}"#, TextKind::Url),
    ];
    for (source, expected) in cases {
        let node = json(source);
        assert_eq!(schema::kind(&node, &node), Kind::Text(expected), "{source}");
    }
}

#[test]
fn a_new_value_starts_from_the_schemas_defaults_and_keeps_what_carries_over() {
    let root = feed();
    let Kind::Variants(variants) = schema::kind(&root, property(&root, "area")) else {
        panic!("variants");
    };
    assert_eq!(schema::default_for(&root, &root), json(r#"{"api_key":""}"#));

    let circle = schema::switched(&root, &variants[1], None);
    assert_eq!(circle, json(r#"{"kind":"circle","lat":-90,"radius_km":50.0}"#));
    assert_eq!(schema::variant_of(&variants, &circle), Some(1));

    let moved = json(r#"{"kind":"circle","lat":51.5,"radius_km":120.0}"#);
    assert_eq!(schema::switched(&root, &variants[1], Some(&moved)), moved, "re-picking the same variant loses nothing");
    let boxed = schema::switched(&root, &variants[0], Some(&moved));
    assert_eq!(schema::variant_of(&variants, &boxed), Some(0), "the tag is never carried over");
    assert_eq!(schema::variant_of(&variants, &json(r#"{"kind":"hexagon"}"#)), None);
}

#[test]
fn a_schema_that_contains_itself_is_not_followed_forever() {
    let looped = json(r##"{"$ref":"#"}"##);
    assert_eq!(schema::kind(&looped, &looped), Kind::Unknown);

    let nested =
        json(r##"{"type":"object","properties":{"child":{"$ref":"#"}},"required":["child"]}"##);
    // Whatever it holds, it is finite: the point is that these return.
    assert!(matches!(schema::default_for(&nested, &nested), Json::Object(_)));
    assert!(render(move || html! { <SchemaForm schema={nested.clone()} /> }).contains("textarea"));
}

#[test]
fn edits_address_the_document_by_pointer() {
    let doc = json(r#"{"a":{"b":1},"list":[1,2,3],"x/y":true}"#);
    let cases = [
        ("/a/b", Some("2"), r#"{"a":{"b":2},"list":[1,2,3],"x/y":true}"#),
        ("/a/b", None, r#"{"a":{},"list":[1,2,3],"x/y":true}"#),
        ("/new/deep", Some("1"), r#"{"a":{"b":1},"list":[1,2,3],"x/y":true,"new":{"deep":1}}"#),
        ("/list/1", Some("9"), r#"{"a":{"b":1},"list":[1,9,3],"x/y":true}"#),
        ("/list/1", None, r#"{"a":{"b":1},"list":[1,3],"x/y":true}"#),
        ("/list/3", Some("4"), r#"{"a":{"b":1},"list":[1,2,3,4],"x/y":true}"#),
        ("/x~1y", Some("false"), r#"{"a":{"b":1},"list":[1,2,3],"x/y":false}"#),
        ("", Some("{}"), "{}"),
    ];
    for (pointer, value, expected) in cases {
        assert_eq!(schema::set_at(&doc, pointer, value.map(json)), json(expected), "{pointer} = {value:?}");
    }
    assert_eq!(schema::at(&doc, "/list/2"), Some(&json("3")));
    assert_eq!(schema::at(&doc, &schema::child("", "x/y")), Some(&Json::Bool(true)));
    assert_eq!(schema::set_at(&Json::Null, "/a", Some(json("1"))), json(r#"{"a":1}"#));
}

#[test]
fn numbers_are_written_as_json() {
    assert_eq!(schema::json_number(3.0, true), Some(json("3")));
    assert_eq!(schema::json_number(51.5, false), Some(json("51.5")));
    assert_eq!(schema::json_number(-0.0, false), Some(json("0")));
    assert_eq!(schema::json_number(f64::NAN, false), None);
}

#[test]
fn errors_accept_pointers_and_dotted_paths() {
    let errors = SchemaErrors::from([("/area/lat", "Too far south"), ("todoist.project", "Unknown project")]);
    assert_eq!(errors.get("/area/lat").map(AttrValue::as_str), Some("Too far south"));
    assert_eq!(errors.get("/todoist/project").map(AttrValue::as_str), Some("Unknown project"));
    assert_eq!(errors.get("/area"), None);
    assert!(SchemaErrors::default().is_empty());
}

#[test]
fn renders_a_labelled_control_for_each_property() {
    let value = json(r#"{"api_key":"s3cret","days":3,"sensors":["viirs"],"symbols":"2525d"}"#);
    let html = render(move || html! { <SchemaForm schema={feed()} value={value.clone()} /> });

    // Titles win over humanised keys; a required field is marked.
    for label in ["Api key", "Days back", "Sensors", "Tags", "Symbols", "Labels"] {
        assert!(html.contains(label), "label {label}");
    }
    assert!(html.contains(r#"type="password""#), "a password format is a secret input");
    assert!(html.contains(r#"value="3""#));
    assert!(html.contains("Remove viirs"), "an enum list is a tag picker");
    assert!(html.contains("MIL-STD-2525D"), "a choice shows the title beside its const");
    assert!(html.contains("Where the feed is looking."));
    assert!(!html.contains("See `Area`"), "only the first paragraph of a description is a hint");
    // An optional group that is absent shows its switch and no fields.
    assert!(html.contains(r#"aria-label="Include Area""#));
    assert!(!html.contains("Radius km"));
}

#[test]
fn a_tagged_union_shows_only_the_chosen_variants_fields() {
    let value = json(r#"{"area":{"kind":"circle","lat":51.5,"radius_km":120}}"#);
    let html = render(move || html! { <SchemaForm schema={feed()} value={value.clone()} /> });
    assert!(html.contains("Kind") && html.contains("Circle"));
    assert!(html.contains("Lat") && html.contains("Radius km"));
    assert!(!html.contains("South"));
}

#[test]
fn a_list_draws_an_item_per_value_with_remove_and_add() {
    let value = json(r#"{"stations":[7,9]}"#);
    let html = render(move || html! { <SchemaForm schema={feed()} value={value.clone()} /> });
    assert!(html.contains(r#"aria-label="Remove Item 1""#) && html.contains(r#"aria-label="Remove Item 2""#));
    assert!(!html.contains("Remove Item 3"));
    assert!(html.contains("Add"));
}

#[test]
fn errors_show_against_the_value_or_group_they_address() {
    let value = json(r#"{"days":9,"area":{"kind":"bbox","south":2,"north":1}}"#);
    let errors = SchemaErrors::from([("/days", "Must be at most 5"), ("/area", "South must be below north")]);
    let html =
        render(move || html! { <SchemaForm schema={feed()} value={value.clone()} errors={errors.clone()} /> });
    assert!(html.contains("Must be at most 5") && html.contains("South must be below north"));
    assert!(html.contains(r#"aria-invalid="true""#));
    assert_eq!(html.matches(r#"role="alert""#).count(), 2);
}

#[test]
fn render_replaces_a_control_and_keeps_its_label() {
    let render_control = Callback::from(|control: SchemaControl| {
        (control.pointer == "/days").then(|| html! { <output>{ "custom days" }</output> })
    });
    let html = render(move || html! { <SchemaForm schema={feed()} render={render_control.clone()} /> });
    assert!(html.contains("custom days") && html.contains("Days back"));
    assert!(html.contains(r#"type="password""#), "other controls are untouched");
}

#[test]
fn disabled_and_root_attributes_reach_the_markup() {
    let html = render(|| {
        html! { <SchemaForm schema={feed()} disabled=true id="config" class="mine" attrs={crate::attrs! { "data-test" => "x" }} /> }
    });
    assert!(html.contains(r#"id="config""#) && html.contains("mine") && html.contains(r#"data-test="x""#));
    assert!(html.contains("disabled"));
}
