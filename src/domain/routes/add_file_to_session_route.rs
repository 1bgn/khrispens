use std::sync::Arc;
use std::time::Duration;

use axum::{
    debug_handler,
    extract::State,
    http::StatusCode,
    Json,
};
use axum::extract::ws::Message;

use crate::{
    app_state::AppState,
    domain::models::session_file::SessionFile,
};
use crate::domain::entities::create_session_file::CreateSessionFile;
use crate::domain::models::websocket_event::WebsocketEvent;
use crate::domain::models::websocket_event_object::WebsocketEventObject;

#[debug_handler]
pub async fn add_file_to_session_route(
    State( mut app_state): State<AppState>,
    Json(s): Json<CreateSessionFile>,
) -> Result<(StatusCode, Json<SessionFile>), (StatusCode, &'static str)> {
    let mut guard = Arc::clone(&app_state.sessions);

    let sender =  app_state.move_of(s.session_number.clone());

    if let Some(mut bundle) = guard.get_mut(&s.session_number)
    {
        // let session= ;
        let file = SessionFile::new(&s);
        if let Some(folder) =  bundle.included_folders.get_mut(&s.root_folder_id){
            folder.add_file(&file);

            bundle.files.insert(file.id,file.clone());
            let _ = sender.send(Message::Text(serde_json::to_string(&WebsocketEventObject { folder:s.root_folder_id,websocket_event_type: WebsocketEvent::FileEvent, data: file.clone() }).unwrap()));
            return Ok((StatusCode::OK, Json(file)));
        }else {
            return Err((StatusCode::BAD_REQUEST, "Папка не найдена"));
        }


    }
    Err((StatusCode::BAD_REQUEST, "Session is not found"))
}
