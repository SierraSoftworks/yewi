use yew::prelude::*;
use yewi::prelude::*;

const KEY: &str = "sk_live_51NqT8vX2mZ7cL1sT8bN5wK3h";

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 22rem" }}>
            <Field label="Read-only" hint="Managed by your organisation.">
                <SecretInput value={KEY} readonly=true generator=true />
            </Field>
            <Field label="Disabled" disabled=true>
                <SecretInput value={KEY} generator=true />
            </Field>
            <Field label="Invalid" error={html! { "This key has been revoked." }}>
                <SecretInput value={KEY} />
            </Field>
            <Field label="Write-only" hint="Cannot be shown once typed.">
                <SecretInput revealable=false placeholder="Enter a new signing key" />
            </Field>
            <Field label="Password">
                <SecretInput autocomplete="current-password" name="password" reveal_label="Show password" />
            </Field>
        </Stack>
    }
}
