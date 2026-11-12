use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Grid min="11rem">
            <RadioGroup label="Default" value="on">
                <Radio value="off">{ "Unselected" }</Radio>
                <Radio value="on">{ "Selected" }</Radio>
            </RadioGroup>
            <RadioGroup label="Disabled" value="on" disabled=true>
                <Radio value="off">{ "Disabled" }</Radio>
                <Radio value="on">{ "Disabled, selected" }</Radio>
            </RadioGroup>
            <RadioGroup label="Invalid" invalid=true>
                <Radio value="a">{ "Invalid" }</Radio>
                <Radio value="b">{ "Invalid" }</Radio>
            </RadioGroup>
            <Stack gap={Space::Sm}>
                <RadioGroup label="Small" value="on" size={Size::Small}>
                    <Radio value="on">{ "Small" }</Radio>
                </RadioGroup>
                <RadioGroup label="Medium" value="on">
                    <Radio value="on">{ "Medium" }</Radio>
                </RadioGroup>
                <RadioGroup label="Large" value="on" size={Size::Large}>
                    <Radio value="on">{ "Large" }</Radio>
                </RadioGroup>
            </Stack>
        </Grid>
    }
}
