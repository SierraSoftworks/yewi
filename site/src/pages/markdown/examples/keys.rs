use yew::prelude::*;
use yewi::prelude::*;

const SOURCE: &str = r#"Press <kbd>Ctrl</kbd> + <kbd>K</kbd> to open the command palette, or
<kbd>?</kbd> for the full list of shortcuts.

Sierra Softworks<br>
Dublin, Ireland
"#;

#[function_component]
pub fn Example() -> Html {
    html! { <Markdown source={SOURCE} /> }
}
