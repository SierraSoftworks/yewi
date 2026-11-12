use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let region = use_state(|| None::<AttrValue>);
    let submitted = use_state(|| false);

    let on_change = {
        let region = region.clone();
        Callback::from(move |value| region.set(Some(value)))
    };
    let onsubmit = {
        let submitted = submitted.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            submitted.set(true);
        })
    };

    let error = (*submitted && region.is_none()).then(|| html! { "Choose a region." });
    let created = region.as_ref().filter(|_| *submitted).map(|region| format!("Created in {region}"));

    html! {
        <form {onsubmit} novalidate=true style="max-inline-size: 22rem">
            <Stack>
                <Field label="Region" required=true {error} group=true>
                    <RadioGroup name="region" value={(*region).clone()} {on_change}>
                        <Radio value="eu-west-1">{ "Ireland" }</Radio>
                        <Radio value="us-east-1">{ "Virginia" }</Radio>
                        <Radio value="ap-southeast-2">{ "Sydney" }</Radio>
                    </RadioGroup>
                </Field>
                <Cluster>
                    <Button kind={ButtonKind::Submit} variant={ButtonVariant::Primary}>{ "Create cluster" }</Button>
                </Cluster>
                <span role="status">{ created.unwrap_or_default() }</span>
            </Stack>
        </form>
    }
}
