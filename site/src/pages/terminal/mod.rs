crate::page! {
    title: "Terminal",
    category: Data,
    summary: "Shows a shell session or command output in a terminal window that stays dark in both themes.",
    feature: "terminal",
    examples: [
        session => "A session": "`TerminalCommand` is a line the user typed and `TerminalOutput` is what came back. The prompt is not selectable, so copying a line yields a command that can be pasted and run.",
        ansi => "Program output": "Pass raw output as `text` and its ANSI colours and styles are drawn from the terminal palette. Cursor movement, titles and other escape sequences are removed.",
        tones => "Tones and prompts": "Set `tone` to colour output you write yourself by meaning. `prompt` changes the prompt per line, for a root shell, a REPL or another host.",
        streaming => "Following live output": "`max_rows` caps the height and makes the body a keyboard-scrollable region. With `follow` the newest line stays in view until the reader scrolls up, and following resumes when they return to the end.",
        chrome => "Window chrome": "`controls` switches the window buttons to neutral dots or removes them, and the title is optional. With neither there is no title bar. `nowrap` scrolls long lines sideways instead of wrapping them.",
    ],
}
