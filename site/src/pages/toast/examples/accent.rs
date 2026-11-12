use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
fn Column(props: &ColumnProps) -> Html {
    let accent = props.accent;
    html! {
        <Stack>
            <Toast {accent} tone={Tone::Ok} on_dismiss={Callback::noop()}>{ "Settings saved." }</Toast>
            <Toast {accent} tone={Tone::Warn} title="Saved with warnings" on_dismiss={Callback::noop()}>
                { "Two hosts were skipped because they are offline." }
            </Toast>
            <Toast
                {accent}
                tone={Tone::Danger}
                action={ToastAction::new("Retry", Callback::noop())}
                on_dismiss={Callback::noop()}
            >
                { "Publishing failed." }
            </Toast>
        </Stack>
    }
}

#[derive(Properties, PartialEq)]
struct ColumnProps {
    accent: ToastAccent,
}

#[function_component]
pub fn Example() -> Html {
    html! {
        <Grid min="18rem">
            <Column accent={ToastAccent::Icon} />
            <Column accent={ToastAccent::Stripe} />
        </Grid>
    }
}
