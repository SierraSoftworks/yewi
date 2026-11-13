use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let zoom = use_state(|| 8_u8);
    let layers = use_state(|| false);

    let step = |by: i8| {
        let zoom = zoom.clone();
        Callback::from(move |_| zoom.set(zoom.saturating_add_signed(by)))
    };
    let toggle_layers = {
        let layers = layers.clone();
        Callback::from(move |_| layers.set(!*layers))
    };

    html! {
        <Stack>
            <Cluster gap={Space::Xl} align={Align::Start}>
                <ButtonGroup label="Map zoom" orientation={Orientation::Vertical}>
                    <IconButton icon={icons::PLUS} label="Zoom in" disabled={*zoom >= 10} on_click={step(1)} />
                    <IconButton icon={icons::MINUS} label="Zoom out" disabled={*zoom <= 6} on_click={step(-1)} />
                </ButtonGroup>
                <ButtonGroup label="Map tools">
                    <IconButton icon={icons::LOCATE} label="Centre on my position" />
                    <IconButton icon={icons::LAYERS} label="Show layers" pressed={*layers} on_click={toggle_layers} />
                    <IconButton icon={icons::MAXIMIZE} label="Full screen" />
                </ButtonGroup>
                <ButtonGroup label="Row actions">
                    <Button size={Size::Small}>{ "Edit" }</Button>
                    <Button size={Size::Small}>{ "Duplicate" }</Button>
                    <IconButton size={Size::Small} icon={icons::ELLIPSIS} label="More actions" />
                </ButtonGroup>
            </Cluster>
            <span role="status">
                { format!("Zoom {}, layers {}", *zoom, if *layers { "shown" } else { "hidden" }) }
            </span>
        </Stack>
    }
}
