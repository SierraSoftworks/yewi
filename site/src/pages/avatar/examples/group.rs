use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack>
            <AvatarGroup label="Reviewers">
                <Avatar name="Ada Lovelace" />
                <Avatar name="Grace Hopper" />
                <Avatar name="Alan Turing" />
            </AvatarGroup>
            <AvatarGroup label="Contributors" max=4>
                <Avatar name="Ada Lovelace" />
                <Avatar name="Grace Hopper" />
                <Avatar name="Alan Turing" />
                <Avatar name="Barbara Liskov" />
                <Avatar name="Edsger Dijkstra" />
                <Avatar name="Margaret Hamilton" />
            </AvatarGroup>
            <AvatarGroup label="Watchers" size={Size::Small} total=128>
                <Avatar name="Ada Lovelace" />
                <Avatar name="Grace Hopper" />
                <Avatar name="Alan Turing" />
            </AvatarGroup>
        </Stack>
    }
}
