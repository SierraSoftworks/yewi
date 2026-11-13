use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let region = use_state(|| AttrValue::from("eu-west-1"));
    let on_change = {
        let region = region.clone();
        Callback::from(move |value: String| region.set(value.into()))
    };
    let options: Rc<[(AttrValue, AttrValue)]> = Rc::from([
        ("eu-west-1".into(), "Ireland".into()),
        ("eu-central-1".into(), "Frankfurt".into()),
        ("us-east-1".into(), "Virginia".into()),
    ]);

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 18rem" }}>
            <Field label="Region" hint="Where new projects are created.">
                <NativeSelect {options} value={(*region).clone()} {on_change} />
            </Field>
            <span role="status">{ format!("Region {}", *region) }</span>
        </Stack>
    }
}
