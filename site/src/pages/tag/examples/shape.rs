use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    // Per tag here; an app sets the token once, in its own stylesheet.
    let row = |label: &'static str, radius: &'static str| {
        let attrs = attrs! { "style" => format!("--yi-tag-radius: var({radius})") };
        html! {
            <Cluster gap={Space::Xs} align={Align::Center}>
                <span style="inline-size: 5rem; color: var(--yi-fg-muted)">{ label }</span>
                <Tag attrs={attrs.clone()}>{ "linux" }</Tag>
                <Tag name="env" attrs={attrs.clone()}>{ "production" }</Tag>
                <Tag tone={Tone::Danger} icon={icons::TRIANGLE_ALERT} attrs={attrs.clone()}>{ "destructive" }</Tag>
                <Tag mono=true on_remove={Callback::noop()} {attrs}>{ "v1.42.0" }</Tag>
            </Cluster>
        }
    };

    html! {
        <Stack>
            { row("Squared", "--yi-radius-sm") }
            { row("Rounded", "--yi-radius-full") }
        </Stack>
    }
}
