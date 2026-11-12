use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack gap={Space::Xl}>
            <Tabs label="Deployment" variant={TabsVariant::Line}>
                <Tab value="overview" label="Overview">{ "Rollout status across every region." }</Tab>
                <Tab value="changes" label="Changes">{ "What this deployment changes." }</Tab>
                <Tab value="logs" label="Logs">{ "Output from the deploy job." }</Tab>
            </Tabs>
            <Tabs label="Change set" variant={TabsVariant::Pill}>
                <Tab value="changed" label="Changed">{ "Only the resources this plan touches." }</Tab>
                <Tab value="all" label="All">{ "Every resource in the fleet." }</Tab>
            </Tabs>
        </Stack>
    }
}
