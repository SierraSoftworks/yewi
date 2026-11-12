use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let template = use_state(|| AttrValue::from("Deployed "));
    let on_input = {
        let template = template.clone();
        Callback::from(move |value: String| template.set(value.into()))
    };
    let insert = |variable: &'static str| {
        let template = template.clone();
        Callback::from(move |_| template.set(format!("{}{{{variable}}}", *template).into()))
    };

    let variables = html! {
        <MenuButton label="Insert" icon={icons::BRACES} placement={Placement::BottomEnd}>
            <MenuItem on_select={insert("service")}>{ "Service name" }</MenuItem>
            <MenuItem on_select={insert("version")}>{ "Version" }</MenuItem>
            <MenuItem on_select={insert("author")}>{ "Author" }</MenuItem>
        </MenuButton>
    };

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 24rem" }}>
            <Field label="Message template">
                <InputGroup suffix={variables}>
                    <TextInput value={(*template).clone()} {on_input} />
                </InputGroup>
            </Field>
            <span role="status">{ format!("Template: {}", *template) }</span>
        </Stack>
    }
}
