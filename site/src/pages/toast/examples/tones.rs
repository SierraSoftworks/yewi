use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 24rem" }}>
            <Toast tone={Tone::Ok}>{ "Settings saved." }</Toast>
            <Toast tone={Tone::Info}>{ "A new version is available." }</Toast>
            <Toast tone={Tone::Warn} title="Saved with warnings">
                { "Two hosts were skipped because they are offline." }
            </Toast>
            <Toast
                tone={Tone::Danger}
                title="Publishing failed"
                action={ToastAction::new("Retry", Callback::noop())}
                on_dismiss={Callback::noop()}
            >
                { "The registry refused the upload." }
            </Toast>
            <Toast tone={Tone::Active}>{ "Rolling out to 12 hosts." }</Toast>
            <Toast>{ "Copied to the clipboard." }</Toast>
        </Stack>
    }
}
