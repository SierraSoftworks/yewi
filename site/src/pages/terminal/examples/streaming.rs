use yew::prelude::*;
use yewi::prelude::*;

const STEPS: [&str; 6] = ["fetch", "build", "test", "package", "upload", "release"];

#[function_component]
pub fn Example() -> Html {
    let lines = use_state(|| 4_usize);
    let on_click = {
        let lines = lines.clone();
        Callback::from(move |_| lines.set(*lines + 3))
    };

    let output: String = (1..=*lines)
        .map(|n| format!("[{n:>3}] \x1b[36m{:<8}\x1b[0m step finished \x1b[32mok\x1b[0m\n", STEPS[(n - 1) % STEPS.len()]))
        .collect();

    html! {
        <Stack>
            <Terminal title="deploy" max_rows=6 follow=true>
                <TerminalCommand>{ "nomad job run -detach=false site.hcl" }</TerminalCommand>
                <TerminalOutput text={output} />
            </Terminal>
            <Cluster>
                <Button {on_click}>{ "Print more output" }</Button>
                <span role="status">{ format!("{} lines printed", *lines) }</span>
            </Cluster>
        </Stack>
    }
}
