use yew::prelude::*;
use yewi::prelude::*;

const PLAN: &str = r#"--- current
+++ planned
@@ -1,4 +1,4 @@
 datacenter = "home"
-servers    = ["«secret:nomad_server@3f9a»"]
+servers    = ["«secret:nomad_server@b21c»"]
 encrypt    = "«secret:gossip_key@77d0»"
 log_level  = "info"
"#;

/// Splits `«secret:NAME@FINGERPRINT»` references out of a line.
fn secrets(line: AttrValue) -> Html {
    let mut parts = Vec::new();
    let mut rest = line.as_str();
    while let Some((before, tail)) = rest.split_once("«secret:")
        && let Some((token, after)) = tail.split_once('»')
    {
        let (name, fingerprint) = token.rsplit_once('@').unwrap_or((token, ""));
        parts.push(html! {
            <>
                { before.to_owned() }
                <strong title={format!("Secret {name}, value fingerprint {fingerprint}")}>
                    <Icon icon={icons::KEY_ROUND} />
                    { format!(" {name}") }
                    <small>{ format!("@{fingerprint}") }</small>
                </strong>
            </>
        });
        rest = after;
    }
    html! { <>{ for parts }{ rest.to_owned() }</> }
}

#[function_component]
pub fn Example() -> Html {
    html! { <Diff unified={PLAN} render_line={Callback::from(secrets)} /> }
}
