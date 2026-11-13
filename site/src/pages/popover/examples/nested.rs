use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let content = html! {
        <Stack gap={Space::Sm}>
            <span>{ "Checked from three regions every minute." }</span>
            <Popover content="Each region must fail twice in a row before the probe is marked failing." placement={Placement::Right}>
                <Button size={Size::Small} variant={ButtonVariant::Subtle}>{ "How failures are counted" }</Button>
            </Popover>
        </Stack>
    };

    html! {
        <Popover header="api.example.com" {content} label="Probe" placement={Placement::BottomStart}>
            <Button icon={icons::ACTIVITY}>{ "Probe" }</Button>
        </Popover>
    }
}
