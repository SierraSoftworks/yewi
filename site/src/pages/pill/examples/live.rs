use yew::prelude::*;
use yewi::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum Rollout {
    Idle,
    Rolling,
    Done,
}

#[function_component]
pub fn Example() -> Html {
    let state = use_state(|| Rollout::Idle);

    let (tone, label, next, action) = match *state {
        Rollout::Idle => (Tone::Neutral, "idle", Rollout::Rolling, "Start rollout"),
        Rollout::Rolling => (Tone::Active, "rolling", Rollout::Done, "Finish rollout"),
        Rollout::Done => (Tone::Ok, "healthy", Rollout::Idle, "Reset"),
    };
    let on_click = {
        let state = state.clone();
        Callback::from(move |_| state.set(next))
    };

    html! {
        <Stack>
            <Cluster align={Align::Center}>
                <Pill {tone} active={*state == Rollout::Rolling}>{ label }</Pill>
                <Button size={Size::Small} {on_click}>{ action }</Button>
            </Cluster>
            <span role="status">{ format!("Rollout is {label}") }</span>
        </Stack>
    }
}
