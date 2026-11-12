use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Field label="Latency alert range (ms)" hint="Alerts fire outside this range." group=true>
            <Cluster nowrap=true attrs={attrs! { "style" => "max-inline-size: 16rem" }}>
                <NumberInput value={50.0} min={0.0} stepper=false attrs={attrs! { "aria-label" => "Minimum" }} />
                <span aria-hidden="true">{ "to" }</span>
                <NumberInput value={400.0} min={0.0} stepper=false attrs={attrs! { "aria-label" => "Maximum" }} />
            </Cluster>
        </Field>
    }
}
