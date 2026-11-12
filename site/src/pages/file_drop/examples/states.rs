use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Grid min="16rem">
            <FileDrop hint="Default" />
            <FileDrop hint="Invalid" invalid=true />
            <FileDrop hint="Disabled" disabled=true />
            <FileDrop hint="Busy" busy=true />
            <FileDrop hint="Without an icon" icon={None::<IconData>} />
        </Grid>
    }
}
