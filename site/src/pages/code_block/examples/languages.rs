use yew::prelude::*;
use yewi::prelude::*;

const MANIFEST: &str = r#"[dependencies]
yew = { version = "0.23", features = ["csr"] }
yewi = { version = "0.1", features = ["csr", "code", "highlight"] }
"#;

const DEPLOY: &str = r#"# Build the site, then roll it out to both regions.
trunk build --release
for region in fra ams; do
  nomad job run -var "region=$region" docs.hcl
done
"#;

const RESPONSE: &str = r#"{
  "job": "docs",
  "healthy": true,
  "allocations": 4,
  "regions": ["fra", "ams"]
}
"#;

const QUERY: &str = "from events | where status >= 500 | summarize count() by service";

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <CodeBlock code={MANIFEST} language="toml" filename="Cargo.toml" />
            <CodeBlock code={DEPLOY} language="sh" />
            <CodeBlock code={RESPONSE} language="JSON" />
            <CodeBlock code={QUERY} language="KQL" />
        </Stack>
    }
}
