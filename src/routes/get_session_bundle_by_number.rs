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
use serde::de::Error;
use tokio::sync::Mutex;

#[debug_handler]
pub async fn get_session_bundle_by_number(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Query(s): Query<GetSession>,
) -> Result<(StatusCode, Json<SessionBundle>), (StatusCode, &'static str)> {
    let state_clone = Arc::clone(&app_state);
    let guard = state_clone.lock().await;
    if let Some(index) = guard
        .sessions
        .iter()
        .position(|session| session.session_number == s.session_number)
    {
        return Ok((StatusCode::OK, Json(guard.sessions[index].clone())));
    }
    Err((StatusCode::BAD_REQUEST, "Session is not found"))
}
