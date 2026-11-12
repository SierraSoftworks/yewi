crate::page! {
    title: "Avatar",
    category: Basic,
    summary: "A person at a glance: their picture, or their initials on a colour that is always theirs. `AvatarGroup` stacks several people and `UserChip` adds the name.",
    examples: [
        basic => "Initials and colour": "With only a `name`, the avatar shows initials on a colour hashed from that name, so a person looks the same everywhere without storing anything. With no name it is a neutral placeholder.",
        sizes => "Sizes": "`Small` (24px) fits table rows and inline text, `Medium` (32px) is the default and `Large` (48px) suits headers. Set `--yi-avatar-size` for anything else, such as a profile page.",
        fill => "Soft and bold": "The soft tint is the default and sits quietly in dense views. `bold` gives a solid fill for the few places where people are the subject of the view.",
        image => "Pictures and fallback": "`src` shows a picture over the initials. If it fails to load the initials are simply what remains, so there is never a broken image.",
        group => "Groups": "`AvatarGroup` overlaps its avatars. `max` limits how many are shown and counts the rest; `total` counts people who were never passed in, such as the rest of a paged list.",
        user_chip => "User chip": "`UserChip` pairs the avatar with the name and an optional line of detail. `Plain` suits table cells and headers; `Soft` makes a person a discrete token.",
        interactive => "Chips as links and buttons": "With `href` the chip is a link, with `on_click` a button. Both take keyboard focus and need nothing else to be accessible.",
        hashed => "Libravatar and Gravatar": "With the `avatar-hash` feature, an `email` loads the person's Libravatar picture, then their Gravatar, then falls back to initials. Only a SHA-256 hash of the address is sent. `avatar_url` gives the same address for use elsewhere.",
    ],
}
