use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <Text tag={TextTag::P}>
                { "yewi is built on " }
                <Link href="https://yew.rs" external=true>{ "Yew" }</Link>
                { " and bundled with " }
                <Link href="https://trunkrs.dev" external=true>{ "Trunk" }</Link>
                { "." }
            </Text>
            <Cluster gap={Space::Lg}>
                <Link href="#external-and-with-an-icon" icon={icons::BOOK_OPEN}>{ "Read the guide" }</Link>
                <Link href="https://github.com/SierraSoftworks/yewi" icon={icons::GIT_BRANCH} external=true>
                    { "Source" }
                </Link>
            </Cluster>
        </Stack>
    }
}
