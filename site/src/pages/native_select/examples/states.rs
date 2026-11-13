use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let options: Rc<[(AttrValue, AttrValue)]> = Rc::from([
        ("eu-west-1".into(), "Ireland".into()),
        ("eu-central-1".into(), "Frankfurt".into()),
        ("us-east-1".into(), "Virginia".into()),
    ]);

    html! {
        <Stack>
            <Grid min="14rem">
                <Field label="Small">
                    <NativeSelect options={options.clone()} value="eu-west-1" size={Size::Small} />
                </Field>
                <Field label="Medium">
                    <NativeSelect options={options.clone()} value="eu-west-1" />
                </Field>
                <Field label="Large">
                    <NativeSelect options={options.clone()} value="eu-west-1" size={Size::Large} />
                </Field>
                <Field label="Empty">
                    <NativeSelect options={options.clone()} placeholder="Choose a region" />
                </Field>
                <Field label="Disabled">
                    <NativeSelect options={options.clone()} value="eu-west-1" disabled=true />
                </Field>
                <Field label="Invalid" error="This region is being retired.">
                    <NativeSelect options={options.clone()} value="us-east-1" />
                </Field>
            </Grid>
            <Cluster>
                <NativeSelect label="Region" fit=true options={options.clone()} value="eu-central-1" />
                <Button>{ "Refresh" }</Button>
            </Cluster>
        </Stack>
    }
}
