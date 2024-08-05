use std::sync::Arc;

use axum::{
    debug_handler,
    extract::State,
    http::StatusCode,
    Json,
};
use axum::extract::ws::Message;
use serde::Serialize;
use tokio::sync::Mutex;

use crate::{
    app_state::AppState,
    domain::models::session_file::SessionFile,
};
use crate::domain::entities::create_session_file::CreateSessionFile;
use crate::domain::entities::create_session_folder::CreateSessionFolder;
use crate::domain::models::session_folder::SessionFolder;
use crate::domain::models::websocket_event::WebsocketEvent;
use crate::domain::models::websocket_event_object::WebsocketEventObject;

#[debug_handler]
pub async fn add_folder_to_session(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Json(s): Json<CreateSessionFolder>,
) -> Result<(StatusCode, Json<SessionFolder>), (StatusCode, &'static str)> {
    let state_clone = Arc::clone(&app_state);
    let mut guard = state_clone.lock().await;
    if let Some(index) = guard
        .sessions
        .iter()
        .position(|session| session.session_number == s.session_number)
    {
        let session= &mut guard.sessions[index];
        if let Some(folder) = session.included_folders.get(&s.system_path){
            let new_folder = SessionFolder::new(folder.system_path.clone(),&s);

            // println!("{:?}",folder);
            session.add_folder(new_folder.clone());
            // let _ = guard.move_of(index).send(Message::Text(serde_json::to_string(&WebsocketEventObject { websocket_event_type: WebsocketEvent::FileEvent, data: file.clone() }).unwrap()));
            return Ok((StatusCode::OK, Json(new_folder.clone())));
        }else {
            return Err((StatusCode::BAD_REQUEST, "Папка не найдена"));
        }


    }
    Err((StatusCode::BAD_REQUEST, "Session is not found"))
}
