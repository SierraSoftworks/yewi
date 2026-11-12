use yew::prelude::*;
use yewi::prelude::*;

pub fn view() -> Html {
    html! {
        <>
            <Diff title="caddy.hcl" before={"count = 1\nimage = \"caddy:2.7\"\n"} after={"count = 2\nimage = \"caddy:2.8\"\n"} />
            <Diff mode={DiffMode::Split} unified={"@@ -1 +1 @@\n-a\n+b\n"} />
        </>
    }
}
