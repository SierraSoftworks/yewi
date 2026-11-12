use yew::prelude::*;
use yewi::prelude::*;

/// A slice of an accounting chart: every level is itself an account.
const ACCOUNTS: [(&[&str], &str); 12] = [
    (&["Assets"], "1000"),
    (&["Assets", "Current assets"], "1100"),
    (&["Assets", "Current assets", "Cash at bank"], "1110"),
    (&["Assets", "Current assets", "Accounts receivable"], "1120"),
    (&["Assets", "Fixed assets"], "1500"),
    (&["Assets", "Fixed assets", "Equipment"], "1510"),
    (&["Expenses"], "5000"),
    (&["Expenses", "Operating"], "5100"),
    (&["Expenses", "Operating", "Software"], "5110"),
    (&["Expenses", "Operating", "Office equipment"], "5120"),
    (&["Expenses", "Travel"], "5200"),
    (&["Expenses", "Travel", "Flights"], "5210"),
];

#[function_component]
pub fn Example() -> Html {
    let account = use_state(|| Some(AttrValue::from("5110")));
    let on_change = {
        let account = account.clone();
        Callback::from(move |value: Option<AttrValue>| account.set(value))
    };
    let options = use_memo((), |_| {
        ACCOUNTS
            .into_iter()
            .map(|(path, code)| TreeOption::new(path.iter().copied(), code).detail(code))
            .collect::<TreeOptions>()
    });

    html! {
        <Stack attrs={attrs! { "style" => "max-inline-size: 20rem" }}>
            <Field label="Account" hint="Search by name or code">
                <TreeSelect {options} value={(*account).clone()} {on_change} search_label="Search accounts" />
            </Field>
            <span role="status">
                { account.as_ref().map_or_else(|| "No account".to_string(), |code| format!("Posting to {code}")) }
            </span>
        </Stack>
    }
}
