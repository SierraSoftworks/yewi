use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <Cluster>
                <IconButton icon={icons::PLUS} label="Add host" variant={ButtonVariant::Primary} />
                <IconButton icon={icons::REFRESH_CW} label="Refresh" />
                <IconButton icon={icons::PENCIL} label="Rename" variant={ButtonVariant::Subtle} />
                <IconButton icon={icons::TRASH_2} label="Delete" variant={ButtonVariant::Danger} />
                <IconButton icon={icons::SETTINGS} label="Settings" disabled=true />
                <IconButton icon={icons::DOWNLOAD} label="Download archive" loading=true />
            </Cluster>
            <Cluster>
                <IconButton icon={icons::COPY} label="Copy (small)" size={Size::Small} />
                <IconButton icon={icons::COPY} label="Copy (medium)" />
                <IconButton icon={icons::COPY} label="Copy (large)" size={Size::Large} />
                <IconButton icon={icons::EXTERNAL_LINK} label="Open the source" href="https://github.com/SierraSoftworks/yewi" />
            </Cluster>
        </Stack>
    }
}
