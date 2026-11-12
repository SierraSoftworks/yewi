use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let check = html! { <Button>{ "Check" }</Button> };
    let check_disabled = html! { <Button disabled=true>{ "Check" }</Button> };

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 22rem" }}>
            <Field label="Default">
                <InputGroup prefix="https://" suffix={check.clone()}>
                    <TextInput value="example.com" />
                </InputGroup>
            </Field>
            <Field label="Disabled" disabled=true>
                <InputGroup prefix="https://" suffix={check_disabled}>
                    <TextInput value="example.com" />
                </InputGroup>
            </Field>
            <Field label="Read-only">
                <InputGroup prefix="https://">
                    <TextInput value="example.com" readonly=true />
                </InputGroup>
            </Field>
            <Field label="Invalid" error="This domain is already taken.">
                <InputGroup prefix="https://" suffix={check}>
                    <TextInput value="example.com" />
                </InputGroup>
            </Field>
            <Field label="Disabled search" disabled=true>
                <SearchInput value="checkout" />
            </Field>
        </Stack>
    }
}
