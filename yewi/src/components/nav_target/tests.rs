use super::*;

#[test]
fn a_string_is_a_plain_href() {
    let target = NavTarget::from("/settings");
    assert_eq!(target.url(None, false), "/settings");
    assert!(!target.is_empty());
    assert!(NavTarget::default().is_empty());
}

#[test]
fn query_parameters_are_appended_and_encoded() {
    let cases = [
        ("/list", vec![("page", "3")], "/list?page=3"),
        ("/list?sort=name", vec![("page", "3")], "/list?sort=name&page=3"),
        ("/search", vec![("q", "a b&c"), ("lang", "en")], "/search?q=a%20b%26c&lang=en"),
        ("", vec![("page", "2")], "?page=2"),
    ];
    for (href, query, expected) in cases {
        assert_eq!(NavTarget::href(href).query(query).url(None, false), expected);
    }
}

#[test]
fn a_router_basename_only_applies_to_routes() {
    assert_eq!(NavTarget::href("/docs").url(Some("/app/"), false), "/docs");
}

#[test]
fn an_optional_url_converts_and_none_is_empty() {
    assert!(NavTarget::from(None::<&'static str>).is_empty());
    assert_eq!(NavTarget::from(Some("/docs")), NavTarget::href("/docs"));
}

#[test]
fn external_is_opt_in_and_part_of_equality() {
    let plain = NavTarget::href("https://example.com");
    assert!(!plain.is_external());
    assert!(plain.clone().external().is_external());
    assert_ne!(plain.clone().external(), plain);
}

#[cfg(feature = "router")]
mod router {
    use yew_router::Routable;

    use super::*;

    #[derive(Debug, Clone, PartialEq, Routable)]
    enum Route {
        #[at("/users/:name")]
        User { name: String },
    }

    #[test]
    fn a_route_renders_its_path_under_the_basename() {
        let target = NavTarget::route(Route::User { name: "ada".into() });
        assert_eq!(target.url(None, false), "/users/ada");
        assert_eq!(target.url(Some("/app/"), false), "/app/users/ada");
        assert_eq!(target.query([("tab", "keys")]).url(None, true), "#/users/ada?tab=keys");
    }

    #[test]
    fn a_route_differs_from_a_plain_link_to_the_same_path() {
        assert_ne!(NavTarget::route(Route::User { name: "ada".into() }), NavTarget::href("/users/ada"));
    }
}
