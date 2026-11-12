use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
fn Notify() -> Html {
    let toast = use_toast();
    let count = use_state(|| 0_u32);
    let on_click = Callback::from(move |_| {
        let next = *count + 1;
        count.set(next);
        toast.info(format!("Notification {next}"));
    });
    html! { <Button variant={ButtonVariant::Primary} {on_click}>{ "Notify" }</Button> }
}

#[function_component]
pub fn Example() -> Html {
    let placement = use_state(ToastPlacement::default);
    let choose = |label: &'static str, value: ToastPlacement| {
        let placement = placement.clone();
        let pressed = if *placement == value { "true" } else { "false" };
        html! {
            <Button attrs={attrs! { "aria-pressed" => pressed }} on_click={move |_| placement.set(value)}>
                { label }
            </Button>
        }
    };

    html! {
        <ToastHost placement={*placement}>
            <Stack>
                <Cluster>
                    { choose("Top start", ToastPlacement::TopStart) }
                    { choose("Top center", ToastPlacement::TopCenter) }
                    { choose("Top end", ToastPlacement::TopEnd) }
                    { choose("Bottom start", ToastPlacement::BottomStart) }
                    { choose("Bottom center", ToastPlacement::BottomCenter) }
                    { choose("Bottom end", ToastPlacement::BottomEnd) }
                </Cluster>
                <Cluster><Notify /></Cluster>
            </Stack>
        </ToastHost>
    }
}
