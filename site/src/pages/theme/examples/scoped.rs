use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Grid columns=2 gap={Space::Sm}>
            <div class="demo-panel" data-theme="light">
                <Stack align={Align::Start}>
                    <span>{ "Always light" }</span>
                    <Button variant={ButtonVariant::Primary}>{ "Approve" }</Button>
                </Stack>
            </div>
            <div class="demo-panel" data-theme="dark">
                <Stack align={Align::Start}>
                    <span>{ "Always dark" }</span>
                    <Button variant={ButtonVariant::Primary}>{ "Approve" }</Button>
                </Stack>
            </div>
        </Grid>
    }
}
