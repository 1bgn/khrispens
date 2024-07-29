use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use chrono::Utc;
use tokio::sync::Mutex;
use uuid::{uuid, Uuid};
// use tokio::sync::Mutex;

use crate::{app_state::AppState, models::session_bundle::SessionBundle};

pub async fn create_session(
    State(app_state): State<Arc<Mutex<AppState>>>,
) -> (StatusCode, Json<SessionBundle>) {
    let state_clone = Arc::clone(&app_state);
    let mut guard = state_clone.lock().await;
    let session_bundle = SessionBundle::new(guard.sessions.len());
    guard.sessions.push(session_bundle.clone());
    (StatusCode::OK, Json(session_bundle))
}
