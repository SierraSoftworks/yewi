use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let scopes = use_state(|| vec![AttrValue::from("read")]);
    let submitted = use_state(|| None::<String>);
    let on_change = {
        let scopes = scopes.clone();
        Callback::from(move |values: Vec<AttrValue>| scopes.set(values))
    };
    let onsubmit = {
        let (scopes, submitted) = (scopes.clone(), submitted.clone());
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let query: Vec<String> = scopes.iter().map(|scope| format!("scope={scope}")).collect();
            submitted.set(Some(query.join("&")));
        })
    };
    let options = use_memo((), |_| {
        SelectOptions::from([
            SelectOption::new("read", "Read"),
            SelectOption::new("write", "Write"),
            SelectOption::new("admin", "Administer"),
        ])
    });

    html! {
        <form {onsubmit} style="max-inline-size: 24rem">
            <Stack>
                <Field label="Token scopes">
                    <TagPicker name="scope" {options} values={(*scopes).clone()} {on_change} />
                </Field>
                <Cluster>
                    <Button kind={ButtonKind::Submit} variant={ButtonVariant::Primary}>{ "Create token" }</Button>
                </Cluster>
                <span role="status">
                    { submitted.as_ref().map_or_else(|| "Not saved".to_string(), |query| format!("Saved {query}")) }
                </span>
            </Stack>
        </form>
    }
}
