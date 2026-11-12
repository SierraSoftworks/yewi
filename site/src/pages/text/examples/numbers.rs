use yew::prelude::*;
use yewi::prelude::*;

const ROWS: [(&str, &str, &str); 3] = [
    ("caddy-ingress", "3f9a1c2", "12,904"),
    ("grey-probe", "b81e07d", "1,117"),
    ("symbols", "09c4e5a", "88"),
];

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack gap={Space::Xs} attrs={attrs! { "style" => "max-inline-size: 22rem" }}>
            { for ROWS.into_iter().map(|(name, commit, requests)| html! {
                <Cluster justify={Justify::Between} nowrap=true>
                    <Text>{ name }</Text>
                    <Cluster gap={Space::Xl} nowrap=true>
                        <Text mono=true color={TextColor::Muted}>{ commit }</Text>
                        <Text tabular=true attrs={attrs! { "style" => "inline-size: 4rem; text-align: end" }}>
                            { requests }
                        </Text>
                    </Cluster>
                </Cluster>
            }) }
        </Stack>
    }
}
