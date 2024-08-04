use std::fs::{File, remove_file};
use std::sync::Arc;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use tokio::sync::Mutex;
use crate::app_state::AppState;
use crate::domain::entities::get_session_file::GetSessionFile;
use crate::domain::models::erroe_message::ErrorMessage;
use crate::domain::models::session_file::SessionFile;

pub async fn delete_session_file_by_id(State(app_state): State<Arc<Mutex<AppState>>>,
                                       Query(get_file): Query<GetSessionFile>,) ->Result<(StatusCode,), (StatusCode, Json<ErrorMessage>)>{



        let state_clone = Arc::clone(&app_state);
        let mut guard = state_clone.lock().await;
        if let Some(index) = guard
            .sessions
            .iter()
            .position(|session| session.session_number == get_file.session_number)
        {
            // let mut session = ;
            let Some(index_file) = guard.sessions[index]
                .files
                .iter()
                .position(|f| f.id == get_file.file_id)
            else {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(ErrorMessage::new(String::from("File is not found"))),
                ));
            };

            let  file =  guard.sessions[index].files.remove(index_file);
            if let Some(path) = file.local_filepath{
                remove_file(path).unwrap();

            }
            return Ok((StatusCode::OK, ));
        }
    Err((
        StatusCode::BAD_REQUEST,
        Json(ErrorMessage::new(String::from("Some error"))),
    ))
        // println!("Length of `{}` is {} bytes", name, data.len());

}