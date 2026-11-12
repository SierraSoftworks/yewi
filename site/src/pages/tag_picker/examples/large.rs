use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let hosts = use_state(Vec::<AttrValue>::new);
    let on_change = {
        let hosts = hosts.clone();
        Callback::from(move |values: Vec<AttrValue>| hosts.set(values))
    };
    // Five thousand hosts across five regions.
    let options = use_memo((), |_| {
        (0..5000)
            .map(|n| {
                let region = ["eu-west", "eu-central", "us-east", "us-west", "ap-south"][n % 5];
                SelectOption::plain(format!("web-{:04}.{region}", n + 1))
            })
            .collect::<SelectOptions>()
    });

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 24rem" }}>
            <Field label="Drain hosts">
                <TagPicker {options} values={(*hosts).clone()} {on_change} mono=true placeholder="Search 5,000 hosts" />
            </Field>
            <span role="status">{ format!("{} hosts chosen", hosts.len()) }</span>
        </Stack>
    }
}
