use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let email = use_state(AttrValue::default);
    let invited = use_state(|| None::<AttrValue>);
    let token = use_state(|| 1_u32);

    let set_email = {
        let email = email.clone();
        Callback::from(move |value: String| email.set(value.into()))
    };
    let onsubmit = {
        let (email, invited) = (email.clone(), invited.clone());
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            if !email.is_empty() {
                invited.set(Some((*email).clone()));
                email.set(AttrValue::default());
            }
        })
    };
    let regenerate = {
        let token = token.clone();
        Callback::from(move |_| token.set(*token + 1))
    };

    let invite = html! {
        <Button kind={ButtonKind::Submit} variant={ButtonVariant::Primary}>{ "Invite" }</Button>
    };
    let rotate = html! { <Button icon={icons::REFRESH_CW} on_click={regenerate}>{ "Regenerate" }</Button> };

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 24rem" }}>
            <form {onsubmit}>
                <Field label="Invite a teammate">
                    <InputGroup suffix={invite}>
                        <TextInput
                            kind={TextInputKind::Email}
                            value={(*email).clone()}
                            on_input={set_email}
                            placeholder="name@example.com"
                        />
                    </InputGroup>
                </Field>
            </form>
            <Field label="Deploy token">
                <InputGroup suffix={rotate}>
                    <TextInput value={format!("yi_live_{:04}", *token * 4217 % 10000)} readonly=true monospace=true />
                </InputGroup>
            </Field>
            <span role="status">
                { match &*invited {
                    Some(email) => format!("Invited {email}"),
                    None => "No invitations sent".to_owned(),
                } }
            </span>
        </Stack>
    }
}
