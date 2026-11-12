use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster>
            <Tooltip tip="Runs every check again from the start">
                <Button icon={icons::REFRESH_CW}>{ "Retry" }</Button>
            </Tooltip>
            <Tooltip tip="Copy the deploy key">
                <Button
                    class="yi-button--icon-only"
                    variant={ButtonVariant::Subtle}
                    icon={icons::COPY}
                    attrs={attrs! { "aria-label" => "Copy" }}
                />
            </Tooltip>
        </Cluster>
    }
}
