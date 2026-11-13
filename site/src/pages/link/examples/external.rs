use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <Text tag={TextTag::P}>
                { "yewi is built on " }
                <Link href={NavTarget::href("https://yew.rs").external()}>{ "Yew" }</Link>
                { " and bundled with " }
                <Link href={NavTarget::href("https://trunkrs.dev").external()}>{ "Trunk" }</Link>
                { "." }
            </Text>
            <Cluster gap={Space::Lg}>
                <Link href="#external-and-with-an-icon" icon={icons::BOOK_OPEN}>{ "Read the guide" }</Link>
                <Link href={NavTarget::href("https://github.com/SierraSoftworks/yewi").external()} icon={icons::GIT_BRANCH}>
                    { "Source" }
                </Link>
            </Cluster>
        </Stack>
    }
}
