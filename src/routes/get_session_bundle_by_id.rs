use std::sync::Arc;

use crate::{
    app_state::AppState,
    models::{get_session::GetSession, session_bundle::SessionBundle},
};
use axum::{
    debug_handler,
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use tokio::sync::Mutex;

#[debug_handler]
pub async fn get_session_bundle_by_id(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Query(s): Query<GetSession>,
) -> (StatusCode, Json<SessionBundle>) {
    let state_clone = Arc::clone(&app_state);
    let guard = state_clone.lock().await;
    let index = guard
        .sessions
        .iter()
        .position(|session| session.id == s.id)
        .unwrap();
    (StatusCode::OK, Json(guard.sessions[index].clone()))
}
