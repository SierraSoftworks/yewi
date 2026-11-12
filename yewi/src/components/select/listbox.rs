//! The floating list of options, shared by `Select`, `Combobox` and
//! `TagPicker`.

use yew::prelude::*;

use super::{SelectOption, dom};
use crate::{Icon, icons};

pub(crate) fn option_id(base: &str, index: usize) -> String {
    format!("{base}-option-{index}")
}

pub(crate) struct Listbox<'a> {
    /// The id the control points `aria-controls` at.
    pub id: AttrValue,
    /// The prefix of each option's id.
    pub base: &'a str,
    pub options: &'a [SelectOption],
    /// The options on show, as indices into `options`.
    pub order: &'a [usize],
    /// The chosen options, as indices into `options`.
    pub selected: &'a [usize],
    /// Whether more than one option can be chosen at once.
    pub multi: bool,
    pub active: Option<usize>,
    /// The control's width, which the list is at least as wide as.
    pub width: f64,
    pub label: Option<AttrValue>,
    pub labelled_by: Option<AttrValue>,
    pub on_pick: Callback<usize>,
    pub on_hover: Callback<usize>,
}

impl Listbox<'_> {
    fn option(&self, index: usize) -> Html {
        let option = &self.options[index];
        let selected = self.selected.contains(&index);
        let class = classes!("yi-select__option", (self.active == Some(index)).then_some("yi-select__option--active"));
        html! {
            <div
                key={index}
                id={option_id(self.base, index)}
                {class}
                role="option"
                aria-selected={if selected { "true" } else { "false" }}
                aria-disabled={option.disabled.then_some("true")}
                data-yi-option={index.to_string()}
            >
                if let Some(icon) = option.icon {
                    <Icon {icon} class="yi-select__option-icon" />
                }
                <span class="yi-select__option-text">
                    <span class="yi-select__option-label">{ option.label.clone() }</span>
                    if let Some(description) = &option.description {
                        <span class="yi-select__option-description">{ description.clone() }</span>
                    }
                </span>
                if selected {
                    <Icon icon={icons::CHECK} class="yi-select__check" />
                }
            </div>
        }
    }

    pub fn render(&self) -> Html {
        // Consecutive options with the same group share one heading.
        let mut sections: Vec<Html> = Vec::new();
        let mut start = 0;
        while start < self.order.len() {
            let group = &self.options[self.order[start]].group;
            let end = start
                + self.order[start..].iter().take_while(|index| self.options[**index].group == *group).count();
            let rows = self.order[start..end].iter().map(|index| self.option(*index));
            sections.push(match group {
                Some(group) => {
                    let first = self.order[start];
                    let heading = format!("{}-group-{first}", self.base);
                    html! {
                        <div key={format!("g{first}")} class="yi-select__group" role="group" aria-labelledby={heading.clone()}>
                            <div id={heading} class="yi-select__group-label" role="presentation">{ group.clone() }</div>
                            { for rows }
                        </div>
                    }
                }
                None => html! { <div key={format!("u{start}")} class="yi-select__group" role="presentation">{ for rows }</div> },
            });
            start = end;
        }

        let onclick = {
            let on_pick = self.on_pick.clone();
            Callback::from(move |event: MouseEvent| {
                if let Some(index) = dom::option_index(&event) {
                    on_pick.emit(index);
                }
            })
        };
        // Movement, not `mouseover`: the list scrolling under a resting
        // pointer during keyboard use must not steal the highlight.
        let onmousemove = {
            let on_hover = self.on_hover.clone();
            Callback::from(move |event: MouseEvent| {
                if let Some(index) = dom::option_index(&event) {
                    on_hover.emit(index);
                }
            })
        };
        // Keeps focus on the control while the pointer works the list.
        let onmousedown = Callback::from(|event: MouseEvent| event.prevent_default());

        html! {
            <div
                id={self.id.clone()}
                class="yi-select__list"
                role="listbox"
                aria-multiselectable={self.multi.then_some("true")}
                aria-label={self.label.clone()}
                aria-labelledby={self.labelled_by.clone()}
                style={format!("--yi-select-anchor-width:{}px", self.width)}
                {onclick}
                {onmousemove}
                {onmousedown}
            >
                { for sections }
            </div>
        }
    }
}
