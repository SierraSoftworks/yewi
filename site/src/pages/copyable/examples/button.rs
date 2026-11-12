use yew::prelude::*;
use yewi::prelude::*;

const COMMAND: &str = "nomad job run caddy.hcl";

#[function_component]
pub fn Example() -> Html {
    let copies = use_state(|| 0_u32);
    let on_copy = {
        let copies = copies.clone();
        Callback::from(move |_| copies.set(*copies + 1))
    };

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 26rem" }}>
            <Cluster justify={Justify::Between}>
                <span>{ "Deploy the job" }</span>
                <CopyButton value={COMMAND} label="Copy command" size={Size::Small} on_copy={on_copy.clone()} />
            </Cluster>
            <pre>{ COMMAND }</pre>
            <Cluster>
                <CopyButton value={COMMAND} variant={ButtonVariant::Default} {on_copy}>
                    { "Copy command" }
                </CopyButton>
                <CopyButton value={COMMAND} disabled=true />
            </Cluster>
            <span role="status">{ format!("Copied {} times", *copies) }</span>
        </Stack>
    }
}
