use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let last = use_state(|| "Nothing yet");
    let act = |name: &'static str| {
        let last = last.clone();
        Callback::from(move |_| last.set(name))
    };

    html! {
        <Stack>
            <Cluster gap={Space::Lg}>
                <ButtonGroup label="Connection actions">
                    <Button on_click={act("Reconnect")}>{ "Reconnect" }</Button>
                    <Button on_click={act("Edit")}>{ "Edit" }</Button>
                    <Button variant={ButtonVariant::Danger} on_click={act("Unlink")}>{ "Unlink" }</Button>
                </ButtonGroup>
                <ButtonGroup label="Mission archive">
                    <Button icon={icons::DOWNLOAD} on_click={act("Download archive")}>{ "Download archive" }</Button>
                    <Button disabled=true>{ "Restore" }</Button>
                </ButtonGroup>
                <ButtonGroup label="Rollout">
                    <Button variant={ButtonVariant::Primary} on_click={act("Approve")}>{ "Approve" }</Button>
                    <Button variant={ButtonVariant::Primary} on_click={act("Approve all")}>{ "Approve all" }</Button>
                </ButtonGroup>
            </Cluster>
            <span role="status">{ format!("Last action: {}", *last) }</span>
        </Stack>
    }
}
