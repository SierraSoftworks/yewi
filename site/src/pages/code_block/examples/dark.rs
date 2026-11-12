use yew::prelude::*;
use yewi::prelude::*;

const COMPOSE: &str = r#"services:
  docs:
    image: ghcr.io/sierrasoftworks/docs:1.4.2
    ports: ["8080:80"]
    environment:
      LOG_LEVEL: info # debug, info, warn or error
    restart: unless-stopped
"#;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Grid min="20rem">
            <CodeBlock code={COMPOSE} language="yaml" filename="compose.yaml" />
            <CodeBlock code={COMPOSE} language="yaml" filename="compose.yaml" theme={Theme::Dark} />
        </Grid>
    }
}
