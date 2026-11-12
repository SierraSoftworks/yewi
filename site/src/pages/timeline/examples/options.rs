use yew::prelude::*;
use yewi::prelude::*;

#[derive(Properties, PartialEq)]
struct LogProps {
    name: AttrValue,
    #[prop_or_default]
    run_off: TimelineRunOff,
    #[prop_or_default]
    divided: bool,
    #[prop_or_default]
    size: Size,
}

#[function_component]
fn Log(props: &LogProps) -> Html {
    html! {
        <Stack gap={Space::Sm}>
            <Text weight={TextWeight::Semibold}>{ props.name.clone() }</Text>
            <Timeline label={props.name.clone()} run_off={props.run_off} divided={props.divided} size={props.size}>
                <TimelineItem title="Deployed" tone={Tone::Ok} meta={html! { "35 minutes ago" }} />
                <TimelineItem title="Approved" meta={html! { "3 hours ago" }} />
                <TimelineItem title="Opened" meta={html! { "Tuesday 14:05" }} />
            </Timeline>
        </Stack>
    }
}

#[function_component]
pub fn Example() -> Html {
    html! {
        <Grid min="11rem" gap={Space::Xl}>
            <Log name="Run-off: both" />
            <Log name="Run-off: end" run_off={TimelineRunOff::End} />
            <Log name="Run-off: start" run_off={TimelineRunOff::Start} />
            <Log name="Run-off: none" run_off={TimelineRunOff::None} />
            <Log name="Divided" divided=true />
            <Log name="Small" size={Size::Small} />
        </Grid>
    }
}
