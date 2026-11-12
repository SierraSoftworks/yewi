use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let open = use_state(|| false);
    let anchor = use_node_ref();
    let floating = use_node_ref();

    use_anchored(&anchor, &floating, *open, AnchorOptions::new(Placement::Top));

    let toggle = {
        let open = open.clone();
        Callback::from(move |_| open.set(!*open))
    };

    html! {
        <div
            class="demo-panel"
            style="block-size: 12rem; overflow: auto"
            tabindex="0"
            role="group"
            aria-label="Scrolling area"
        >
            <div style="block-size: 30rem; padding-block-start: 9rem">
                <span ref={anchor}>
                    <Button on_click={toggle}>{ if *open { "Hide" } else { "Show" } }</Button>
                </span>
                if *open {
                    <div ref={floating} class="yi-anchored demo-card" popover="manual" role="status">
                        { "Above while there is room, below when there is not" }
                    </div>
                }
            </div>
        </div>
    }
}
