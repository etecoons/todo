use shared_frontend::storage::StorageService;
use shared_frontend::theme::{Theme, mapping::Scheme};
use yew::prelude::*;

#[hook]
pub fn use_theme() -> (UseStateHandle<String>, Callback<MouseEvent>) {
    let theme = use_state(|| {
        let raw = StorageService::new().get_item("theme");
        let raw = if raw.is_empty() {
            Theme::default().name().to_string()
        } else {
            raw
        };
        let theme = if let Some(scheme) = Scheme::from_id(&raw) {
            scheme.to_theme().name().to_string()
        } else {
            Theme::from_name(&raw)
                .unwrap_or_default()
                .name()
                .to_string()
        };
        if theme != raw {
            StorageService::new().set_item("theme", &theme);
        }
        theme
    });

    let toggle_theme = {
        let theme = theme.clone();
        Callback::from(move |_| {
            let current = Theme::from_name(&theme).unwrap_or_default();
            let next = match current {
                Theme::Brinstar => Theme::Norfair,
                Theme::Norfair => Theme::WreckedShip,
                Theme::WreckedShip => Theme::Maridia,
                Theme::Maridia => Theme::Tourian,
                Theme::Tourian => Theme::Crateria,
                Theme::Crateria => Theme::Brinstar,
            };
            if let Some(el) = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.document_element())
            {
                let _ = el.set_attribute("data-theme", next.name());
                let _ = el.set_attribute("class", next.name());
            }
            StorageService::new().set_item("theme", next.name());
            theme.set(next.name().to_string());
        })
    };

    (theme, toggle_theme)
}
