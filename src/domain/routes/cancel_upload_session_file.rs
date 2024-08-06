use std::sync::Arc;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use tokio::sync::Mutex;
use crate::app_state::AppState;
use crate::domain::entities::get_session_file::GetSessionFile;
use crate::domain::models::erroe_message::ErrorMessage;

pub  async  fn cancel_upload_session_file(State(app_state): State<AppState>,
                                          Query(get_file): Query<GetSessionFile>,)->Result<(StatusCode,), (StatusCode, Json<ErrorMessage>)>{
 if let Some(mut bundle) = app_state.sessions.get_mut(&get_file.session_number) {
     bundle.files.remove(&get_file.file_id);
 }

    return Ok((StatusCode::OK, ));
}