use yew::prelude::*;
use yewi::prelude::*;

const FILTERS: [&str; 3] = ["All", "Healthy", "Needs attention"];

#[function_component]
pub fn Example() -> Html {
    let filter = use_state(|| FILTERS[0]);

    html! {
        <Stack gap={Space::Lg}>
            <PageHeader
                title="Hosts"
                subtitle={html! { "31 hosts across 3 clusters." }}
                actions={html! { <Button icon={icons::PLUS}>{ "Enrol host" }</Button> }}
                divided=true
            >
                <Cluster attrs={attrs! { "role" => "group", "aria-label" => "Filter hosts" }}>
                    { for FILTERS.into_iter().map(|name| {
                        let on_click = {
                            let filter = filter.clone();
                            Callback::from(move |_| filter.set(name))
                        };
                        html! {
                            <Button
                                size={Size::Small}
                                variant={ButtonVariant::Subtle}
                                attrs={attrs! { "aria-pressed" => (*filter == name).to_string() }}
                                {on_click}
                            >
                                { name }
                            </Button>
                        }
                    }) }
                </Cluster>
            </PageHeader>
            <span role="status">{ format!("Showing: {}", *filter) }</span>
        </Stack>
    }
}
