use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let host = use_state(|| None::<AttrValue>);
    let on_change = {
        let host = host.clone();
        Callback::from(move |value: AttrValue| host.set(Some(value)))
    };
    // Five thousand hosts across five regions.
    let options = use_memo((), |_| {
        (0..5000)
            .map(|n| {
                let region = ["eu-west", "eu-central", "us-east", "us-west", "ap-south"][n % 5];
                SelectOption::plain(format!("web-{:04}.{region}.example.net", n + 1))
            })
            .collect::<SelectOptions>()
    });

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 22rem" }}>
            <Field label="Host">
                <Combobox {options} value={(*host).clone()} {on_change} placeholder="Search 5,000 hosts" />
            </Field>
            <span role="status">
                { host.as_ref().map_or_else(|| "No host chosen".to_string(), |host| format!("Host {host}")) }
            </span>
        </Stack>
    }
}
