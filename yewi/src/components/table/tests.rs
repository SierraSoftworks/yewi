use std::{cmp::Ordering, rc::Rc};

use yew::prelude::*;
use yewi_test::render;

use super::*;

#[derive(Clone, PartialEq)]
struct Host {
    name: &'static str,
    memory: u32,
}

fn hosts() -> Rc<[Host]> {
    Rc::from([
        Host { name: "web-10", memory: 16 },
        Host { name: "web-2", memory: 64 },
        Host { name: "db-1", memory: 16 },
    ])
}

fn columns() -> Rc<[Column<Host>]> {
    Rc::from([
        Column::new("host", "Host", |host: &Host| host.name).row_header().sort_text(|host| host.name),
        Column::new("memory", "Memory", |host: &Host| host.memory).sort_by_key(|host| host.memory).numeric(),
        Column::new("notes", "Notes", |_: &Host| "").sortable(),
    ])
}

/// The host names in `html`, in document order.
fn names(html: &str) -> Vec<&'static str> {
    let mut found: Vec<_> =
        hosts().iter().filter_map(|host| html.find(&format!(">{}<", host.name)).map(|at| (at, host.name))).collect();
    found.sort_unstable();
    found.into_iter().map(|(_, name)| name).collect()
}

fn table(sort: Vec<Sort>) -> String {
    render(move || html! { <Table<Host> columns={columns()} rows={hosts()} default_sort={sort.clone()} /> })
}

#[test]
fn renders_a_native_table_with_column_and_row_headers() {
    let html = table(vec![]);
    assert!(html.contains("<table"));
    assert_eq!(html.matches(r#"scope="col""#).count(), 3);
    assert_eq!(html.matches(r#"scope="row""#).count(), 3);
    assert_eq!(names(&html), ["web-10", "web-2", "db-1"], "unsorted rows keep the order given");
}

#[test]
fn sorts_by_the_typed_ordering_of_each_column() {
    let cases = [
        (vec![Sort::asc("host")], ["db-1", "web-2", "web-10"]),
        (vec![Sort::desc("host")], ["web-10", "web-2", "db-1"]),
        (vec![Sort::desc("memory")], ["web-2", "web-10", "db-1"]),
        (vec![Sort::asc("memory"), Sort::asc("host")], ["db-1", "web-10", "web-2"]),
        // No ordering of its own, and a column that does not exist: left to the caller.
        (vec![Sort::asc("notes")], ["web-10", "web-2", "db-1"]),
        (vec![Sort::asc("missing")], ["web-10", "web-2", "db-1"]),
    ];
    for (sort, expected) in cases {
        assert_eq!(names(&table(sort.clone())), expected, "{sort:?}");
    }
}

#[test]
fn the_leading_sort_column_reports_its_direction() {
    for (sort, expected) in [(Sort::asc("memory"), "ascending"), (Sort::desc("memory"), "descending")] {
        let html = table(vec![sort, Sort::asc("host")]);
        assert_eq!(html.matches(&format!(r#"aria-sort="{expected}""#)).count(), 1, "{html}");
        assert_eq!(html.matches(r#"aria-sort="ascending""#).count() + html.matches(r#"aria-sort="descending""#).count(), 1);
    }
    assert!(!table(vec![]).contains("aria-sort"));
}

#[test]
fn sortable_headers_are_buttons_and_others_are_not() {
    let sortable = table(vec![]);
    assert_eq!(sortable.matches("<button").count(), 3);

    let plain = render(|| {
        let columns: Rc<[Column<Host>]> = Rc::from([Column::new("host", "Host", |host: &Host| host.name)]);
        html! { <Table<Host> {columns} rows={hosts()} /> }
    });
    assert!(!plain.contains("<button"));
}

#[test]
fn a_controlled_sort_wins_over_the_default() {
    let html = render(|| {
        html! { <Table<Host> columns={columns()} rows={hosts()} sort={Sort::asc("host")} default_sort={Sort::desc("host")} /> }
    });
    assert_eq!(names(&html), ["db-1", "web-2", "web-10"]);
}

#[test]
fn next_sort_reverses_replaces_or_extends() {
    let host = AttrValue::from("host");
    let memory = AttrValue::from("memory");
    let current = [Sort::asc("host")];

    assert_eq!(next_sort(&[], &host, false), [Sort::asc("host")]);
    assert_eq!(next_sort(&current, &host, false), [Sort::desc("host")]);
    assert_eq!(next_sort(&current, &memory, false), [Sort::asc("memory")]);
    assert_eq!(next_sort(&current, &memory, true), [Sort::asc("host"), Sort::asc("memory")]);
    assert_eq!(next_sort(&[Sort::asc("host"), Sort::asc("memory")], &host, true), [Sort::desc("host"), Sort::asc("memory")]);
}

#[test]
fn natural_order_ignores_case_and_reads_numbers_by_value() {
    let cases = [
        ("web-2", "web-10", Ordering::Less),
        ("web-10", "web-9", Ordering::Greater),
        ("Edge", "db", Ordering::Greater),
        ("v1.02", "v1.2", Ordering::Less), // equal by value, so the spelling decides
        ("alpha", "alpha", Ordering::Equal),
        ("", "a", Ordering::Less),
        ("host-123456789012345678901234567890", "host-123456789012345678901234567891", Ordering::Less),
    ];
    for (a, b, expected) in cases {
        assert_eq!(natural_cmp(a, b), expected, "{a} vs {b}");
        assert_eq!(natural_cmp(b, a), expected.reverse(), "{b} vs {a}");
    }
}

#[test]
fn sort_order_is_stable_for_equal_rows() {
    let order = sort_order(&hosts(), &columns(), &[Sort::asc("memory")]);
    assert_eq!(order, [0, 2, 1]);
    let order = sort_order(&hosts(), &columns(), &[Sort::desc("memory")]);
    assert_eq!(order, [1, 0, 2]);
}

#[test]
fn selection_adds_named_checkboxes_reflecting_the_selected_keys() {
    let html = render(|| {
        let selected: Selection = Rc::new(["web-2".into()].into_iter().collect());
        let name = RowFn::new(|host: &Host| AttrValue::from(host.name));
        html! {
            <Table<Host> columns={columns()} rows={hosts()} row_key={name.clone()} row_label={name} {selected} />
        }
    });
    assert_eq!(html.matches(r#"type="checkbox""#).count(), 4);
    assert!(html.contains(r#"aria-label="Select all rows""#));
    for host in hosts().iter() {
        assert!(html.contains(&format!(r#"aria-label="Select {}""#, host.name)));
    }
    assert_eq!(html.matches(" checked").count(), 1);

    assert!(!table(vec![]).contains("checkbox"));
}

#[test]
fn details_are_collapsed_until_asked_for() {
    let html = render(|| {
        let detail = RowFn::new(|host: &Host| html! { <p>{ format!("Details of {}", host.name) }</p> });
        html! { <Table<Host> columns={columns()} rows={hosts()} {detail} /> }
    });
    assert_eq!(html.matches(r#"aria-expanded="false""#).count(), 3);
    assert!(!html.contains("Details of"));
}

#[test]
fn loading_and_empty_states() {
    let empty: Rc<[Host]> = Rc::from([]);

    let loading = {
        let rows = empty.clone();
        render(move || html! { <Table<Host> columns={columns()} rows={rows.clone()} loading=true skeleton_rows=2 /> })
    };
    assert!(loading.contains(r#"aria-busy="true""#));
    assert_eq!(loading.matches("<tr").count(), 3, "a header row and two placeholders");

    let nothing = {
        let rows = empty.clone();
        render(move || html! { <Table<Host> columns={columns()} rows={rows.clone()} /> })
    };
    assert!(nothing.contains("Nothing to show"));
    assert!(!nothing.contains("aria-busy"));

    let custom = render(move || {
        html! { <Table<Host> columns={columns()} rows={empty.clone()} empty={html! { "No hosts enrolled" }} /> }
    });
    assert!(custom.contains("No hosts enrolled"));

    let refreshing = render(|| html! { <Table<Host> columns={columns()} rows={hosts()} loading=true /> });
    assert!(refreshing.contains(r#"aria-busy="true""#));
    assert_eq!(names(&refreshing).len(), 3, "rows stay while a refresh runs");
}

#[test]
fn every_variant_is_distinguishable() {
    let rendered: Vec<String> = TableVariant::ALL
        .into_iter()
        .map(|variant| render(move || html! { <Table<Host> columns={columns()} rows={hosts()} {variant} /> }))
        .collect();
    for (i, a) in rendered.iter().enumerate() {
        for b in &rendered[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn id_class_attrs_and_label_reach_the_table() {
    let html = render(|| {
        html! {
            <Table<Host>
                id="hosts"
                class="mine"
                label="Hosts"
                attrs={attrs! { "data-testid" => "hosts-table" }}
                columns={columns()}
                rows={hosts()}
            />
        }
    });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="hosts""#));
    assert!(root.contains("mine"));
    assert!(root.contains(r#"data-testid="hosts-table""#));
    assert!(html.contains(r#"aria-label="Hosts""#));
}

#[test]
fn simple_table_frames_hand_written_markup() {
    let html = render(|| {
        html! {
            <SimpleTable label="Stages" compact=true max_height="10rem" footer={html! { "2 stages" }}>
                <thead><tr><SortHeader direction={SortDirection::Descending}>{ "Stage" }</SortHeader></tr></thead>
                <tbody><tr><td>{ "Canary" }</td></tr></tbody>
            </SimpleTable>
        }
    });
    assert!(html.contains(r#"<table"#) && html.contains("Canary") && html.contains("2 stages"));
    assert!(html.contains(r#"aria-sort="descending""#));
    assert!(html.contains(r#"data-yi-density="compact""#));
    assert!(html.contains("10rem"));
}
