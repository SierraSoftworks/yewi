crate::page! {
    title: "Terminal",
    category: Data,
    summary: "Terminal shows a shell session or command output in a terminal window, which stays dark in both themes.",
    feature: "terminal",
    examples: [
        session => "A session": "`TerminalCommand` is a line the user typed, and `TerminalOutput` is what came back. The prompt can't be selected, so copying a line gives you a command you can paste and run.",
        ansi => "Program output": "Pass raw output as `text`, and its ANSI colours and styles are drawn from the terminal palette. Cursor movement, titles and other escape sequences are stripped out.",
        tones => "Tones and prompts": "Set `tone` to colour output you write yourself according to what it means. `prompt` changes the prompt for each line, for example for a root shell, a REPL or another host.",
        streaming => "Following live output": "`max_rows` caps the height and turns the body into a region you can scroll with the keyboard. With `follow`, the newest line stays in view until the reader scrolls up, and following picks up again when they scroll back to the end.",
        chrome => "Window chrome": "`controls` switches the window buttons to neutral dots or removes them, and the title is optional. If you leave out both, there's no title bar at all. `nowrap` scrolls long lines sideways instead of wrapping them.",
    ],
}
