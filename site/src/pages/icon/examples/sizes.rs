use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <Cluster gap={Space::Lg} align={Align::End}>
                <Icon icon={icons::LAYERS} size="14px" />
                <Icon icon={icons::LAYERS} size="16px" />
                <Icon icon={icons::LAYERS} size="20px" />
                <Icon icon={icons::LAYERS} size="24px" />
                <Icon icon={icons::LAYERS} size="32px" />
            </Cluster>
            <span style="color: var(--yw-ok)">
                <Icon icon={icons::CIRCLE_CHECK} />{ " All 31 hosts are healthy" }
            </span>
            <span style="color: var(--yw-danger)">
                <Icon icon={icons::CIRCLE_X} />{ " 2 hosts failed their health check" }
            </span>
        </Stack>
    }
}
