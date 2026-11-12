use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster nowrap=true attrs={attrs! { "style" => "max-inline-size: 22rem" }}>
            <Field label="Filter services" hide_label=true attrs={attrs! { "style" => "flex: 1" }}>
                <TextInput kind={TextInputKind::Search} placeholder="Filter services" />
            </Field>
            <Button>{ "Clear" }</Button>
        </Cluster>
    }
}
