use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack gap={Space::Lg} attrs={attrs! { "style" => "max-inline-size: 28rem" }}>
            <Button variant={ButtonVariant::Primary}>{ "Sign in with a passkey" }</Button>
            <Divider>{ "or" }</Divider>
            <Button>{ "Sign in with single sign-on" }</Button>
            <Divider align={DividerAlign::Start}>{ "Yesterday" }</Divider>
            <span>{ "edge-01 was enrolled." }</span>
            <Divider align={DividerAlign::End}>{ "End of log" }</Divider>
        </Stack>
    }
}
