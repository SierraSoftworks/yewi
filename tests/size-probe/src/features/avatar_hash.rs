use yew::prelude::*;
use yewi::prelude::*;

/// Only the hashing: `Avatar` itself is a core component, so rendering one
/// here would charge its whole cost to this feature.
pub fn view() -> Html {
    html! { <img src={avatar_url("ada@example.com", 64)} alt="" /> }
}
