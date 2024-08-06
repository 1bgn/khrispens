use std::ops::Deref;
use std::os::macos::raw::stat;
use std::sync::Arc;

use axum::{
    debug_handler,
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use tokio::sync::{Mutex, MutexGuard};

use crate::{
    app_state::AppState,
    domain::models::{ session_bundle::SessionBundle},
};
use crate::domain::entities::get_session::GetSession;

#[debug_handler]
pub async fn get_session_bundle_by_number(
    State(mut app_state): State<AppState>,
    Query(s): Query<GetSession>,
) -> Result<(StatusCode, Json<SessionBundle>), (StatusCode, &'static str)> {
    let guard = app_state.sessions;
    if let Some(v) = guard
        .get(&s.session_number)
    {
        return Ok((StatusCode::OK, Json(v.clone())));
    };
    Err((StatusCode::BAD_REQUEST, "Session is not found"))
}
