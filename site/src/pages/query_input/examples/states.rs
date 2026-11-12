use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Grid min="16rem">
            <QueryInput size={Size::Small} label="Small" placeholder="Small" />
            <QueryInput label="Medium" placeholder="Medium" />
            <QueryInput size={Size::Large} label="Large" placeholder="Large" />
            <QueryInput label="Filled" value="status:failed budget" />
            <QueryInput
                label="As code"
                syntax={QuerySyntax::Expression}
                icon={icons::TERMINAL}
                monospace=true
                value={r#"country == "DE""#}
            />
            <QueryInput label="Invalid" syntax={QuerySyntax::Expression} value="country ==" invalid=true />
            <QueryInput label="Disabled" value="status:failed" disabled=true />
        </Grid>
    }
}
