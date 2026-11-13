use std::rc::Rc;

use yew::prelude::*;
use yewi::prelude::*;

/// No callbacks: everything here is done by the browser, as it would be on a
/// page rendered by the server alone.
#[function_component]
pub fn Example() -> Html {
    let plans: Rc<[(AttrValue, AttrValue)]> =
        Rc::from([("free".into(), "Free".into()), ("team".into(), "Team".into()), ("enterprise".into(), "Enterprise".into())]);
    let cycles: Rc<[(AttrValue, AttrValue)]> =
        Rc::from([("monthly".into(), "Monthly".into()), ("yearly".into(), "Yearly".into())]);

    html! {
        <form method="get" style="max-inline-size: 18rem">
            <Stack>
                <Field label="Plan">
                    <NativeSelect name="plan" options={plans} value="team" />
                </Field>
                <Field label="Billing" required=true>
                    <NativeSelect name="billing" options={cycles} placeholder="Choose a cycle" />
                </Field>
                <Cluster>
                    <Button kind={ButtonKind::Submit} variant={ButtonVariant::Primary}>{ "Save" }</Button>
                </Cluster>
            </Stack>
        </form>
    }
}
