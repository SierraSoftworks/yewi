use yew::prelude::*;
use yewi::prelude::*;

/// What `cargo test --color always` wrote to the terminal.
const OUTPUT: &str = "\x1b[1m\x1b[32m   Compiling\x1b[0m yewi v0.1.0 (/src/yewi)\n\
\x1b[1m\x1b[33mwarning\x1b[0m\x1b[1m: unused variable: `rows`\x1b[0m\n\
\x20\x1b[1m\x1b[34m-->\x1b[0m yewi/src/components/table/mod.rs:42:9\n\
\x1b[1m\x1b[32m    Finished\x1b[0m `test` profile in 8.31s\n\
\x1b[1m\x1b[32m     Running\x1b[0m unittests src/lib.rs\n\
\n\
test table::sorts_by_column ... \x1b[32mok\x1b[0m\n\
test table::keeps_selection ... \x1b[31mFAILED\x1b[0m\n\
test table::virtual_rows ... \x1b[33mignored\x1b[0m\n\
\n\
test result: \x1b[31mFAILED\x1b[0m. 1 passed; 1 failed; 1 ignored; \x1b[2mfinished in 0.02s\x1b[0m\n";

/// The 16 palette colours, as text and as backgrounds.
const PALETTE: &str = "\n\
\x1b[30m 0 \x1b[31m 1 \x1b[32m 2 \x1b[33m 3 \x1b[34m 4 \x1b[35m 5 \x1b[36m 6 \x1b[37m 7 \x1b[0m\
\x20 \x1b[40m   \x1b[41m   \x1b[42m   \x1b[43m   \x1b[44m   \x1b[45m   \x1b[46m   \x1b[47m   \x1b[0m\n\
\x1b[90m 8 \x1b[91m 9 \x1b[92m 10\x1b[93m 11\x1b[94m 12\x1b[95m 13\x1b[96m 14\x1b[97m 15\x1b[0m\
\x20 \x1b[100m   \x1b[101m   \x1b[102m   \x1b[103m   \x1b[104m   \x1b[105m   \x1b[106m   \x1b[107m   \x1b[0m\n";

#[function_component]
pub fn Example() -> Html {
    html! {
        <Terminal title="cargo test">
            <TerminalCommand>{ "cargo test --color always" }</TerminalCommand>
            <TerminalOutput text={OUTPUT} />
            <TerminalCommand>{ "colortest" }</TerminalCommand>
            <TerminalOutput text={PALETTE} attrs={attrs! { "aria-hidden" => "true" }} />
        </Terminal>
    }
}
