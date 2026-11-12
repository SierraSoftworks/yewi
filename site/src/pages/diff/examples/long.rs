use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let rule = |n: u32, action: &str| {
        format!("allow from 10.0.{n}.0/24 to service \"edge-{n}\" port 443 action {action} # reviewed quarterly by the platform team\n")
    };
    let before: String = (1..=40).map(|n| rule(n, "accept")).collect();
    let after: String = (1..=40).map(|n| rule(n, if n % 13 == 0 { "reject" } else { "accept" })).collect();

    html! {
        <Diff
            title="firewall.rules"
            {before}
            {after}
            context={2}
            wrap=true
            attrs={attrs! { "style" => "--yi-diff-max-height: 18rem" }}
        />
    }
}
