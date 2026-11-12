use yew::prelude::*;
use yewi::prelude::*;

const SOURCE: &str = r#"use yew::prelude::*;
use yewi::prelude::*;

/// The landing page of the deploy console.
#[function_component]
pub fn App() -> Html {
    let rollouts = use_state(|| 0_u32);
    let on_click = {
        let rollouts = rollouts.clone();
        Callback::from(move |_| rollouts.set(*rollouts + 1))
    };

    html! {
        <Stack>
            <Button variant={ButtonVariant::Primary} {on_click}>{ "Deploy" }</Button>
            <span>{ format!("{} rollouts today", *rollouts) }</span>
        </Stack>
    }
}
"#;

#[function_component]
pub fn Example() -> Html {
    let copied = use_state(|| None::<usize>);
    let on_copy = {
        let copied = copied.clone();
        Callback::from(move |code: AttrValue| copied.set(Some(code.lines().count())))
    };

    html! {
        <Stack>
            <CodeBlock code={SOURCE} language="rust" filename="src/app.rs" {on_copy} />
            <span role="status">
                { match *copied {
                    Some(lines) => format!("Copied {lines} lines"),
                    None => "Nothing copied yet".to_owned(),
                } }
            </span>
        </Stack>
    }
}
