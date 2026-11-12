use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let chosen = use_state(|| None::<SelectedFile>);
    let on_files = {
        let chosen = chosen.clone();
        Callback::from(move |files: Vec<SelectedFile>| chosen.set(files.into_iter().next()))
    };

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 28rem" }}>
            <FileDrop
                label="Drop a package here, or choose one"
                hint="One at a time. The file goes straight from the browser to the store."
                {on_files}
            />
            <span role="status">
                { match &*chosen {
                    Some(file) => format!("Chosen: {} ({} bytes)", file.name, file.size),
                    None => "Nothing chosen".to_owned(),
                } }
            </span>
        </Stack>
    }
}
