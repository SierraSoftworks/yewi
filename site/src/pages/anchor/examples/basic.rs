use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let open = use_state(|| false);
    let anchor = use_node_ref();
    let floating = use_node_ref();

    use_anchored(&anchor, &floating, *open, AnchorOptions::new(Placement::BottomStart));

    let toggle = {
        let open = open.clone();
        Callback::from(move |_| open.set(!*open))
    };

    html! {
        <>
            <span ref={anchor}>
                <Button icon={icons::FILTER} on_click={toggle} attrs={attrs! { "aria-expanded" => open.to_string() }}>
                    { "Filters" }
                </Button>
            </span>
            if *open {
                <div ref={floating} class="yi-anchored demo-card" popover="manual" role="group" aria-label="Filters">
                    { "Anything can go here." }
                </div>
            }
        </>
    }
}
