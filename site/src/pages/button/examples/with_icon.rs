use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster>
            <Button variant={ButtonVariant::Primary} icon={icons::PLAY}>{ "Start rollout" }</Button>
            <Button icon={icons::REFRESH_CW}>{ "Refresh" }</Button>
            <Button variant={ButtonVariant::Subtle} icon={icons::COPY}>{ "Copy" }</Button>
            <Button variant={ButtonVariant::Danger} icon={icons::TRASH_2}>{ "Delete" }</Button>
        </Cluster>
    }
}
