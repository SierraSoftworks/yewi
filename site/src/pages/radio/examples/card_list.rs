use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let method = use_state(|| AttrValue::from("tailscale"));
    let on_change = {
        let method = method.clone();
        Callback::from(move |value| method.set(value))
    };

    html! {
        <Stack>
            <Field label="Connect" group=true hint="You can change this later by joining the host again.">
                <RadioGroup value={(*method).clone()} {on_change}>
                    <RadioCard value="tailscale" description="Joins the tailnet with a single-use, tagged key.">
                        { "Over Tailscale" }
                    </RadioCard>
                    <RadioCard value="token" description="For a host that reaches the server directly.">
                        { "With a join token" }
                    </RadioCard>
                    <RadioCard value="ssh" description="Needs the SSH gateway, which is not set up." disabled=true>
                        { "Over SSH" }
                    </RadioCard>
                </RadioGroup>
            </Field>
            <span role="status">{ format!("Method: {}", *method) }</span>
        </Stack>
    }
}
