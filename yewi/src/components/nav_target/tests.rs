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
