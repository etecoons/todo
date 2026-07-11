use crate::state::SharedState;
use crate::types::TodoState;
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};

/// Read the todos file. Returns the envelope form `{ version, lists }`.
/// Migrates legacy plain-map format transparently and rewrites the file
/// in envelope form on the next save.
pub async fn get_todos(State(state): State<SharedState>) -> Response {
    let data_file = state.data_file.clone();

    // Run the read+parse on a blocking thread — file IO should not
    // block the executor.
    let read_result =
        tokio::task::spawn_blocking(move || match std::fs::read_to_string(&data_file) {
            Ok(content) => {
                let (todo_state, needs_rewrite) = TodoState::parse_with_migration(&content)
                    .map_err(|e| format!("data file is corrupt: {e}"))?;
                Ok::<(TodoState, bool), Box<dyn std::error::Error + Send + Sync>>((
                    todo_state,
                    needs_rewrite,
                ))
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                let default_state = TodoState {
                    version: 0,
                    lists: std::collections::HashMap::new(),
                };
                Ok((default_state, false))
            }
            Err(e) => Err(Box::new(e) as Box<dyn std::error::Error + Send + Sync>),
        })
        .await;

    // Return the full envelope so clients can round-trip `version` on save.
    // Returning only `lists` forced version=0 on the next POST and caused 409s
    // after the first versioned write.
    match read_result {
        Ok(Ok((todo_state, _needs_rewrite))) => Json(todo_state).into_response(),
        Ok(Err(msg)) => (
            StatusCode::SERVICE_UNAVAILABLE,
            format!(
                "{msg}. Please restore from `data/todos.json.bak` or contact the administrator."
            ),
        )
            .into_response(),
        Err(join_err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to read todos: {join_err}"),
        )
            .into_response(),
    }
}

/// Save the todos file with optimistic-concurrency control.
///
/// Body shape can be either the versioned `TodoState` envelope, or the
/// raw legacy `TodoLists` map.
pub async fn save_todos(
    State(state): State<SharedState>,
    Json(value): Json<serde_json::Value>,
) -> Response {
    let data_file = state.data_file.clone();

    // Parse payload as either TodoState or TodoLists
    let (lists, version) = if let Some(lists_val) = value.get("lists") {
        let lists: shared_core::types::TodoLists = match serde_json::from_value(lists_val.clone()) {
            Ok(l) => l,
            Err(e) => {
                return (StatusCode::BAD_REQUEST, format!("Invalid lists field: {e}"))
                    .into_response();
            }
        };
        let version = value.get("version").and_then(|v| v.as_u64()).unwrap_or(0);
        (lists, version)
    } else {
        let lists: shared_core::types::TodoLists = match serde_json::from_value(value) {
            Ok(l) => l,
            Err(e) => {
                return (
                    StatusCode::BAD_REQUEST,
                    format!("Invalid TodoLists payload: {e}"),
                )
                    .into_response();
            }
        };
        (lists, 0)
    };

    // 1. Read the current file to compare versions.
    let current_version: u64 = match tokio::fs::read_to_string(&data_file).await {
        Ok(content) => match TodoState::parse_with_migration(&content) {
            Ok((s, _)) => s.version,
            Err(_) => 0, // Treat corrupt file as version 0; rewrite will heal it.
        },
        Err(_) => 0,
    };

    // 2. Check optimistic-concurrency. A save is accepted only if the
    //    client's observed version matches the current file version.
    if version != current_version && current_version != 0 {
        return (
            StatusCode::CONFLICT,
            Json(serde_json::json!({
                "error": "version_conflict",
                "current_version": current_version,
                "your_version": version,
            })),
        )
            .into_response();
    }

    // 3. Build the new state with version = current + 1.
    let new_state = TodoState {
        version: current_version + 1,
        lists,
    };
    // Capture the new version *before* moving `new_state` into the
    // blocking write below — we need it for the response.
    let new_version = new_state.version;

    // 4. Write atomically: backup current → write to .tmp → rename.
    //    On failure, leave the original file untouched.
    let write_res = tokio::task::spawn_blocking(move || {
        use std::fs::{self, File};
        use std::io::BufWriter;

        // Backup current file (best-effort; ignore failure if file
        // doesn't exist yet).
        if let Ok(content) = fs::read_to_string(&data_file) {
            let backup = format!("{data_file}.bak");
            let _ = fs::write(&backup, content);
        }

        let temp_file = format!("{data_file}.tmp");
        let file = File::create(&temp_file)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &new_state)?;
        fs::rename(&temp_file, &data_file)?;
        Ok::<(), std::io::Error>(())
    })
    .await;

    match write_res {
        Ok(Ok(())) => Json(serde_json::json!({
            "success": true,
            "version": new_version,
        }))
        .into_response(),
        Ok(Err(e)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to save todos: {e}"),
        )
            .into_response(),
        Err(join_err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to save todos: {join_err}"),
        )
            .into_response(),
    }
}
