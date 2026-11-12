crate::page! {
    title: "Empty and error states",
    category: Basic,
    summary: "What a view says when it has nothing to show: EmptyState when that is expected, ErrorState when loading failed.",
    examples: [
        basic => "Nothing here yet": "Say what is missing, whether that is a problem, and offer the next step. An empty state that only says \"nothing here\" leaves the reader guessing.",
        tones => "Good news and no results": "`tone` colours the icon and carries meaning: `Ok` when empty is the desired outcome, neutral when a search or filter matched nothing.",
        alignment => "Alignment": "`Center` suits a view or card with nothing else in it. `Start` lines up with the surrounding content, for a section that sits among others.",
        sizes => "Sizes": "`Small` fits a table body or a narrow card, `Medium` a panel, and `Large` a whole page. The icon is optional at every size.",
        error => "Failed to load": "`ErrorState` is announced as an alert. Pass `on_retry` to offer a retry and `retrying` while it runs.",
        error_no_retry => "When retrying cannot help": "Leave `on_retry` unset when trying again cannot change the outcome, such as a refused permission, and offer a way forward in `actions` instead.",
    ],
}
