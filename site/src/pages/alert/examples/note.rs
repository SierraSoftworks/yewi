use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack gap={Space::Lg}>
            <div class="demo-card" style="max-inline-size: 26rem">
                <Stack gap={Space::Sm}>
                    <b class="demo-title">{ "Lockout policy" }</b>
                    <span>{ "Accounts lock for 15 minutes after 5 failed sign-ins." }</span>
                    <Note>{ "Passkey sign-ins never count towards a lockout." }</Note>
                </Stack>
            </div>
            <Stack gap={Space::Xs}>
                <Note tone={Tone::Ok}>{ "Saved. The service picks the change up on its next poll." }</Note>
                <Note tone={Tone::Warn}>{ "This certificate expires in 6 days." }</Note>
                <Note tone={Tone::Danger}>{ "The package could not be read." }</Note>
                <Note tone={Tone::Info} hide_icon=true>{ "Changes apply to new sessions only." }</Note>
                <Note icon={icons::LOCK}>{ "Only administrators can change this setting." }</Note>
            </Stack>
        </Stack>
    }
}
