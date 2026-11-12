use yew::prelude::*;
use yewi::prelude::*;

const STEPS: [(&str, &str); 4] = [
    ("Cart", "3 items"),
    ("Shipping", "Address and method"),
    ("Payment", "Card or invoice"),
    ("Review", "Confirm the order"),
];

#[function_component]
pub fn Example() -> Html {
    let at = use_state(|| 1_usize);
    let go = |to: usize| {
        let at = at.clone();
        Callback::from(move |_: MouseEvent| at.set(to))
    };

    html! {
        <Stack gap={Space::Lg}>
            <Steps label="Checkout progress">
                { for STEPS.iter().enumerate().map(|(i, (title, description))| {
                    let status = match i.cmp(&*at) {
                        std::cmp::Ordering::Less => StepStatus::Done,
                        std::cmp::Ordering::Equal => StepStatus::Current,
                        std::cmp::Ordering::Greater => StepStatus::Pending,
                    };
                    // Only finished steps can be revisited.
                    let on_click = (i < *at).then(|| go(i));
                    html! { <Step title={*title} description={*description} {status} {on_click} /> }
                }) }
            </Steps>
            <Cluster>
                <Button disabled={*at == 0} on_click={go(at.saturating_sub(1))}>{ "Back" }</Button>
                <Button variant={ButtonVariant::Primary} disabled={*at == STEPS.len() - 1} on_click={go(*at + 1)}>
                    { "Next" }
                </Button>
            </Cluster>
            <span role="status">{ format!("Step {} of {}: {}", *at + 1, STEPS.len(), STEPS[*at].0) }</span>
        </Stack>
    }
}
