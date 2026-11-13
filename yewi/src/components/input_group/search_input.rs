use yew::prelude::*;

use super::InputGroup;
use crate::components::native_select::set_value;
use crate::{Attrs, ButtonVariant, FieldContext, Icon, IconButton, Size, Spinner, TextInput, TextInputKind, icons};

/// The current `value` of the element behind `node`.
fn current_value(node: &NodeRef) -> String {
    #[cfg(target_arch = "wasm32")]
    {
        node.get()
            .and_then(|element| js_sys::Reflect::get(&element, &"value".into()).ok())
            .and_then(|value| value.as_string())
            .unwrap_or_default()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = node;
        String::new()
    }
}

fn focus(node: &NodeRef) {
    #[cfg(target_arch = "wasm32")]
    if let Some(element) = node.cast::<web_sys::HtmlElement>() {
        let _ = element.focus();
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = node;
    }
}

#[derive(Properties, PartialEq)]
pub struct SearchInputProps {
    /// The query shown. Update it from `on_input` to keep the input
    /// controlled; leave it unset and the browser keeps the text itself.
    #[prop_or_default]
    pub value: Option<AttrValue>,
    /// Fired with the new query on every edit, and with an empty string when
    /// it is cleared. Filter a list that is already loaded from here.
    #[prop_or_default]
    pub on_input: Callback<String>,
    /// Fired with the query when the user commits it: on blur or Enter, if
    /// it changed.
    #[prop_or_default]
    pub on_change: Callback<String>,
    /// Fired with the query on Enter, changed or not. Run a search that
    /// costs something (a request) from here.
    #[prop_or_default]
    pub on_search: Callback<String>,
    /// Fired after the query is cleared with the clear button or Escape.
    #[prop_or_default]
    pub on_clear: Callback<()>,
    #[prop_or_default]
    pub on_focus: Callback<FocusEvent>,
    #[prop_or_default]
    pub on_blur: Callback<FocusEvent>,
    /// Fired after the input has handled its own keys (Enter and Escape).
    #[prop_or_default]
    pub on_keydown: Callback<KeyboardEvent>,
    /// The accessible name. Defaults to "Search" outside a `Field`; inside
    /// one the field's label names the input.
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// What can be searched, e.g. "Search services". Must not be empty: the
    /// clear button shows whenever the placeholder does not.
    #[prop_or(AttrValue::Static("Search"))]
    pub placeholder: AttrValue,
    #[prop_or_default]
    pub size: Size,
    /// A search is running: swaps the icon for a spinner and marks the
    /// input busy.
    #[prop_or_default]
    pub loading: bool,
    /// Shown after the clear button: a `Button` to run the search or an
    /// `NativeSelect` for its scope.
    #[prop_or_default]
    pub suffix: Html,
    /// The name the query is submitted under in a native form.
    #[prop_or_default]
    pub name: Option<AttrValue>,
    /// The native `autocomplete` hint. Off by default, so the browser's
    /// history does not cover the results.
    #[prop_or(AttrValue::Static("off"))]
    pub autocomplete: AttrValue,
    #[prop_or_default]
    pub disabled: bool,
    /// Accessible name of the clear button.
    #[prop_or(AttrValue::Static("Clear search"))]
    pub clear_label: AttrValue,
    /// A reference to the `<input>`, for moving focus.
    #[prop_or_default]
    pub input_ref: NodeRef,
    #[prop_or_default]
    pub class: Classes,
    /// The id of the `<input>`. Ignored inside a `Field`.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// Extra attributes for the `<input>` (not the wrapper).
    #[prop_or_default]
    pub attrs: Attrs,
}

/// A search box: a magnifier, the query, and a clear button once there is
/// text. Escape clears the query and Enter runs the search.
#[function_component]
pub fn SearchInput(props: &SearchInputProps) -> Html {
    let in_field = use_context::<FieldContext>().is_some();
    let input = props.input_ref.clone();

    let clear = {
        let (input, on_input, on_clear) = (input.clone(), props.on_input.clone(), props.on_clear.clone());
        move || {
            set_value(&input, "");
            on_input.emit(String::new());
            on_clear.emit(());
            focus(&input);
        }
    };

    let on_keydown = {
        let (input, clear) = (input.clone(), clear.clone());
        let (on_search, on_keydown) = (props.on_search.clone(), props.on_keydown.clone());
        Callback::from(move |event: KeyboardEvent| {
            match event.key().as_str() {
                // Not prevented: Enter still submits a surrounding form.
                "Enter" => on_search.emit(current_value(&input)),
                // An empty box lets Escape through, to close a dialog or menu.
                "Escape" if !current_value(&input).is_empty() => {
                    event.prevent_default();
                    event.stop_propagation();
                    clear();
                }
                _ => {}
            }
            on_keydown.emit(event);
        })
    };

    let label = props.label.clone().or((!in_field).then_some(AttrValue::Static("Search")));
    let attrs = Attrs::new(
        label
            .into_iter()
            .map(|label| ("aria-label", label))
            .chain(props.attrs.iter().map(|(key, value)| (key, value.clone()))),
    );

    let prefix = html! {
        if props.loading {
            <Spinner />
        } else {
            <Icon icon={icons::SEARCH} />
        }
    };
    let suffix = html! {
        <>
            <span
                class="yi-search-input__clear"
                // Keeps focus (and the caret) in the input.
                onmousedown={Callback::from(|event: MouseEvent| event.prevent_default())}
            >
                <IconButton
                    icon={icons::X}
                    label={props.clear_label.clone()}
                    variant={ButtonVariant::Subtle}
                    size={Size::Small}
                    on_click={Callback::from(move |_| clear())}
                    // Escape does the same from the keyboard.
                    attrs={crate::attrs! { "tabindex" => "-1" }}
                />
            </span>
            { props.suffix.clone() }
        </>
    };

    html! {
        <InputGroup
            class={classes!("yi-search-input", props.class.clone())}
            size={props.size}
            {prefix}
            {suffix}
            attrs={if props.loading { crate::attrs! { "aria-busy" => "true" } } else { Attrs::default() }}
        >
            <TextInput
                kind={TextInputKind::Search}
                value={props.value.clone()}
                on_input={props.on_input.clone()}
                on_change={props.on_change.clone()}
                on_focus={props.on_focus.clone()}
                on_blur={props.on_blur.clone()}
                {on_keydown}
                placeholder={props.placeholder.clone()}
                name={props.name.clone()}
                autocomplete={props.autocomplete.clone()}
                disabled={props.disabled}
                input_ref={input}
                id={props.id.clone()}
                {attrs}
            />
        </InputGroup>
    }
}
