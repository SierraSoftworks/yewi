use yew::prelude::*;
use yewi::prelude::*;

const UPDATE: &str = r#"# What happened

Requests to the EU gateway failed for **14 minutes** after a certificate was
renewed with the wrong chain.

# What we are doing

- The certificate has been reissued and `eu-gateway` is healthy again.
- Renewals now verify the full chain before they are installed.

## Still to do

Review the other regions for the same renewal path.
"#;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Card
            title="EU gateway outage"
            subtitle="Resolved, updated 12 minutes ago"
            attrs={attrs! { "style" => "max-inline-size: 32rem" }}
        >
            <Markdown source={UPDATE} top_heading={HeadingLevel::H4} />
        </Card>
    }
}
