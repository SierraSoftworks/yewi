use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let footer = html! {
        <>
            <Button>{ "Cancel" }</Button>
            <Button variant={ButtonVariant::Primary}>{ "Save changes" }</Button>
        </>
    };

    html! {
        <Card
            title="Maintenance window"
            subtitle="Rollouts only start inside this window"
            {footer}
            attrs={attrs! { "style" => "max-inline-size: 28rem" }}
        >
            { "Weekdays from 22:00 to 04:00, in the fleet's local time." }
        </Card>
    }
}
