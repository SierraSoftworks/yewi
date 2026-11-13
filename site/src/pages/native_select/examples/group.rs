use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let units: Rc<[(AttrValue, AttrValue)]> =
        Rc::from([("sec".into(), "seconds".into()), ("min".into(), "minutes".into()), ("hr".into(), "hours".into())]);
    let unit = html! { <NativeSelect label="Unit" name="unit" options={units} value="min" /> };

    html! {
        <Field label="Retry after" attrs={attrs! { "style" => "max-inline-size: 18rem" }}>
            <InputGroup suffix={unit}>
                <NumberInput name="retry" value={5.0} min={1.0} stepper=false />
            </InputGroup>
        </Field>
    }
}
