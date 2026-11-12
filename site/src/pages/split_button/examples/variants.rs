use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let items = html! {
        <>
            <MenuItem>{ "Save as draft" }</MenuItem>
            <MenuItem>{ "Save a copy" }</MenuItem>
        </>
    };

    html! {
        <Stack>
            <Cluster>
                <SplitButton label="Default">{ items.clone() }</SplitButton>
                <SplitButton label="Primary" variant={ButtonVariant::Primary}>{ items.clone() }</SplitButton>
                <SplitButton label="Subtle" variant={ButtonVariant::Subtle}>{ items.clone() }</SplitButton>
                <SplitButton label="Danger" variant={ButtonVariant::Danger}>{ items.clone() }</SplitButton>
            </Cluster>
            <Cluster>
                <SplitButton label="Small" size={Size::Small}>{ items.clone() }</SplitButton>
                <SplitButton label="Medium">{ items.clone() }</SplitButton>
                <SplitButton label="Large" size={Size::Large}>{ items }</SplitButton>
            </Cluster>
        </Stack>
    }
}
