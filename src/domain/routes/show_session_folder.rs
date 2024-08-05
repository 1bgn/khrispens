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
use crate::domain::entities::get_session_folder::GetSessionFolder;
use crate::domain::models::session_folder::SessionFolder;
use crate::domain::models::websocket_event::WebsocketEvent;
use crate::domain::models::websocket_event_object::WebsocketEventObject;
use crate::domain::result_models::show_folder::ShowFolder;

#[debug_handler]
pub async fn show_session_folder(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Json(s): Json<GetSessionFolder>,
) -> Result<(StatusCode, Json<ShowFolder>), (StatusCode, &'static str)> {
    let state_clone = Arc::clone(&app_state);
    let mut guard = state_clone.lock().await;
    if let Some(index) = guard
        .sessions
        .iter()
        .position(|session| session.session_number == s.session_number)
    {
        let session= &mut guard.sessions[index];
        if let Some(folder) = session.included_folders.get(&s.system_path){
            // let mut files = folder.included_file_ids.iter().map(|file_id|)

            let show_folder = ShowFolder::new(folder,vec![],vec![]);

            // println!("{:?}",folder);
            // let _ = guard.move_of(index).send(Message::Text(serde_json::to_string(&WebsocketEventObject { websocket_event_type: WebsocketEvent::FileEvent, data: file.clone() }).unwrap()));
            return Ok((StatusCode::OK, Json(show_folder)));
        }else {
            return Err((StatusCode::BAD_REQUEST, "Папка не найдена"));
        }


    }
    Err((StatusCode::BAD_REQUEST, "Session is not found"))
}
