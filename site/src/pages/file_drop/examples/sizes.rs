use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 32rem" }}>
            <FileDrop size={Size::Small} label="Attach a file" hint="Up to 10 MB" icon={icons::PAPERCLIP} />
            <FileDrop size={Size::Medium} hint="Up to 10 MB" />
            <FileDrop size={Size::Large} hint="Up to 10 MB" />
        </Stack>
    }
}
