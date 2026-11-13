crate::page! {
    title: "Empty and error states",
    category: Basic,
    summary: "Empty states explain why a view has nothing to show. Use EmptyState when that's expected, and ErrorState when loading failed.",
    examples: [
        basic => "Nothing here yet": "Say what's missing, whether that's a problem, and what the reader can do next. An empty state that only says \"nothing here\" leaves the reader guessing.",
        tones => "Good news and no results": "`tone` colours the icon and carries meaning. Use `Ok` when an empty view is the outcome you wanted, and leave it neutral when a search or filter matched nothing.",
        alignment => "Alignment": "`Center` suits a view or card that has nothing else in it. `Start` lines up with the surrounding content, which suits a section that sits among others.",
        sizes => "Sizes": "`Small` fits a table body or a narrow card, `Medium` fits a panel, and `Large` fits a whole page. The icon is optional at every size.",
        error => "Failed to load": "`ErrorState` is announced as an alert. Pass `on_retry` to offer a retry, and set `retrying` while the retry runs.",
        error_no_retry => "When retrying cannot help": "When trying again can't change the outcome, such as when a permission was refused, leave `on_retry` unset and offer a way forward in `actions` instead.",
    ],
}
