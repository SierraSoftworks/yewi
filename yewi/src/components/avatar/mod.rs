//! People: [`Avatar`], [`AvatarGroup`] and [`UserChip`].
//!
//! An avatar is a circle showing a person's picture, or their initials on a
//! colour derived from their name, so the same person always looks the same.

use yew::prelude::*;

use crate::{Attrs, Icon, Size, icons};

#[cfg(feature = "avatar-hash")]
mod hash;

/// Up to two initials for a name: the first letters of its first and last
/// words, in upper case. Empty when the name has no letters or digits.
pub fn initials(name: &str) -> String {
    let mut letters = name.split_whitespace().filter_map(|word| word.chars().find(|c| c.is_alphanumeric()));
    let first = letters.next();
    let last = letters.next_back();
    first.into_iter().chain(last).flat_map(char::to_uppercase).collect()
}

/// The hue, in degrees, an avatar uses for `seed`. The same seed always
/// gives the same hue, on the server and in the browser; case and
/// surrounding whitespace are ignored.
pub fn avatar_hue(seed: &str) -> u16 {
    // FNV-1a, finished with an avalanche step so similar names land far apart.
    let mut hash = seed
        .trim()
        .chars()
        .flat_map(char::to_lowercase)
        .fold(0x811c_9dc5_u32, |hash, c| (hash ^ c as u32).wrapping_mul(0x0100_0193));
    hash ^= hash >> 16;
    hash = hash.wrapping_mul(0x85eb_ca6b);
    hash ^= hash >> 13;
    (hash % 360) as u16
}

/// The picture for an email address: its Libravatar, falling back to its
/// Gravatar, and failing (so an [`Avatar`] shows initials) when it has
/// neither. `pixels` is the width and height to request.
///
/// Only a SHA-256 hash of the address is sent. Requires the `avatar-hash`
/// feature.
#[cfg(feature = "avatar-hash")]
pub fn avatar_url(email: &str, pixels: u16) -> String {
    let hash = hash::sha256_hex(email.trim().to_lowercase().as_bytes());
    format!(
        "https://seccdn.libravatar.org/avatar/{hash}?s={pixels}&d=https%3A%2F%2Fwww.gravatar.com%2Favatar%2F{hash}%3Fs%3D{pixels}%26d%3D404"
    )
}

/// The rendered width of an avatar in CSS pixels, for requesting pictures.
#[cfg(feature = "avatar-hash")]
const fn pixels(size: Size) -> u16 {
    match size {
        Size::Small => 24,
        Size::Medium => 32,
        Size::Large => 48,
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct AvatarProps {
    /// The person's name. It gives the initials, the accessible name and,
    /// unless `seed` or `email` is set, the colour.
    #[prop_or_default]
    pub name: AttrValue,
    /// A picture. The initials stay underneath and show while it loads or
    /// if it fails.
    #[prop_or_default]
    pub src: Option<AttrValue>,
    /// The person's email address. With the `avatar-hash` feature and no
    /// `src`, their Libravatar or Gravatar picture is loaded; only a hash of
    /// the address leaves the page. Without the feature it only seeds the colour.
    #[prop_or_default]
    pub email: Option<AttrValue>,
    /// What the colour is derived from, when a name is not stable or unique
    /// enough: a user id, for example. Defaults to `email`, then `name`.
    #[prop_or_default]
    pub seed: Option<AttrValue>,
    /// `Small` is 24px, `Medium` 32px and `Large` 48px. For any other size
    /// set `--yi-avatar-size`.
    #[prop_or_default]
    pub size: Size,
    /// A solid fill instead of the default soft tint.
    #[prop_or_default]
    pub bold: bool,
    /// Hides the avatar from assistive technology. Set it when the name is
    /// already shown next to the avatar.
    #[prop_or_default]
    pub decorative: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A person, as their picture or their initials on a colour of their own.
#[function_component]
pub fn Avatar(props: &AvatarProps) -> Html {
    let src = props.src.clone();
    #[cfg(feature = "avatar-hash")]
    let src = src.or_else(|| props.email.as_ref().map(|email| avatar_url(email, pixels(props.size) * 2).into()));

    // The picture that failed to load, if any. A new `src` gets a new chance.
    let failed = use_state(|| None::<AttrValue>);
    let image = use_node_ref();
    let src = src.filter(|src| failed.as_ref() != Some(src));

    // An error raised before hydration never reaches `onerror`, so look once.
    #[cfg(target_arch = "wasm32")]
    {
        let (image, failed) = (image.clone(), failed.clone());
        use_effect_with(src.clone(), move |src| {
            let broken = image.get().is_some_and(|node| {
                let read = |key: &str| js_sys::Reflect::get(&node, &key.into()).ok();
                read("complete").and_then(|v| v.as_bool()) == Some(true)
                    && read("naturalWidth").and_then(|v| v.as_f64()) == Some(0.0)
            });
            if broken {
                failed.set(src.clone());
            }
        });
    }

    let onerror = {
        let (failed, src) = (failed.clone(), src.clone());
        Callback::from(move |_: Event| failed.set(src.clone()))
    };

    let seed = props.seed.as_ref().or(props.email.as_ref()).unwrap_or(&props.name);
    let seeded = !seed.trim().is_empty();
    let initials = initials(&props.name);
    let named = !props.decorative && !props.name.is_empty();

    props.attrs.apply(html! {
        <span
            id={props.id.clone()}
            class={classes!(
                "yi-avatar",
                format!("yi-avatar--{}", props.size.as_str()),
                props.bold.then_some("yi-avatar--bold"),
                (!seeded).then_some("yi-avatar--anonymous"),
                props.class.clone(),
            )}
            style={seeded.then(|| format!("--yi-avatar-hue:{}", avatar_hue(seed)))}
            role={named.then_some("img")}
            aria-label={named.then(|| props.name.clone())}
            aria-hidden={(!named).then_some("true")}
            title={named.then(|| props.name.clone())}
        >
            if initials.is_empty() {
                <Icon icon={icons::USER} />
            } else {
                <span class="yi-avatar__initials" aria-hidden="true">{ initials }</span>
            }
            if let Some(src) = src {
                <img
                    ref={image}
                    class="yi-avatar__image"
                    {src}
                    alt=""
                    loading="lazy"
                    decoding="async"
                    referrerpolicy="no-referrer"
                    {onerror}
                />
            }
        </span>
    })
}

#[derive(Properties, PartialEq)]
pub struct AvatarGroupProps {
    /// The avatars, most relevant first.
    #[prop_or_default]
    pub children: ChildrenWithProps<Avatar>,
    /// The most avatars to show. The rest are counted in a "+N" marker.
    #[prop_or_default]
    pub max: Option<usize>,
    /// How many people there are, when only some were passed as children
    /// (a page from a server, for example). Defaults to the number of children.
    #[prop_or_default]
    pub total: Option<usize>,
    /// Sets the size of every avatar in the group.
    #[prop_or_default]
    pub size: Option<Size>,
    /// Accessible name for the group: "Reviewers", "Assigned to".
    #[prop_or_default]
    pub label: Option<AttrValue>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// Several people as an overlapping row, with a count of those not shown.
#[function_component]
pub fn AvatarGroup(props: &AvatarGroupProps) -> Html {
    let count = props.children.len();
    let shown = props.max.map_or(count, |max| max.min(count));
    let more = props.total.unwrap_or(count).saturating_sub(shown);

    // Name the people who were passed but are not shown.
    let hidden: Vec<AttrValue> =
        props.children.iter().skip(shown).map(|c| c.props.name.clone()).filter(|n| !n.is_empty()).collect();
    let unnamed = more.saturating_sub(hidden.len());
    let mut names = hidden.join(", ");
    if !names.is_empty() && unnamed > 0 {
        names = format!("{names} and {unnamed} more");
    }
    let size = props.size.unwrap_or_else(|| props.children.iter().next().map(|c| c.props.size).unwrap_or_default());

    props.attrs.apply(html! {
        <span
            id={props.id.clone()}
            class={classes!("yi-avatar-group", props.class.clone())}
            role="group"
            aria-label={props.label.clone()}
        >
            { for props.children.iter().take(shown).map(|mut child| {
                if let Some(size) = props.size.filter(|size| *size != child.props.size) {
                    std::rc::Rc::make_mut(&mut child.props).size = size;
                }
                child
            }) }
            if more > 0 {
                <span
                    class={classes!("yi-avatar", "yi-avatar--more", format!("yi-avatar--{}", size.as_str()))}
                    role="img"
                    aria-label={format!("{more} more")}
                    title={(!names.is_empty()).then_some(names)}
                >
                    { format!("+{more}") }
                </span>
            }
        </span>
    })
}

/// How a [`UserChip`] is framed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum UserChipVariant {
    /// No chrome: an avatar and a name, for table cells, lists and headers.
    #[default]
    Plain,
    /// A soft rounded fill, for people as discrete tokens: assignees,
    /// recipients, filters.
    Soft,
}

impl UserChipVariant {
    pub const ALL: [UserChipVariant; 2] = [UserChipVariant::Plain, UserChipVariant::Soft];

    const fn as_str(self) -> &'static str {
        match self {
            UserChipVariant::Plain => "plain",
            UserChipVariant::Soft => "soft",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct UserChipProps {
    /// The person's name.
    pub name: AttrValue,
    /// A second, quieter line: a role, a team or an email address.
    #[prop_or_default]
    pub detail: Option<AttrValue>,
    /// The avatar's picture. See [`Avatar`].
    #[prop_or_default]
    pub src: Option<AttrValue>,
    /// The person's email address, passed to the [`Avatar`]. It is not shown.
    #[prop_or_default]
    pub email: Option<AttrValue>,
    /// What the avatar's colour is derived from. See [`Avatar`].
    #[prop_or_default]
    pub seed: Option<AttrValue>,
    #[prop_or_default]
    pub variant: UserChipVariant,
    /// Sets the avatar size and the text size that goes with it.
    #[prop_or_default]
    pub size: Size,
    /// A solid avatar fill instead of the default soft tint.
    #[prop_or_default]
    pub bold: bool,
    /// Renders the chip as a link, to a profile for example.
    #[prop_or_default]
    pub href: Option<AttrValue>,
    /// Renders the chip as a button and fires on activation. Ignored with `href`.
    #[prop_or_default]
    pub on_click: Option<Callback<MouseEvent>>,
    /// Content after the name, such as a pill or a chevron.
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A person as an avatar with their name, and optionally a line of detail.
#[function_component]
pub fn UserChip(props: &UserChipProps) -> Html {
    let class = classes!(
        "yi-user-chip",
        format!("yi-user-chip--{}", props.variant.as_str()),
        format!("yi-user-chip--{}", props.size.as_str()),
        props.class.clone(),
    );

    let content = html! {
        <>
            <Avatar
                name={props.name.clone()}
                src={props.src.clone()}
                email={props.email.clone()}
                seed={props.seed.clone()}
                size={props.size}
                bold={props.bold}
                decorative=true
            />
            <span class="yi-user-chip__text">
                <span class="yi-user-chip__name">{ props.name.clone() }</span>
                if let Some(detail) = &props.detail {
                    <span class="yi-user-chip__detail">{ detail.clone() }</span>
                }
            </span>
            { props.children.clone() }
        </>
    };

    let id = props.id.clone();
    props.attrs.apply(match (&props.href, &props.on_click) {
        (Some(href), _) => html! { <a {id} {class} href={href.clone()}>{ content }</a> },
        (None, Some(on_click)) => html! {
            <button {id} {class} type="button" onclick={on_click.clone()}>{ content }</button>
        },
        (None, None) => html! { <span {id} {class}>{ content }</span> },
    })
}

#[cfg(test)]
mod tests;
