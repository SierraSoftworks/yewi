use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let row = |size: Size, label: &'static str| {
        html! {
            <>
                <Cluster nowrap=true>
                    <SearchInput {size} value="api" label={format!("{label} search")} />
                    <Button {size}>{ "Apply" }</Button>
                </Cluster>
                <InputGroup {size} prefix="https://" suffix={html! { <Button>{ "Check" }</Button> }}>
                    <TextInput placeholder={label} attrs={attrs! { "aria-label" => label }} />
                </InputGroup>
            </>
        }
    };
    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 22rem" }}>
            { row(Size::Small, "Small") }
            { row(Size::Medium, "Medium") }
            { row(Size::Large, "Large") }
        </Stack>
    }
}
