use yew::prelude::*;
use yewi::prelude::*;

const STYLES: [(CardHeader, &str); 3] =
    [(CardHeader::Plain, "Plain"), (CardHeader::Ruled, "Ruled"), (CardHeader::Tonal, "Tonal")];

#[function_component]
pub fn Example() -> Html {
    html! {
        <Grid min="15rem">
            { for STYLES.into_iter().map(|(header, name)| html! {
                <Card
                    title={name}
                    subtitle="Certificate authority"
                    actions={html! { <Button size={Size::Small}>{ "Renew" }</Button> }}
                    {header}
                >
                    { "The signing certificate expires in 212 days." }
                </Card>
            }) }
        </Grid>
    }
}
