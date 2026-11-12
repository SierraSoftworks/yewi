use yew::prelude::*;
use yewi::prelude::*;

const TOKEN: &str = "rtk_live_9Zp4mQ2vX7cL1sT8bN5wK3hJ6dF0gR4yU2aE7iO1pM8nB5vC3xZ6lK9jH";
const WEBHOOK: &str = "https://automate.example.com/api/v1/webhooks/deploy-caddy/3f1c9a7e52b84d06";
const ADDRESS: &str = "Sierra Softworks, 12 Harbour Road, Cape Town, 8001";

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 26rem" }}>
            <Copyable variant={CopyableVariant::Field} value={TOKEN} label="Copy token" />
            <Copyable variant={CopyableVariant::Field} value={WEBHOOK} label="Copy address" truncate=true />
            <Copyable variant={CopyableVariant::Field} value={ADDRESS} mono=false />
        </Stack>
    }
}
