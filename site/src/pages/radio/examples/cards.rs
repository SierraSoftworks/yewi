use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let platform = use_state(|| None::<AttrValue>);
    let on_change = {
        let platform = platform.clone();
        Callback::from(move |value| platform.set(Some(value)))
    };

    html! {
        <Stack>
            <Field label="Platform" group=true>
                <RadioGroup value={(*platform).clone()} {on_change} orientation={Orientation::Horizontal}>
                    <RadioCard value="linux" description="A systemd service, installed by one command.">
                        { "Linux" }
                    </RadioCard>
                    <RadioCard value="docker" description="A container beside your other workloads.">
                        { "Docker" }
                    </RadioCard>
                    <RadioCard value="nomad" description="A system job on every client in the cluster.">
                        { "Nomad" }
                    </RadioCard>
                </RadioGroup>
            </Field>
            <span role="status">
                { match &*platform {
                    Some(platform) => format!("Platform: {platform}"),
                    None => "Nothing chosen yet".to_string(),
                } }
            </span>
        </Stack>
    }
}
