use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

const SCHEMA: &str = r#"{
  "type": "object",
  "required": ["account"],
  "properties": {
    "account": { "type": "string", "format": "connection", "description": "The Todoist account tasks are created in." },
    "filter": { "type": "string", "format": "filter", "description": "Only items matching this are forwarded." },
    "title": { "title": "Task title", "type": "string", "examples": ["Read: {{ title }}"] }
  }
}"#;

#[function_component]
pub fn Example() -> Html {
    let schema = use_memo((), |_| Json::parse(SCHEMA).unwrap_or(Json::Null));
    let value = use_state(|| Json::parse(r#"{"account":"acct_2"}"#).unwrap_or(Json::Null));
    let on_change = {
        let value = value.clone();
        Callback::from(move |next| value.set(next))
    };

    // What the app knows and the schema does not: the linked accounts and
    // the properties a filter can name.
    let accounts = use_memo((), |_| {
        SelectOptions::from([SelectOption::new("acct_1", "Personal"), SelectOption::new("acct_2", "Sierra Softworks")])
    });
    let fields = use_memo((), |_| -> Rc<[QueryField]> {
        Rc::from([
            QueryField::new("author").description("Who wrote the item"),
            QueryField::new("tag").description("A tag on the item").values(["rust", "release", "security"]),
        ])
    });

    let render = use_callback((accounts, fields), |control: SchemaControl, (accounts, fields)| {
        let text = match &control.value {
            Some(Json::String(text)) => text.clone(),
            _ => AttrValue::default(),
        };
        let SchemaControl { on_change, disabled, .. } = control;
        match control.format.as_deref() {
            Some("connection") => Some(html! {
                <Select
                    options={(**accounts).clone()}
                    value={text}
                    placeholder="Choose an account"
                    {disabled}
                    on_change={on_change.reform(|account: AttrValue| Some(Json::String(account)))}
                />
            }),
            Some("filter") => Some(html! {
                <QueryInput
                    value={text}
                    fields={(**fields).clone()}
                    placeholder="tag:release"
                    {disabled}
                    on_input={on_change.reform(|query: String| (!query.is_empty()).then(|| Json::from(query)))}
                />
            }),
            _ => None,
        }
    });

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 28rem" }}>
            <SchemaForm {schema} value={(*value).clone()} {on_change} {render} />
            <code role="status">{ value.to_string() }</code>
        </Stack>
    }
}
