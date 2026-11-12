use yewi_test::render;

use super::*;
use crate::attrs;

/// The opening tag of the root element.
fn root(html: &str) -> &str {
    &html[..html.find('>').unwrap()]
}

#[test]
fn initials_come_from_the_first_and_last_words() {
    for (name, expected) in [
        ("Ada Lovelace", "AL"),
        ("ada", "A"),
        ("Jean-Luc Picard", "JP"),
        ("Mary Jane van der Berg", "MB"),
        ("  Grace   Hopper  ", "GH"),
        ("Émile Zola", "ÉZ"),
        ("(deploy bot)", "DB"),
        ("", ""),
        ("---", ""),
    ] {
        assert_eq!(initials(name), expected, "{name:?}");
    }
}

#[test]
fn the_hue_is_stable_and_ignores_case_and_padding() {
    assert_eq!(avatar_hue("Ada Lovelace"), avatar_hue("  ada lovelace "));
    assert!(avatar_hue("Ada Lovelace") < 360);
}

#[test]
fn different_people_get_different_hues() {
    let names = ["Ada Lovelace", "Grace Hopper", "Alan Turing", "Edsger Dijkstra", "Barbara Liskov", "Ada Lovelacf"];
    let mut hues: Vec<u16> = names.into_iter().map(avatar_hue).collect();
    hues.sort_unstable();
    hues.dedup();
    assert_eq!(hues.len(), names.len());
}

#[test]
fn shows_initials_and_is_named_after_the_person() {
    let html = render(|| html! { <Avatar name="Ada Lovelace" /> });
    assert!(html.contains(">AL<"));
    assert!(root(&html).contains(r#"role="img""#));
    assert!(root(&html).contains(r#"aria-label="Ada Lovelace""#));
}

#[test]
fn a_picture_keeps_the_initials_underneath_as_the_fallback() {
    let html = render(|| html! { <Avatar name="Ada Lovelace" src="/ada.png" /> });
    assert!(html.contains(r#"src="/ada.png""#));
    assert!(html.contains(r#"alt="""#));
    assert!(html.contains(">AL<"));
}

#[test]
fn without_a_name_it_is_a_hidden_placeholder() {
    let html = render(|| html! { <Avatar /> });
    assert!(root(&html).contains(r#"aria-hidden="true""#));
    assert!(!root(&html).contains("role="));
    assert!(html.contains("<svg"));
}

#[test]
fn decorative_hides_it_from_assistive_technology() {
    let html = render(|| html! { <Avatar name="Ada Lovelace" decorative=true /> });
    assert!(root(&html).contains(r#"aria-hidden="true""#));
    assert!(!root(&html).contains("aria-label"));
}

#[test]
fn the_colour_follows_seed_then_email_then_name() {
    let style = |html: String| root(&html).split("style=\"").nth(1).unwrap().split('"').next().unwrap().to_owned();
    let by_name = style(render(|| html! { <Avatar name="Ada" /> }));
    let by_email = style(render(|| html! { <Avatar name="Ada" email="ada@example.com" /> }));
    let by_seed = style(render(|| html! { <Avatar name="Ada" email="ada@example.com" seed="user-1" /> }));
    let renamed = style(render(|| html! { <Avatar name="Countess of Lovelace" email="ada@example.com" seed="user-1" /> }));

    assert_ne!(by_name, by_email);
    assert_ne!(by_email, by_seed);
    assert_eq!(by_seed, renamed);
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let avatar = render(|| html! { <Avatar name="Ada" id="a" class="mine" attrs={attrs! { "data-user" => "1" }} /> });
    let group = render(|| html! { <AvatarGroup id="a" class="mine" attrs={attrs! { "data-user" => "1" }} /> });
    let chip = render(|| html! { <UserChip name="Ada" id="a" class="mine" attrs={attrs! { "data-user" => "1" }} /> });
    for html in [avatar, group, chip] {
        let root = root(&html);
        assert!(root.contains(r#"id="a""#), "{root}");
        assert!(root.contains("mine"), "{root}");
        assert!(root.contains(r#"data-user="1""#), "{root}");
    }
}

fn group(max: Option<usize>, total: Option<usize>) -> String {
    render(move || {
        html! {
            <AvatarGroup label="Reviewers" {max} {total}>
                <Avatar name="Ada Lovelace" />
                <Avatar name="Grace Hopper" />
                <Avatar name="Alan Turing" />
                <Avatar name="Barbara Liskov" />
            </AvatarGroup>
        }
    })
}

#[test]
fn a_group_is_labelled_and_shows_everyone_by_default() {
    let html = group(None, None);
    assert!(root(&html).contains(r#"role="group""#));
    assert!(root(&html).contains(r#"aria-label="Reviewers""#));
    assert_eq!(html.matches(r#"role="img""#).count(), 4);
    assert!(!html.contains("more"));
}

#[test]
fn a_group_counts_the_people_it_does_not_show() {
    for (max, total, shown, more) in [
        (Some(2), None, 2, Some(2)),
        (Some(4), None, 4, None),
        (Some(9), None, 4, None),
        (None, Some(12), 4, Some(8)),
        (Some(3), Some(12), 3, Some(9)),
        (Some(0), None, 0, Some(4)),
    ] {
        let html = group(max, total);
        let marker = more.map(|n| format!(r#"aria-label="{n} more""#));
        assert_eq!(html.matches("yi-avatar__initials").count(), shown, "max {max:?} total {total:?}");
        match marker {
            Some(marker) => assert!(html.contains(&marker), "max {max:?} total {total:?}: {html}"),
            None => assert!(!html.contains(" more\""), "max {max:?} total {total:?}"),
        }
    }
}

#[test]
fn the_marker_names_the_hidden_people_it_knows() {
    let html = group(Some(2), Some(6));
    assert!(html.contains(r#"title="Alan Turing, Barbara Liskov and 2 more""#));
}

#[test]
fn a_chip_shows_the_name_once_to_assistive_technology() {
    let html = render(|| html! { <UserChip name="Ada Lovelace" detail="Analyst" /> });
    assert!(html.starts_with("<span"));
    assert!(html.contains("Ada Lovelace"));
    assert!(html.contains("Analyst"));
    assert!(!html.contains("aria-label"));
}

#[test]
fn a_chip_is_a_link_with_href_and_a_button_with_on_click() {
    let link = render(|| html! { <UserChip name="Ada" href="/people/ada" on_click={Callback::noop()} /> });
    assert!(link.starts_with("<a"));
    assert!(link.contains(r#"href="/people/ada""#));

    let button = render(|| html! { <UserChip name="Ada" on_click={Callback::noop()} /> });
    assert!(button.starts_with("<button"));
    assert!(button.contains(r#"type="button""#));
}

#[cfg(feature = "avatar-hash")]
mod hashing {
    use super::*;

    #[test]
    fn sha256_matches_the_published_vectors() {
        for (input, digest) in [
            ("", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
            ("abc", "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"),
            (
                "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
                "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1",
            ),
        ] {
            assert_eq!(hash::sha256_hex(input.as_bytes()), digest, "{input:?}");
        }
        // Long enough to need a second block for the padding alone.
        assert_eq!(
            hash::sha256_hex(&[b'a'; 64]),
            "ffe054fe7ae0cb6dc65c3af9b61d5209f439851db43d0ba5997337df154668eb"
        );
    }

    #[test]
    fn the_url_tries_libravatar_then_gravatar_then_fails() {
        let url = avatar_url("  Ada@Example.com ", 64);
        let hash = hash::sha256_hex(b"ada@example.com");
        assert!(url.starts_with(&format!("https://seccdn.libravatar.org/avatar/{hash}?")));
        assert!(url.contains("s=64"));
        assert!(url.contains(&format!("gravatar.com%2Favatar%2F{hash}")));
        assert!(url.ends_with("d%3D404"));
    }

    #[test]
    fn an_email_loads_the_hashed_picture_unless_src_is_given() {
        let hashed = render(|| html! { <Avatar name="Ada" email="ada@example.com" /> });
        assert!(hashed.contains("libravatar.org/avatar/"));
        assert!(!hashed.contains("ada@example.com"));

        let explicit = render(|| html! { <Avatar name="Ada" email="ada@example.com" src="/ada.png" /> });
        assert!(explicit.contains(r#"src="/ada.png""#));
        assert!(!explicit.contains("libravatar"));
    }
}

#[cfg(not(feature = "avatar-hash"))]
#[test]
fn without_the_feature_an_email_loads_nothing() {
    let html = render(|| html! { <Avatar name="Ada" email="ada@example.com" /> });
    assert!(!html.contains("<img"));
}
