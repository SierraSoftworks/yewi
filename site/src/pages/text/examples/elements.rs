use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 32rem" }}>
            <Text tag={TextTag::P} size={TextSize::Lg}>
                { "Pausing a rollout " }
                <Text tag={TextTag::Strong}>{ "does not roll back" }</Text>
                { " hosts that have already been updated. Resume it, or start a rollback " }
                <Text tag={TextTag::Em}>{ "before" }</Text>
                { " the next maintenance window." }
            </Text>
            <Text tag={TextTag::Small} color={TextColor::Subtle}>{ "Rollbacks are recorded in the audit log." }</Text>
        </Stack>
    }
}
