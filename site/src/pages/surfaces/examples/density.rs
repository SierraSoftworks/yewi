use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let toolbar = || {
        html! {
            <>
                <Button variant={ButtonVariant::Primary}>{ "Approve" }</Button>
                <Button>{ "Pause" }</Button>
                <Button variant={ButtonVariant::Subtle} icon={icons::REFRESH_CW}>{ "Refresh" }</Button>
            </>
        }
    };
    html! {
        <Stack>
            <Cluster>{ toolbar() }</Cluster>
            <Cluster attrs={attrs! { "data-yi-density" => "compact" }}>{ toolbar() }</Cluster>
        </Stack>
    }
}
