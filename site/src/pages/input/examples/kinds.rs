use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Grid min="14rem">
            <Field label="Email">
                <TextInput kind={TextInputKind::Email} value="ben@example.com" autocomplete="email" />
            </Field>
            <Field label="Password">
                <TextInput kind={TextInputKind::Password} value="correct horse" autocomplete="new-password" />
            </Field>
            <Field label="Website">
                <TextInput kind={TextInputKind::Url} placeholder="https://" />
            </Field>
            <Field label="Search">
                <TextInput kind={TextInputKind::Search} placeholder="Search deployments" />
            </Field>
            <Field label="Image digest">
                <TextInput mono=true value="sha256:9f86d081884c7d65" />
            </Field>
        </Grid>
    }
}
