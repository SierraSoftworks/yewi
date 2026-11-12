use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Terminal title="root@edge-01">
            <TerminalCommand prompt="#">{ "systemctl restart caddy" }</TerminalCommand>
            <TerminalOutput tone={Tone::Danger}>
                { "Job for caddy.service failed because the control process exited with error code.\n" }
            </TerminalOutput>
            <TerminalCommand prompt="#">{ "caddy validate --config /etc/caddy/Caddyfile" }</TerminalCommand>
            <TerminalOutput tone={Tone::Warn}>{ "warning: Caddyfile input is not formatted\n" }</TerminalOutput>
            <TerminalOutput tone={Tone::Info}>{ "using adapter: caddyfile\n" }</TerminalOutput>
            <TerminalOutput tone={Tone::Ok}>{ "Valid configuration\n" }</TerminalOutput>
            <TerminalCommand prompt=">>>">{ "print(\"a REPL prompt\")" }</TerminalCommand>
            <TerminalOutput>{ "a REPL prompt\n" }</TerminalOutput>
        </Terminal>
    }
}
