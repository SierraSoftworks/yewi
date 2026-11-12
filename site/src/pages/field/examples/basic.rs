use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let email = use_state(|| AttrValue::from("ben@example"));
    let on_input = {
        let email = email.clone();
        Callback::from(move |value: String| email.set(value.into()))
    };

    let valid = email.split_once('@').is_some_and(|(name, host)| !name.is_empty() && host.contains('.'));
    let error = (!valid).then(|| html! { "Enter an address like name@example.com." });

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 22rem" }}>
            <Field label="Email" hint="Deployment notices are sent here." {error}>
                <TextInput kind={TextInputKind::Email} value={(*email).clone()} {on_input} />
            </Field>
            <span role="status">{ if valid { "Looks good" } else { "Not a valid address" } }</span>
        </Stack>
    }
}
