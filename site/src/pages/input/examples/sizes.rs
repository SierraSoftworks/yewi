use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let row = |size: Size, label: &'static str| {
        html! {
            <Cluster nowrap=true>
                <TextInput {size} placeholder={label} attrs={attrs! { "aria-label" => label }} />
                <NumberInput {size} value={3.0} attrs={attrs! { "aria-label" => "Replicas" }} />
                <Button {size}>{ "Apply" }</Button>
            </Cluster>
        }
    };
    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 28rem" }}>
            { row(Size::Small, "Small") }
            { row(Size::Medium, "Medium") }
            { row(Size::Large, "Large") }
        </Stack>
    }
}
