pub fn setup_online_offline_listeners(
    show_toast: yew::Callback<(String, crate::types::ToastType)>,
    locale: yew::UseStateHandle<crate::i18n::Locale>,
) {
    use crate::types::ToastType;
    use shared_frontend::i18n::Language;
    use shared_frontend::i18n::strings::{StringKey, lookup};
    use wasm_bindgen::JsCast;

    if let Some(window) = web_sys::window() {
        let show_toast_online = show_toast.clone();
        let loc_online = locale.clone();
        let on_online =
            wasm_bindgen::prelude::Closure::<dyn FnMut(_)>::new(move |_: web_sys::Event| {
                let lang = Language::from_code(loc_online.to_str());
                show_toast_online.emit((
                    lookup(StringKey::StatusOnline, lang).to_string(),
                    ToastType::Success,
                ));
            });
        let _ =
            window.add_event_listener_with_callback("online", on_online.as_ref().unchecked_ref());
        on_online.forget();

        let show_toast_offline = show_toast.clone();
        let loc_offline = locale.clone();
        let on_offline =
            wasm_bindgen::prelude::Closure::<dyn FnMut(_)>::new(move |_: web_sys::Event| {
                let lang = Language::from_code(loc_offline.to_str());
                show_toast_offline.emit((
                    lookup(StringKey::StatusOffline, lang).to_string(),
                    ToastType::Error,
                ));
            });
        let _ =
            window.add_event_listener_with_callback("offline", on_offline.as_ref().unchecked_ref());
        on_offline.forget();
    }
}
