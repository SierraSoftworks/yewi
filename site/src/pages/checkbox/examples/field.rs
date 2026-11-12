use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let accepted = use_state(|| false);
    let submitted = use_state(|| false);

    let on_change = {
        let accepted = accepted.clone();
        Callback::from(move |value| accepted.set(value))
    };
    let onsubmit = {
        let submitted = submitted.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            submitted.set(true);
        })
    };

    let error = (*submitted && !*accepted).then(|| html! { "Confirm the change window to continue." });
    let done = *submitted && *accepted;

    html! {
        <form {onsubmit} novalidate=true style="max-inline-size: 24rem">
            <Stack>
                <Field label="Change window" required=true {error}>
                    <Checkbox name="confirmed" checked={*accepted} {on_change}>
                        { "This deploy is inside an approved change window" }
                    </Checkbox>
                </Field>
                <Cluster>
                    <Button kind={ButtonKind::Submit} variant={ButtonVariant::Primary}>{ "Deploy" }</Button>
                </Cluster>
                <span role="status">{ if done { "Deploy started" } else { "" } }</span>
            </Stack>
        </form>
    }
}
