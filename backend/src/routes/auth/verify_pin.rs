use axum::{
    Json,
    extract::{ConnectInfo, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use axum_extra::extract::cookie::CookieJar;
use std::net::SocketAddr;
use std::time::Duration;

use super::super::{PIN_MAX_LEN, PIN_MIN_LEN};
use crate::auth::secure_compare;
use crate::state::{SharedState, get_client_ip};
use shared_core::types::{VerifyPinRequest, VerifyPinResponse};

pub async fn verify_pin(
    State(state): State<SharedState>,
    connect_info: ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    cookie_jar: CookieJar,
    Json(payload): Json<VerifyPinRequest>,
) -> Response {
    let client_ip = get_client_ip(
        &headers,
        connect_info.0,
        state.trust_proxy,
        &state.trusted_proxies,
    );
    let max_attempts = state.max_attempts as u32;
    let pin_env = match &state.pin {
        Some(p) => p,
        None => {
            return (
                StatusCode::OK,
                Json(VerifyPinResponse {
                    valid: true,
                    error: None,
                    attempts_left: None,
                    locked: None,
                    lockout_minutes: None,
                }),
            )
                .into_response();
        }
    };

    // 1. Lockout check — return 429 without inspecting the PIN.
    if shared_backend::auth::is_locked_out(&client_ip, max_attempts, state.lockout_duration) {
        let remaining =
            shared_backend::auth::lockout_remaining_secs(&client_ip, state.lockout_duration);
        let minutes = remaining.div_ceil(60);
        return (
            StatusCode::TOO_MANY_REQUESTS,
            Json(VerifyPinResponse {
                valid: false,
                error: Some(format!(
                    "Too many attempts. Please try again in {} minutes.",
                    minutes
                )),
                attempts_left: Some(0),
                locked: Some(true),
                lockout_minutes: Some(minutes),
            }),
        )
            .into_response();
    }

    // 2. Format check — return 400 WITHOUT incrementing the attempt
    //    counter. Format errors are not brute-force attempts.
    if payload.pin.len() < PIN_MIN_LEN || payload.pin.len() > PIN_MAX_LEN {
        return (
            StatusCode::BAD_REQUEST,
            Json(VerifyPinResponse {
                valid: false,
                error: Some(format!(
                    "PIN must be between {PIN_MIN_LEN} and {PIN_MAX_LEN} characters"
                )),
                attempts_left: None,
                locked: None,
                lockout_minutes: None,
            }),
        )
            .into_response();
    }

    // 3. Constant-time comparison. Tiny randomised delay to flatten
    //    timing-side-channels between match and mismatch on online
    //    attacks; constant_time_eq itself prevents micro-timing leaks
    //    from the comparison itself.
    let delay_ms = {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        50 + (seed % 101)
    };
    tokio::time::sleep(Duration::from_millis(delay_ms)).await;

    let valid = secure_compare(payload.pin.as_bytes(), pin_env.as_bytes());

    if valid {
        // Success — clear lockout counter, issue session cookie.
        shared_backend::auth::reset_attempts(&client_ip);

        let session_id = crate::session_id::generate_session_id();
        state
            .active_sessions
            .write()
            .await
            .insert(session_id.clone());

        // Todo doesn't have a `base_url` config; the shared helper falls
        // back to checking only the X-Forwarded-Proto header when given
        // an empty base_url, which matches the prior local behaviour.
        let is_secure = crate::cookie_auth::cookie_should_be_secure(&headers, "");

        let cookie = crate::cookie_auth::build_cookie(&session_id,
            state.cookie_max_age_hours,
            is_secure,
        );
        let updated_jar = cookie_jar.add(cookie);

        (
            StatusCode::OK,
            updated_jar,
            Json(VerifyPinResponse {
                valid: true,
                error: None,
                attempts_left: None,
                locked: None,
                lockout_minutes: None,
            }),
        )
            .into_response()
    } else {
        // Failed comparison — increment counter. If this is the attempt
        // that crosses `max_attempts`, the next call will see the
        // lockout.
        let attempt = shared_backend::auth::record_attempt(&client_ip);
        let left = max_attempts.saturating_sub(attempt.count) as usize;

        (
            StatusCode::UNAUTHORIZED,
            Json(VerifyPinResponse {
                valid: false,
                error: Some(format!(
                    "Invalid PIN. {} attempts remaining before lockout.",
                    left
                )),
                attempts_left: Some(left),
                locked: Some(left == 0),
                lockout_minutes: Some(if left == 0 { 15 } else { 0 }),
            }),
        )
            .into_response()
    }
}
