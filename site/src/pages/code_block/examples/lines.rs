use yew::prelude::*;
use yewi::prelude::*;

const JOB: &str = r#"job "docs" {
  datacenters = ["fra", "ams"]

  group "web" {
    count = 4

    task "caddy" {
      driver = "docker"

      config {
        image = "caddy:2.8-alpine"
        ports = ["http"]
      }

      resources {
        cpu    = 200 # MHz
        memory = 128 # MB
      }
    }
  }
}
"#;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <CodeBlock code={JOB} language="hcl" filename="docs.hcl" line_numbers=true marked="5, 17-18" />
            <CodeBlock code={"- count = 2\n+ count = 4"} language="diff" marked="2" copy=false />
        </Stack>
    }
}
