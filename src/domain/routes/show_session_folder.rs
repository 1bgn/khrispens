use std::sync::Arc;

use axum::{
    debug_handler,
    extract::State,
    http::StatusCode,
    Json,
};
use axum::extract::Query;
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
    State(app_state): State<AppState>,
    Query(s): Query<GetSessionFolder>,
) -> Result<(StatusCode, Json<ShowFolder>), (StatusCode, &'static str)> {

    if let Some(bundle) = app_state
        .sessions
        .get(&s.session_number)
    {
        if let Some(folder) = bundle.included_folders.get(&s.root_folder_id){
            let  files:Vec<SessionFile> = folder.included_file_ids.iter().map(|file_id|{bundle.files.get(file_id).unwrap().clone()}).collect();
            let  folders:Vec<SessionFolder> = folder.included_folder_ids.iter().map(|file_id|{bundle.included_folders.get(file_id).unwrap().clone()}).collect();

            let show_folder = ShowFolder::new(folder,files,folders);

            // println!("{:?}",folder);
            // let _ = guard.move_of(index).send(Message::Text(serde_json::to_string(&WebsocketEventObject { websocket_event_type: WebsocketEvent::FileEvent, data: file.clone() }).unwrap()));
            return Ok((StatusCode::OK, Json(show_folder)));
        }else {
            return Err((StatusCode::BAD_REQUEST, "Папка не найдена"));
        }


    }
    Err((StatusCode::BAD_REQUEST, "Session is not found"))
}
