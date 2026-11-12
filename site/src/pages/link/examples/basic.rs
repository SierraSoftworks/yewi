use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <Text tag={TextTag::P}>
                { "The rollout is paused. Read the " }
                <Link href="#in-text-and-alone">{ "runbook" }</Link>
                { " before you resume it, or ask in " }
                <Link href="#in-text-and-alone">{ "#operations" }</Link>
                { "." }
            </Text>
            <Cluster gap={Space::Lg}>
                <Link href="#in-text-and-alone">{ "Documentation" }</Link>
                <Link href="#in-text-and-alone">{ "Changelog" }</Link>
                <Link href="#in-text-and-alone">{ "Status" }</Link>
            </Cluster>
        </Stack>
    }
}
