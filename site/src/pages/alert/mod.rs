crate::page! {
    title: "Alert, Note, Banner",
    category: Basic,
    summary: "Alerts, banners and notes tell people about the state of a page. An alert sits in the flow of the page, a banner runs across the top, and a note sits beside the thing it explains.",
    examples: [
        tones => "Alert tones": "The tone sets the colour, the icon and how urgent the message is. `Warn` and `Danger` interrupt a screen reader, while the other tones wait their turn. Add a title when the message needs a headline, and leave it out when the message is a single sentence.",
        actions => "Actions and dismissal": "Offer the way out right next to the problem. When you set `on_dismiss`, the alert shows a dismiss button. Leave it unset for anything the page still depends on being true.",
        rich => "Rich content": "The message can hold a list of advice or links. Use `icon` to replace the tone's icon, or `hide_icon` to remove it.",
        note => "Note": "A note is a quiet line of supporting text. `Neutral` is muted, any other tone colours it, and `with_icon` adds the tone's icon. Give it `role=\"status\"` when it reports the result of an action.",
        banner => "Banner": "A banner is a full-width strip for a message about the whole page or app. It takes a sentence, an action at the trailing end and an optional dismiss button.",
        banner_solid => "Solid banner": "`Solid` fills the banner with the tone. Save it for the one status a page exists to report. `Large` turns the banner into a headline and `Small` makes it a slim strip.",
    ],
}
