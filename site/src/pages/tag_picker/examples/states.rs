use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    let options = use_memo((), |_| {
        ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"]
            .into_iter()
            .map(SelectOption::plain)
            .collect::<SelectOptions>()
    });
    let days = |days: &[&'static str]| days.iter().map(|day| AttrValue::from(*day)).collect::<Vec<_>>();
    let picker = |label: &'static str, values: Vec<AttrValue>, disabled: bool, invalid: bool| {
        html! {
            <Stack gap={Space::Xs}>
                <span style="font-size: var(--yi-text-sm); font-weight: 500" aria-hidden="true">{ label }</span>
                <TagPicker options={options.clone()} {values} {label} {disabled} {invalid} placeholder="Add days" />
            </Stack>
        }
    };

    html! {
        <Grid min="16rem">
            { picker("Empty", days(&[]), false, false) }
            { picker("Chosen", days(&["Monday", "Friday"]), false, false) }
            { picker("Wrapping", days(&["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"]), false, false) }
            { picker("Disabled", days(&["Monday", "Friday"]), true, false) }
            { picker("Invalid", days(&["Sunday"]), false, true) }
        </Grid>
    }
}
