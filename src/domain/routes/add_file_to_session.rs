use std::sync::Arc;

use axum::{
    debug_handler,
    extract::State,
    http::StatusCode,
    Json,
};
use tokio::sync::Mutex;

use crate::{
    app_state::AppState,
    domain::models::session_file::SessionFile,
};
use crate::domain::entities::create_session_file::CreateSessionFile;

#[debug_handler]
pub async fn add_file_to_session(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Json(s): Json<CreateSessionFile>,
) -> Result<(StatusCode, Json<SessionFile>), (StatusCode, &'static str)> {
    let state_clone = Arc::clone(&app_state);
    let mut guard = state_clone.lock().await;
    if let Some(index) = guard
        .sessions
        .iter()
        .position(|session| session.session_number == s.session_number)
    {
        let file = SessionFile::new(s);
        guard.sessions[index].files.push(file.clone());
        return Ok((StatusCode::OK, Json(file)));
    }
    Err((StatusCode::BAD_REQUEST, "Session is not found"))
}
