use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let query = use_state(AttrValue::default);
    let scope = use_state(|| AttrValue::from("all"));
    let searched = use_state(|| None::<String>);

    let scopes: Rc<[(AttrValue, AttrValue)]> =
        Rc::from([("all".into(), "All".into()), ("errors".into(), "Errors".into()), ("deploys".into(), "Deploys".into())]);

    let on_input = {
        let query = query.clone();
        Callback::from(move |value: String| query.set(value.into()))
    };
    let set_scope = {
        let scope = scope.clone();
        Callback::from(move |value: String| scope.set(value.into()))
    };
    let on_search = {
        let searched = searched.clone();
        Callback::from(move |query: String| searched.set(Some(query)))
    };
    let on_click = {
        let (query, on_search) = (query.clone(), on_search.clone());
        Callback::from(move |_| on_search.emit(query.to_string()))
    };
    let on_clear = {
        let searched = searched.clone();
        Callback::from(move |()| searched.set(None))
    };

    let suffix = html! {
        <>
            <InputGroupSelect label="Scope" options={scopes} value={(*scope).clone()} on_change={set_scope} />
            <Button variant={ButtonVariant::Primary} {on_click}>{ "Search" }</Button>
        </>
    };

    html! {
        <Stack>
            <SearchInput
                size={Size::Large}
                value={(*query).clone()}
                label="Search events"
                placeholder="Search events"
                {on_input}
                {on_search}
                {on_clear}
                {suffix}
            />
            <span role="status">
                { match &*searched {
                    Some(query) => format!("Searched {} for “{query}”", *scope),
                    None => "No search yet".to_owned(),
                } }
            </span>
            <SearchInput value="timeout" loading=true label="Search logs" placeholder="Search logs" />
        </Stack>
    }
}
