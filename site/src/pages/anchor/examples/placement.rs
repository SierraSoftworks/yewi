use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let placement = use_state(|| None::<Placement>);
    let anchor = use_node_ref();
    let floating = use_node_ref();

    let options = AnchorOptions::new(placement.unwrap_or_default()).offset(8.0);
    use_anchored(&anchor, &floating, placement.is_some(), options);

    html! {
        <Stack gap={Space::Xxxl} align={Align::Center}>
            <Grid columns=3 gap={Space::Xs}>
                { for Placement::ALL.into_iter().map(|option| {
                    let placement = placement.clone();
                    let pressed = (*placement == Some(option)).to_string();
                    html! {
                        <Button
                            size={Size::Small}
                            attrs={attrs! { "aria-pressed" => pressed }}
                            on_click={move |_| placement.set(Some(option))}
                        >
                            { format!("{option:?}") }
                        </Button>
                    }
                }) }
            </Grid>
            <span ref={anchor} class="demo-card">{ "Anchor" }</span>
            if let Some(placement) = *placement {
                <div ref={floating} class="yi-anchored demo-card" popover="manual" role="status">
                    { format!("{placement:?}") }
                </div>
            }
        </Stack>
    }
}
