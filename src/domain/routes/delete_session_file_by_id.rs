use std::fs::{File, remove_file};
use std::sync::Arc;
use axum::extract::{Query, State};
use axum::extract::ws::Message;
use axum::http::StatusCode;
use axum::Json;
use tokio::sync::Mutex;
use crate::app_state::AppState;
use crate::domain::entities::get_session_file::GetSessionFile;
use crate::domain::models::erroe_message::ErrorMessage;
use crate::domain::models::session_file::SessionFile;
use crate::domain::models::websocket_event::WebsocketEvent;
use crate::domain::models::websocket_event_object::WebsocketEventObject;

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
            let  file =  guard.sessions[index].files.remove(&get_file.file_id).unwrap();
            let folder = guard.sessions[index].included_folders.get_mut(&get_file.root_folder_id).unwrap();
            let pos = folder.included_file_ids.iter().position(|s|s==&get_file.file_id).unwrap();
            folder.included_file_ids.remove(pos);
            if let Some(ref path) = file.local_filepath{
                remove_file(path).unwrap();

            }
            let k = guard.move_of(get_file.session_number).send(Message::Text(serde_json::to_string(&WebsocketEventObject { websocket_event_type: WebsocketEvent::FileEventDeleted,folder:get_file.root_folder_id, data: file.clone() }).unwrap()));
            return Ok((StatusCode::OK, ));
        }
    Err((
        StatusCode::BAD_REQUEST,
        Json(ErrorMessage::new(String::from("Some error"))),
    ))
        // println!("Length of `{}` is {} bytes", name, data.len());

}