use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Terminal title="ben@workstation: ~/src/yewi">
            <TerminalCommand>{ "cargo add yewi --features terminal" }</TerminalCommand>
            <TerminalOutput>
                { "    Updating crates.io index\n      Adding yewi v0.1.0 to dependencies\n" }
            </TerminalOutput>
            <TerminalCommand>{ "trunk build --release" }</TerminalCommand>
            <TerminalOutput>{ "    Finished release profile [optimized] target(s) in 41.07s\n" }</TerminalOutput>
            <TerminalCommand cursor=true />
        </Terminal>
    }
}
