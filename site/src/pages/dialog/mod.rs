crate::page! {
    title: "Dialog",
    category: Overlay,
    summary: "Dialogs show a modal task or question and hold the keyboard until it's answered. Use them sparingly, because a dialog interrupts the user.",
    examples: [
        basic => "A dialog": "Your component holds `open` and clears it in `on_close`, which fires for `Escape`, the backdrop and the close button. Focus moves into the dialog when it opens and returns to the trigger when it closes.",
        sizes => "Sizes": "`Small` suits a short question, `Medium` is the default, and `Large` gives room for columns or a table. Every size shrinks to fit a narrow screen.",
        long => "Long content": "The body scrolls while the actions stay in view. With `dismissible` off there's no close button, and `Escape` and the backdrop do nothing, so the user has to choose.",
        confirm => "Confirming an action": "`ConfirmDialog` says what's about to happen and labels its button with the action, never \"Yes\". Focus starts on the way out, so a stray `Enter` doesn't destroy anything.",
        steps => "Requiring a deliberate step": "For actions that can't be undone, `acknowledge` asks the user to tick a statement and `type_to_confirm` asks them to type a word. The action stays disabled until both are done, and the dialog starts from scratch each time it opens. A press on the word copies it, and `fields` takes further inputs after those two rows. While a dialog is open, Tab also passes through the browser's own controls once per cycle; that is how native modal dialogs behave, not a leak of focus into the page.",
        failure => "When the action fails": "When the action fails, keep the dialog open, pass the reason as `error` and let the user try again. Set `busy` while the request runs, which shows progress and stops the dialog from being dismissed.",
    ],
}
