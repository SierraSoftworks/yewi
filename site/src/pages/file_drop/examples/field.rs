use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let certificate = use_state(|| None::<AttrValue>);
    let submitted = use_state(|| false);

    let on_files = {
        let certificate = certificate.clone();
        Callback::from(move |files: Vec<SelectedFile>| certificate.set(files.first().map(|file| file.name.clone())))
    };
    let onsubmit = {
        let submitted = submitted.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            submitted.set(true);
        })
    };

    let error = (*submitted && certificate.is_none()).then(|| html! { "Choose a certificate to import." });
    let hint = match &*certificate {
        Some(name) => html! { format!("Chosen: {name}") },
        None => html! { "A PEM or PKCS #12 file." },
    };

    html! {
        <form {onsubmit} novalidate=true style="max-inline-size: 28rem">
            <Stack>
                <Field label="Name">
                    <TextInput value="Field team" />
                </Field>
                <Field label="Certificate" required=true {hint} {error}>
                    <FileDrop accept=".pem,.p12" icon={icons::FILE_KEY} {on_files} />
                </Field>
                <Cluster>
                    <Button variant={ButtonVariant::Primary} kind={ButtonKind::Submit}>{ "Import" }</Button>
                </Cluster>
                if *submitted && certificate.is_some() {
                    <span role="status">{ "Imported" }</span>
                }
            </Stack>
        </form>
    }
}
