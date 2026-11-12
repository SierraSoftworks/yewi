use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <p>{ "Run " }<code>{ "nomad job run caddy.hcl" }</code>{ " to deploy." }</p>
            <pre>{ "fn main() -> Result<(), Error> {\n    if a != b && c >= d { run()?; }\n    Ok(())\n}" }</pre>
            <pre style="--yi-font-mono-ligatures: normal">{ "ligatures on:  -> != >= => ==" }</pre>
            <Stack gap={Space::None} class="yi-tabular" align={Align::End}>
                <span>{ "12,904" }</span>
                <span>{ "1,117" }</span>
                <span>{ "88" }</span>
            </Stack>
        </Stack>
    }
}
