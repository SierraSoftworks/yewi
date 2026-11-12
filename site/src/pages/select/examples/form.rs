use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let plan = use_state(|| AttrValue::from("team"));
    let submitted = use_state(|| None::<AttrValue>);
    let on_change = {
        let plan = plan.clone();
        Callback::from(move |value: AttrValue| plan.set(value))
    };
    let onsubmit = {
        let (plan, submitted) = (plan.clone(), submitted.clone());
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            submitted.set(Some((*plan).clone()));
        })
    };
    let options = use_memo((), |_| {
        SelectOptions::from([
            SelectOption::new("free", "Free").description("One project"),
            SelectOption::new("team", "Team").description("Unlimited projects"),
            SelectOption::new("enterprise", "Enterprise").description("Single sign-on and audit log"),
        ])
    });

    html! {
        <form {onsubmit} style="max-inline-size: 18rem">
            <Stack>
                <Field label="Plan">
                    <Select name="plan" {options} value={(*plan).clone()} {on_change} />
                </Field>
                <Cluster>
                    <Button kind={ButtonKind::Submit} variant={ButtonVariant::Primary}>{ "Save" }</Button>
                </Cluster>
                <span role="status">
                    { submitted.as_ref().map_or_else(|| "Not saved".to_string(), |plan| format!("Saved plan={plan}")) }
                </span>
            </Stack>
        </form>
    }
}
