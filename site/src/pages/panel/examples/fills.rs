use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Card title="Signing key" attrs={attrs! { "style" => "max-inline-size: 30rem" }}>
            <Panel fill={PanelFill::Plain}>
                { "Used to sign every revision before it is offered to a host." }
            </Panel>
            <Panel title="Strong" fill={PanelFill::Strong}>{ "The stronger tonal fill." }</Panel>
            <Panel title="Soft" fill={PanelFill::Soft}>{ "The lighter tonal fill." }</Panel>
        </Card>
    }
}
