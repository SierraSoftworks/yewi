use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let images = use_state(Vec::<SelectedFile>::new);
    let problem = use_state(|| None::<String>);

    let on_files = {
        let (images, problem) = (images.clone(), problem.clone());
        Callback::from(move |files: Vec<SelectedFile>| {
            let mut all = (*images).clone();
            all.extend(files);
            images.set(all);
            problem.set(None);
        })
    };
    let on_reject = {
        let problem = problem.clone();
        Callback::from(move |rejected: Vec<RejectedFile>| {
            let reasons: Vec<String> = rejected
                .iter()
                .map(|rejected| match rejected.reason {
                    FileRejection::Type => format!("{} is not an image", rejected.file.name),
                    FileRejection::TooLarge => format!("{} is larger than 1 MB", rejected.file.name),
                    FileRejection::TooMany => format!("{} was one too many", rejected.file.name),
                })
                .collect();
            problem.set(Some(reasons.join(", ")));
        })
    };

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 28rem" }}>
            <FileDrop
                multiple=true
                accept="image/*"
                max_size={1024 * 1024}
                icon={icons::IMAGE_UP}
                label="Drop map tiles here, or choose them"
                hint="Images up to 1 MB each."
                {on_files}
                {on_reject}
            />
            if let Some(problem) = &*problem {
                <span role="alert">{ format!("Skipped: {problem}.") }</span>
            }
            <span role="status">{ format!("{} added", images.len()) }</span>
            <ul aria-label="Added images">
                { for images.iter().map(|file| html! { <li>{ file.name.clone() }</li> }) }
            </ul>
        </Stack>
    }
}
