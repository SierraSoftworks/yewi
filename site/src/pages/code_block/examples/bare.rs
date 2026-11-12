use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <p>{ "Install the command line tool, then sign in:" }</p>
            <CodeBlock code={"cargo install buckle-cli\nbuckle login --server https://buckle.example.com"} copy=false />
            <p>{ "With a language the header returns, and the code is coloured:" }</p>
            <CodeBlock code={"cargo install buckle-cli\nbuckle login --server https://buckle.example.com"} language="sh" />
        </Stack>
    }
}
