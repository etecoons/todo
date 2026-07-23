//! Backward-compatible `Login` wrapper for `todo`'s state-management model.
//!
//! Todo pre-fetches `PinRequiredResponse` and tracks the current error
//! message in the parent (`app.rs`) rather than in the component. The
//! shared [`shared_frontend::components::login::Login`] is rendered
//! with the right props; this wrapper exists so the call sites keep
//! working with the existing `on_submit` callback shape.

use shared_core::types::PinRequiredResponse;
use shared_frontend::components::login::Login as SharedLogin;
use shared_frontend::i18n::Language;
use yew::prelude::*;

/// Props for the legacy [`Login`] component.
#[derive(Properties, PartialEq, Clone)]
pub struct LoginProps {
    /// Backend-reported PIN-required state, polled by the parent.
    pub pin_required: PinRequiredResponse,
    /// Optional inline error message to display under the input.
    #[prop_or_default]
    pub pin_error: Option<String>,
    /// Fires when the user submits a complete PIN. The parent calls
    /// the verify-pin API and reports back via `pin_error` / `pin_required`.
    pub on_submit: Callback<String>,
    /// Current theme name (kebab-case CSS name) — applied to the form
    /// wrapper.
    #[prop_or_default]
    pub theme: String,
    /// Callback to toggle the theme.
    #[prop_or_default]
    pub on_toggle_theme: Callback<MouseEvent>,
}

#[function_component(Login)]
pub fn login(props: &LoginProps) -> Html {
    let pin_input = use_state(String::new);

    // The shared `Login` owns the input element and the on_input
    // auto-submit logic; this wrapper is a thin pass-through that
    // exposes todo's existing `on_submit: Callback<String>` API to
    // the call site.
    let _on_input = {
        let pin_input = pin_input.clone();
        let pin_len = props.pin_required.length;
        let on_submit = props.on_submit.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let val = input.value();
            let filtered: String = val.chars().filter(|c| c.is_ascii_digit()).collect();
            input.set_value(&filtered);
            if filtered.len() <= pin_len {
                pin_input.set(filtered.clone());
                if filtered.len() == pin_len {
                    on_submit.emit(filtered);
                }
            }
        })
    };

    let _on_form_submit = {
        let pin_input = pin_input.clone();
        let on_submit = props.on_submit.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let val = (*pin_input).clone();
            if !val.is_empty() {
                on_submit.emit(val);
            }
        })
    };

    let prompt_text = if props.pin_required.locked {
        "Account locked. Try again later.".to_string()
    } else {
        "Enter your PIN".to_string()
    };
    let locked_text = "Account locked. Try again later.".to_string();

    let _ = props.theme.clone(); // theme is rendered by the parent's CSS context

    html! {
        <SharedLogin
            pin_required={true}
            pin_length={props.pin_required.length}
            locked={props.pin_required.locked}
            on_verify={props.on_submit.clone()}
            on_login_success={Callback::noop()}
            prompt_text={prompt_text}
            locked_text={locked_text}
            language={Some(Language::English)}
        />
    }
}
