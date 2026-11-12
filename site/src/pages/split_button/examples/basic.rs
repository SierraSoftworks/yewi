use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let done = use_state(|| "Nothing deployed yet");
    let act = |action: &'static str| {
        let done = done.clone();
        Callback::from(move |_| done.set(action))
    };

    html! {
        <Stack align={Align::Start}>
            <SplitButton
                label="Deploy"
                icon={icons::ROCKET}
                variant={ButtonVariant::Primary}
                menu_label="More ways to deploy"
                on_click={act("Deployed to production")}
            >
                <MenuItem on_select={act("Deployed to staging")}>{ "Deploy to staging" }</MenuItem>
                <MenuItem on_select={act("Deployed a canary")}>{ "Deploy a canary" }</MenuItem>
                <MenuItem on_select={act("Rolled back")}>{ "Roll back" }</MenuItem>
            </SplitButton>
            <span role="status">{ *done }</span>
        </Stack>
    }
}
