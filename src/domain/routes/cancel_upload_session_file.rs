use std::sync::Arc;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use tokio::sync::Mutex;
use crate::app_state::AppState;
use crate::domain::entities::get_session_file::GetSessionFile;
use crate::domain::models::erroe_message::ErrorMessage;

pub  async  fn cancel_upload_session_file(State(app_state): State<Arc<Mutex<AppState>>>,
                                          Query(get_file): Query<GetSessionFile>,)->Result<(StatusCode,), (StatusCode, Json<ErrorMessage>)>{
    let state_clone = Arc::clone(&app_state);
    let mut guard = state_clone.lock().await;
    guard.sessions[get_file.session_number].files.remove(&get_file.file_id);

    return Ok((StatusCode::OK, ));
}