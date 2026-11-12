crate::page! {
    title: "Alert, Note, Banner",
    category: Basic,
    summary: "Messages about the state of a page: an alert in the flow, a banner across the top, a note beside the thing it explains.",
    examples: [
        tones => "Alert tones": "The tone sets the colour, the icon and the urgency: `Warn` and `Danger` interrupt a screen reader, the rest wait their turn. Use a title when the message needs a headline, and leave it out for a single sentence.",
        actions => "Actions and dismissal": "Offer the way out next to the problem. An alert with `on_dismiss` shows a dismiss button; leave it unset for anything the page still depends on being true.",
        rich => "Rich content": "The message can hold a list of advice or links. `icon` replaces the tone's icon and `hide_icon` removes it.",
        note => "Note": "A quiet line of supporting text. `Neutral` is muted; a tone colours it, and `with_icon` adds the tone's icon. Give it `role=\"status\"` when it reports the result of an action.",
        banner => "Banner": "A full-width strip for a message about the whole page or app. It takes a sentence, an action at the trailing end and an optional dismiss button.",
        banner_solid => "Solid banner": "`Solid` fills the banner with the tone, for the one status a page exists to report. `Large` turns it into a headline and `Small` into a slim strip.",
    ],
}
