use axum::{
    extract::State,
    http::{HeaderMap, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::cookie::CookieJar;

use crate::auth::secure_compare;
use crate::state::{AppState, SharedState};

pub fn is_authenticated(state: &AppState, cookie_jar: &CookieJar, headers: &HeaderMap) -> bool {
    let pin_env = match &state.pin {
        Some(p) => p,
        None => return true,
    };

    let provided_pin = cookie_jar
        .get("RUSTDO_PIN")
        .map(|c| c.value())
        .or_else(|| headers.get("x-pin").and_then(|h| h.to_str().ok()));

    if let Some(provided) = provided_pin {
        secure_compare(provided, pin_env)
    } else {
        false
    }
}

pub async fn auth_middleware(
    State(state): State<SharedState>,
    cookie_jar: CookieJar,
    headers: HeaderMap,
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    if is_authenticated(&state, &cookie_jar, &headers) {
        next.run(request).await
    } else {
        (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({ "error": "Invalid PIN" })),
        )
            .into_response()
    }
}

pub async fn origin_validation_middleware(
    State(state): State<SharedState>,
    headers: HeaderMap,
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    if state.allowed_origins == "*" || !state.is_production {
        return next.run(request).await;
    }

    let origin = headers
        .get("origin")
        .or_else(|| headers.get("referer"))
        .and_then(|val| val.to_str().ok());

    if let Some(origin_str) = origin {
        let origin_norm = extract_origin(origin_str);

        let allowed = state
            .allowed_origins
            .split(',')
            .any(|o| extract_origin(o.trim()) == origin_norm);

        if allowed {
            next.run(request).await
        } else {
            (StatusCode::FORBIDDEN, "Forbidden").into_response()
        }
    } else {
        (StatusCode::FORBIDDEN, "Forbidden").into_response()
    }
}

fn extract_origin(url: &str) -> &str {
    if let Some(pos) = url.find("://") {
        let start = pos + 3;
        let rest = &url[start..];
        if let Some(end) = rest.find('/') {
            &url[..start + end]
        } else {
            url
        }
    } else {
        url.trim()
    }
}
