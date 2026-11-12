use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <Stack gap={Space::Lg}>
            <Cluster gap={Space::Xl}>
                <UserChip name="Ada Lovelace" size={Size::Small} />
                <UserChip name="Grace Hopper" detail="Platform team" />
                <UserChip name="Alan Turing" detail="alan@example.com" size={Size::Large} />
            </Cluster>
            <Cluster>
                <UserChip name="Ada Lovelace" variant={UserChipVariant::Soft} size={Size::Small} />
                <UserChip name="Grace Hopper" variant={UserChipVariant::Soft} size={Size::Small} />
                <UserChip name="Alan Turing" variant={UserChipVariant::Soft} size={Size::Small} />
            </Cluster>
            <Cluster>
                <UserChip name="Barbara Liskov" detail="Owner" variant={UserChipVariant::Soft} />
                <UserChip
                    name="Wolfeschlegelsteinhausenbergerdorff the Elder of Philadelphia"
                    detail="Long names truncate"
                    variant={UserChipVariant::Soft}
                />
            </Cluster>
        </Stack>
    }
}
