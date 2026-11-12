use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster>
            <Avatar name="Ada Lovelace" size={Size::Small} />
            <Avatar name="Ada Lovelace" />
            <Avatar name="Ada Lovelace" size={Size::Large} />
            <Avatar name="Ada Lovelace" attrs={attrs! { "style" => "--yi-avatar-size: 5rem" }} />
        </Cluster>
    }
}
