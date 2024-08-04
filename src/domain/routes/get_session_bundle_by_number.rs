use std::sync::Arc;

use axum::{
    debug_handler,
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use tokio::sync::Mutex;

use crate::{
    app_state::AppState,
    domain::models::{ session_bundle::SessionBundle},
};
use crate::domain::entities::get_session::GetSession;

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
