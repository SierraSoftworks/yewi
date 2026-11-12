use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let branch = use_state(|| AttrValue::from("main"));
    let on_change = {
        let branch = branch.clone();
        Callback::from(move |value: AttrValue| branch.set(value))
    };
    let options = use_memo((), |_| {
        ["main", "release/2.14", "release/2.13", "fix/login-redirect"]
            .into_iter()
            .map(SelectOption::plain)
            .collect::<SelectOptions>()
    });

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 18rem" }}>
            <Field label="Branch" hint="Pick a branch or type the name of a new one.">
                <Combobox {options} value={(*branch).clone()} {on_change} allow_custom=true />
            </Field>
            <span role="status">{ format!("Deploying from {}", *branch) }</span>
        </Stack>
    }
}
