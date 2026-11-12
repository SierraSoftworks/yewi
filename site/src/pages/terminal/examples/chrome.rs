use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <Terminal title="Neutral buttons" controls={TerminalControls::Muted}>
                <TerminalCommand>{ "git status --short" }</TerminalCommand>
                <TerminalOutput>{ " M yewi/src/components/terminal/mod.rs\n" }</TerminalOutput>
            </Terminal>
            <Terminal title="No buttons" controls={TerminalControls::Hidden}>
                <TerminalCommand>{ "git status --short" }</TerminalCommand>
                <TerminalOutput>{ " M yewi/src/components/terminal/mod.rs\n" }</TerminalOutput>
            </Terminal>
            <Terminal label="Install command" controls={TerminalControls::Hidden} nowrap=true>
                <TerminalCommand>
                    { "curl --proto '=https' --tlsv1.2 --silent --show-error --fail --location https://example.com/install.sh --output install.sh" }
                </TerminalCommand>
            </Terminal>
        </Stack>
    }
}
