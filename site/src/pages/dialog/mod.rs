crate::page! {
    title: "Dialog",
    category: Overlay,
    summary: "A modal task or question that holds the keyboard until it is answered. Use it sparingly: a dialog interrupts.",
    examples: [
        basic => "A dialog": "The owner holds `open` and clears it in `on_close`, which fires for `Escape`, the backdrop and the close button. Focus moves into the dialog and returns to the trigger when it closes.",
        sizes => "Sizes": "`Small` suits a short question, `Medium` is the default and `Large` gives room for columns or a table. Every size shrinks to fit a narrow screen.",
        long => "Long content": "The body scrolls and the actions stay in view. With `dismissible` off there is no close button and `Escape` and the backdrop do nothing, so the user must choose.",
        confirm => "Confirming an action": "`ConfirmDialog` names what is about to happen and labels its button with the action, never \"Yes\". Focus starts on the way out, so a stray `Enter` destroys nothing.",
        steps => "Requiring a deliberate step": "For the irreversible, `acknowledge` asks for a ticked statement and `type_to_confirm` for a typed word. The action stays disabled until both are done, and each opening starts from scratch.",
        failure => "When the action fails": "Keep the dialog open, pass the reason as `error` and let the user try again. Set `busy` while the request runs to show progress and stop the dialog being dismissed.",
    ],
}
