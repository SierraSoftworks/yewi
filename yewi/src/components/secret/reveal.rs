use yew::prelude::*;

use crate::{Attrs, Button, ButtonVariant, CopyButton, Icon, IconButton, Size, VisuallyHidden, icons, use_id};

/// What a concealed value shows in place of its text. A fixed length, so the
/// mask gives away nothing about the secret.
const MASK: &str = "••••••••••••••••";

#[derive(Properties, PartialEq)]
pub struct SecretValueProps {
    /// What this value is, shown above it: "Token", "Enrolment link".
    pub label: AttrValue,
    /// The secret. It is what the copy button copies, shown or not.
    pub value: AttrValue,
    /// Starts masked, with a button to show it. Until then the value is not
    /// in the document at all, which suits screens that get shared. Copying
    /// works either way.
    #[prop_or_default]
    pub concealed: bool,
    /// Sets the value in the monospace face. Turn it off for prose such as
    /// a recovery phrase.
    #[prop_or(true)]
    pub mono: bool,
    /// Accessible name of the copy button. The label is attached as its
    /// description, so several values can be told apart.
    #[prop_or(AttrValue::Static("Copy"))]
    pub copy_label: AttrValue,
    /// Accessible name of the show and hide button of a concealed value.
    #[prop_or(AttrValue::Static("Show value"))]
    pub reveal_label: AttrValue,
    /// Fired with the value once it is on the clipboard.
    #[prop_or_default]
    pub on_copy: Callback<AttrValue>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// One labelled secret in a read-only box, with a copy button. Use it inside
/// a [`SecretReveal`], or alone wherever a stored secret is shown.
#[function_component]
pub fn SecretValue(props: &SecretValueProps) -> Html {
    let base = use_id();
    let label_id = AttrValue::from(format!("{base}-label"));
    let shown = use_state_eq(|| false);
    let visible = !props.concealed || *shown;

    let on_toggle = {
        let shown = shown.clone();
        Callback::from(move |_: MouseEvent| shown.set(!visible))
    };
    // When the clipboard is refused the value has to be copied by hand.
    let on_error = {
        let shown = shown.clone();
        Callback::from(move |()| shown.set(true))
    };

    let class = classes!("yi-secret-value", props.mono.then_some("yi-secret-value--mono"), props.class.clone());

    props.attrs.apply(html! {
        <div id={props.id.clone()} {class} role="group" aria-labelledby={label_id.clone()}>
            <span id={label_id.clone()} class="yi-secret-value__label">{ props.label.clone() }</span>
            <div class="yi-secret-value__box">
                if visible {
                    <span class="yi-secret-value__text">{ props.value.clone() }</span>
                } else {
                    <span class="yi-secret-value__text yi-secret-value__text--masked">
                        <span aria-hidden="true">{ MASK }</span>
                        <VisuallyHidden>{ "Hidden" }</VisuallyHidden>
                    </span>
                }
                <span class="yi-secret-value__actions">
                    if props.concealed {
                        <IconButton
                            icon={if visible { icons::EYE_OFF } else { icons::EYE }}
                            label={props.reveal_label.clone()}
                            variant={ButtonVariant::Subtle}
                            size={Size::Small}
                            pressed={visible}
                            on_click={on_toggle}
                            attrs={crate::attrs! { "aria-describedby" => label_id.clone() }}
                        />
                    }
                    <CopyButton
                        value={props.value.clone()}
                        label={props.copy_label.clone()}
                        size={Size::Small}
                        on_copy={props.on_copy.clone()}
                        {on_error}
                        attrs={crate::attrs! { "aria-describedby" => label_id }}
                    />
                </span>
            </div>
        </div>
    })
}

#[derive(Properties, PartialEq)]
pub struct SecretRevealProps {
    /// The heading, which should name what was created: "Token created".
    pub title: AttrValue,
    /// The values, as [`SecretValue`]s, then anything else that belongs with
    /// them: instructions, a follow-up action.
    #[prop_or_default]
    pub children: Html,
    /// Replaces the standard warning under the title. Say what the user must
    /// do and why it cannot be done later.
    #[prop_or_default]
    pub warning: Option<Html>,
    /// Content set beside the values, such as a QR code of an enrolment
    /// link. It moves below them when the panel is narrow.
    #[prop_or_default]
    pub aside: Html,
    /// Shows a button that dismisses the panel, and is fired when it is
    /// pressed. The panel never closes on its own: a secret that vanished
    /// while it was being typed in would have to be created again.
    #[prop_or_default]
    pub on_dismiss: Option<Callback<()>>,
    /// The text of the dismiss button.
    #[prop_or(AttrValue::Static("Done"))]
    pub dismiss_label: AttrValue,
    /// The heading level of the title, 1 to 6.
    #[prop_or(3)]
    pub level: u8,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub attrs: Attrs,
}

/// The panel a freshly created secret is shown in, once.
#[function_component]
pub fn SecretReveal(props: &SecretRevealProps) -> Html {
    let title_id = use_id();
    let on_click = props.on_dismiss.clone().map(|on_dismiss| on_dismiss.reform(|_: MouseEvent| ()));

    props.attrs.apply(html! {
        <section id={props.id.clone()} class={classes!("yi-secret-reveal", props.class.clone())} aria-labelledby={title_id.clone()}>
            <header class="yi-secret-reveal__header">
                <Icon icon={icons::KEY_ROUND} class="yi-secret-reveal__icon" />
                <div class="yi-secret-reveal__heading">
                    <@{format!("h{}", props.level.clamp(1, 6))} id={title_id} class="yi-secret-reveal__title">
                        { props.title.clone() }
                    </@>
                    <p class="yi-secret-reveal__warning">
                        if let Some(warning) = &props.warning {
                            { warning.clone() }
                        } else {
                            { "This is the only time this is shown. Copy it now and keep it somewhere safe." }
                        }
                    </p>
                </div>
                if let Some(on_click) = on_click {
                    <Button class="yi-secret-reveal__dismiss" size={Size::Small} {on_click}>
                        { props.dismiss_label.clone() }
                    </Button>
                }
            </header>
            <div class="yi-secret-reveal__body">
                <div class="yi-secret-reveal__values">{ props.children.clone() }</div>
                if props.aside != Html::default() {
                    <div class="yi-secret-reveal__aside">{ props.aside.clone() }</div>
                }
            </div>
        </section>
    })
}
