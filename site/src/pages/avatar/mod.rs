crate::page! {
    title: "Avatar",
    category: Basic,
    summary: "Avatars show who someone is at a glance, using their picture or their initials on a colour that's always theirs. `AvatarGroup` stacks several people together and `UserChip` adds the person's name.",
    examples: [
        basic => "Initials and colour": "When you only give a `name`, the avatar shows initials on a colour hashed from that name. That means a person looks the same everywhere without you storing anything. With no name, the avatar is a neutral placeholder.",
        sizes => "Sizes": "`Small` (24px) fits table rows and inline text, `Medium` (32px) is the default, and `Large` (48px) suits headers. For any other size, such as on a profile page, set `--yi-avatar-size`.",
        fill => "Soft and bold": "The soft tint is the default, and it sits quietly in dense views. Use `bold` for a solid fill in the few places where people are the subject of the view.",
        image => "Pictures and fallback": "`src` shows a picture over the initials. If the picture fails to load, the initials are simply what's left, so you never see a broken image.",
        group => "Groups": "`AvatarGroup` overlaps its avatars. `max` limits how many are shown and counts the rest. `total` also counts people you never passed in, such as the rest of a paged list.",
        user_chip => "User chip": "`UserChip` pairs the avatar with the name and an optional line of detail. `Plain` suits table cells and headers, while `Soft` sets the person apart as a self-contained token.",
        interactive => "Chips as links and buttons": "With `href` the chip becomes a link, and with `on_click` it becomes a button. Both take keyboard focus and are accessible without anything extra.",
        hashed => "Libravatar and Gravatar": "With the `avatar-hash` feature, giving an `email` loads the person's Libravatar picture, then their Gravatar, and falls back to initials if neither is found. Only a SHA-256 hash of the address is sent. If you need the same picture address elsewhere, `avatar_url` gives it to you.",
    ],
}
