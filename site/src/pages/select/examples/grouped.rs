use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let region = use_state(|| None::<AttrValue>);
    let on_change = {
        let region = region.clone();
        Callback::from(move |value: AttrValue| region.set(Some(value)))
    };
    let options = use_memo((), |_| {
        SelectOptions::from([
            SelectOption::new("eu-west-1", "Ireland").description("eu-west-1").icon(icons::SERVER).group("Europe"),
            SelectOption::new("eu-central-1", "Frankfurt")
                .description("eu-central-1")
                .icon(icons::SERVER)
                .group("Europe"),
            SelectOption::new("us-east-1", "Virginia").description("us-east-1").icon(icons::SERVER).group("Americas"),
            SelectOption::new("us-gov-1", "GovCloud")
                .description("Needs approval")
                .icon(icons::LOCK)
                .group("Americas")
                .disabled(true),
            SelectOption::new("sa-east-1", "São Paulo").description("sa-east-1").icon(icons::SERVER).group("Americas"),
        ])
    });

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 18rem" }}>
            <Field label="Region" hint="Where the probe runs.">
                <Select {options} value={(*region).clone()} {on_change} placeholder="Choose a region" />
            </Field>
            <span role="status">
                { region.as_ref().map_or_else(|| "No region chosen".to_string(), |region| format!("Region {region}")) }
            </span>
        </Stack>
    }
}
