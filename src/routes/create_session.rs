use std::sync::{Arc, Mutex};

use axum::{extract::State, http::StatusCode, Json};
// use tokio::sync::Mutex;

use crate::{app_state::AppState, models::session_bundle::SessionBundle};

pub async fn create_session(State(app_state): State<Arc<Mutex<AppState>>>) -> Json<AppState> {
    let state_clone = Arc::clone(&app_state);

    let mut guard = state_clone.lock().unwrap();
    let session_bundle = SessionBundle {
        create_at: "".to_string(),
        update_at: "".to_string(),
        session_number: guard.sessions.len().to_string(),
        files: vec![],
        id: 0,
    };
    // let app_state_arc = app_state.clone();
    // let m = *state_clone;
    guard.sessions.push(session_bundle.clone());
    // guard.sessions.len().to_string()
    // usCode::OK
    Json(guard.clone())
}
