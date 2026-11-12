use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    // The name of the file being sent. A real app clears it when the request
    // settles; here a button stands in for the server.
    let sending = use_state(|| None::<AttrValue>);
    let done = use_state(|| 0);

    let on_files = {
        let sending = sending.clone();
        Callback::from(move |files: Vec<SelectedFile>| sending.set(files.first().map(|file| file.name.clone())))
    };
    let finish = {
        let (sending, done) = (sending.clone(), done.clone());
        Callback::from(move |_| {
            sending.set(None);
            done.set(*done + 1);
        })
    };

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 28rem" }}>
            <FileDrop
                busy={sending.is_some()}
                busy_label={sending.as_ref().map(|name| format!("Uploading {name}…")).unwrap_or_default()}
                hint="Uploads start as soon as a file is chosen."
                {on_files}
            />
            <Cluster>
                <Button on_click={finish} disabled={sending.is_none()}>{ "Finish upload" }</Button>
                <span role="status">{ format!("{} uploaded", *done) }</span>
            </Cluster>
        </Stack>
    }
}
