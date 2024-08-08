use std::fs::remove_file;

use axum::extract::{Query, State};
use axum::extract::ws::Message;
use axum::http::StatusCode;
use axum::{debug_handler, Json};

use crate::app_state::AppState;
use crate::domain::entities::get_session_folder::GetSessionFolder;
use crate::domain::entities::rename_session_folder::RenameSessionFolder;
use crate::domain::models::erroe_message::ErrorMessage;
use crate::domain::models::session_bundle::SessionBundle;
use crate::domain::models::session_folder::SessionFolder;
use crate::domain::models::websocket_event::WebsocketEvent;
use crate::domain::models::websocket_event_object::WebsocketEventObject;

#[debug_handler]
pub async fn rename_session_folder_route(State(app_state): State<AppState>,
                                         Query(rename_session_folder): Query<RenameSessionFolder>, ) -> Result<(StatusCode, Json<SessionFolder>), (StatusCode, Json<ErrorMessage>)> {
    if let Some(mut bundle) = app_state
        .sessions
        .get_mut( &rename_session_folder.session_number)
    {

            let mut folder = bundle.included_folders.get_mut(&rename_session_folder.root_folder_id).unwrap();


            folder.rename_folder(rename_session_folder.new_name);


        let k = app_state.move_of(rename_session_folder.session_number).send(Message::Text(serde_json::to_string(&WebsocketEventObject { websocket_event_type: WebsocketEvent::FolderRenamedEvent, folder: folder.parent_id, data: folder.clone() }).unwrap()));


        return Ok((StatusCode::OK,Json(folder.clone())));
    }
    Err((
        StatusCode::BAD_REQUEST,
        Json(ErrorMessage::new(String::from("Some error"))),
    ))
    // println!("Length of `{}` is {} bytes", name, data.len());
}
