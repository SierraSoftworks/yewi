use yew::prelude::*;
use yewi::prelude::*;

pub fn view() -> Html {
    html! {
        <Terminal title="zsh" max_rows=8 follow=true>
            <TerminalCommand cursor=true>{ "cargo build" }</TerminalCommand>
            <TerminalOutput tone={Tone::Ok} text={"\x1b[1;32mFinished\x1b[0m in 2.1s\n"} />
        </Terminal>
    }
}
