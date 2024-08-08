use axum::{
    debug_handler,
    extract::State,
    http::StatusCode,
    Json,
};
use axum::extract::ws::Message;

use crate::app_state::AppState;
use crate::domain::entities::create_session_folder::CreateSessionFolder;
use crate::domain::models::session_folder::SessionFolder;
use crate::domain::models::websocket_event::WebsocketEvent;
use crate::domain::models::websocket_event_object::WebsocketEventObject;

#[debug_handler]
pub async fn add_folder_to_session_route(
    State(mut app_state): State<AppState>,
    Json(s): Json<CreateSessionFolder>,
) -> Result<(StatusCode, Json<SessionFolder>), (StatusCode, &'static str)> {
    // let state_clone = Arc::clone(&app_state);
    // let mut guard = state_clone.lock().await;
    let sender = app_state.move_of(s.session_number);
    if let Some(mut bundle) = app_state
        .sessions
        .get_mut(&s.session_number)
    {
        if let Some(folder) = bundle.included_folders.get_mut(&s.root_folder_id) {
            let new_folder = SessionFolder::new(folder.id, folder.system_path.clone(), &s);

            // println!("{:?}",folder);
            folder.add_folder(&new_folder);
            bundle.add_folder(new_folder.clone());
            sender.send(Message::Text(serde_json::to_string(&WebsocketEventObject { folder: s.root_folder_id, websocket_event_type: WebsocketEvent::FolderCreateEvent, data: new_folder.clone() }).unwrap()));
            return Ok((StatusCode::OK, Json(new_folder.clone())));
        } else {
            return Err((StatusCode::BAD_REQUEST, "Папка не найдена"));
        }
    }
    Err((StatusCode::BAD_REQUEST, "Session is not found"))
}
