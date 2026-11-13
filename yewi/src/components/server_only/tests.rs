use yewi_test::render;

use super::*;
use crate::{CopyButton, NumberInput, ThemeToggle};

fn page() -> Html {
    html! {
        <>
            <CopyButton value="abc" />
            <NumberInput value={1.0} step={0.5} />
            <ThemeToggle />
        </>
    }
}

const HOOKS: [&str; 4] = ["data-yi-copy=", "data-yi-step=", "data-yi-step-by=", "data-yi-theme-toggle"];

#[test]
fn components_declare_their_behaviour_only_inside_server_only() {
    let inside = render(|| html! { <ServerOnly>{ page() }</ServerOnly> });
    let outside = render(page);
    for hook in HOOKS {
        assert!(inside.contains(hook), "{hook} missing inside ServerOnly");
        assert!(!outside.contains(hook), "{hook} rendered outside ServerOnly");
    }
    assert!(!outside.contains("<script"));
}

#[test]
fn the_script_is_inlined_intact_or_linked() {
    let inline = render(|| html! { <ServerOnly /> });
    assert!(inline.contains(BEHAVIOURS), "the inlined script was escaped or altered");

    let linked = render(|| html! { <ServerOnly script="/static/yewi.js" /> });
    assert!(linked.contains(r#"src="/static/yewi.js""#));
    assert!(!linked.contains("yewiBehaviours"));
}

#[test]
fn the_script_cannot_end_its_own_element() {
    assert!(!BEHAVIOURS.to_lowercase().contains("</script"));
}
