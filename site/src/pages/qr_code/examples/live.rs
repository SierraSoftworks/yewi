use yew::prelude::*;
use yewi::prelude::*;

const DEVICES: [(&str, &str); 3] = [
    ("Phone", "tak://enrol?host=tak.example.com&user=avery&device=phone"),
    ("Tablet", "tak://enrol?host=tak.example.com&user=avery&device=tablet"),
    ("Radio", "tak://enrol?host=tak.example.com&user=avery&device=radio"),
];

#[function_component]
pub fn Example() -> Html {
    let selected = use_state(|| 0_usize);
    let (device, link) = DEVICES[*selected];

    html! {
        <Stack align={Align::Start}>
            <Cluster>
                { for DEVICES.iter().enumerate().map(|(index, (name, _))| {
                    let selected = selected.clone();
                    let pressed = *selected == index;
                    html! {
                        <Button
                            variant={if pressed { ButtonVariant::Primary } else { ButtonVariant::Default }}
                            attrs={attrs! { "aria-pressed" => pressed.to_string() }}
                            on_click={Callback::from(move |_| selected.set(index))}
                        >
                            { *name }
                        </Button>
                    }
                }) }
            </Cluster>
            <QrCode value={link} label={format!("Enrolment link for the {}", device.to_lowercase())} />
            <span role="status">{ format!("Enrolling the {}", device.to_lowercase()) }</span>
        </Stack>
    }
}
