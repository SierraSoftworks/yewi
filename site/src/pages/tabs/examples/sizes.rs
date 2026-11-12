use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let sizes = [("Small", Size::Small), ("Medium", Size::Medium), ("Large", Size::Large)];

    html! {
        <Stack gap={Space::Xl}>
            { for sizes.into_iter().map(|(name, size)| html! {
                <Tabs label={name} {size}>
                    <Tab value="summary" label={name} count="4">{ "Summary of the run." }</Tab>
                    <Tab value="steps" label="Steps">{ "Each step and how long it took." }</Tab>
                    <Tab value="output" label="Output">{ "What the run printed." }</Tab>
                </Tabs>
            }) }
        </Stack>
    }
}
