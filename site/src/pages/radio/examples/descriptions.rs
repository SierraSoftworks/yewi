use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let role = use_state(|| AttrValue::from("viewer"));
    let on_change = {
        let role = role.clone();
        Callback::from(move |value| role.set(value))
    };

    html! {
        <Stack>
            <Field label="Role" group=true>
                <RadioGroup value={(*role).clone()} {on_change}>
                    <Radio value="viewer" description="Can see deploys and logs.">{ "Viewer" }</Radio>
                    <Radio value="operator" description="Can start, pause and roll back deploys.">
                        { "Operator" }
                    </Radio>
                    <Radio value="owner" description="Only an existing owner can grant this." disabled=true>
                        { "Owner" }
                    </Radio>
                </RadioGroup>
            </Field>
            <span role="status">{ format!("Role: {}", *role) }</span>
        </Stack>
    }
}
