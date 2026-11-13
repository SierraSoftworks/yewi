use yew::prelude::*;
use yewi::prelude::*;

// A solid mark: two overlapping squares, the inner one cut out.
const FRAME: IconData = IconData::filled("M3 3h18v18H3zM8 8v8h8V8z");
const DIAMOND: IconData = IconData::filled("M12 2l10 10-10 10L2 12z");

#[function_component]
pub fn Example() -> Html {
    html! {
        <Cluster>
            <Icon icon={FRAME} size="32px" label="Frame" />
            <Icon icon={DIAMOND} size="32px" label="Diamond" />
            <Button icon={DIAMOND}>{ "With a solid icon" }</Button>
        </Cluster>
    }
}
