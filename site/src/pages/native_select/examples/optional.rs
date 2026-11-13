use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    // What was saved before `eu-west-3` was withdrawn.
    let region = use_state(|| AttrValue::from("eu-west-3"));
    let on_change = {
        let region = region.clone();
        Callback::from(move |value: String| region.set(value.into()))
    };
    let options: Rc<[(AttrValue, AttrValue)]> = Rc::from([
        ("eu-west-1".into(), "Ireland".into()),
        ("eu-central-1".into(), "Frankfurt".into()),
        ("us-east-1".into(), "Virginia".into()),
    ]);
    let unknown_label = Callback::from(|value: AttrValue| AttrValue::from(format!("{value} (no longer offered)")));

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 18rem" }}>
            <Field label="Backup region" hint="Optional.">
                <NativeSelect
                    {options}
                    value={(*region).clone()}
                    {on_change}
                    placeholder="No backup"
                    clearable=true
                    {unknown_label}
                />
            </Field>
            <span role="status">
                { if region.is_empty() { "No backup region".to_owned() } else { format!("Backup region {}", *region) } }
            </span>
        </Stack>
    }
}
