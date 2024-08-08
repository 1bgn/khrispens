use std::fs::remove_file;

use axum::extract::{Query, State};
use axum::extract::ws::Message;
use axum::http::StatusCode;
use axum::Json;

use crate::app_state::AppState;
use crate::domain::entities::get_session_file::GetSessionFile;
use crate::domain::entities::rename_session_file::RenameSessionFile;
use crate::domain::models::erroe_message::ErrorMessage;
use crate::domain::models::websocket_event::WebsocketEvent;
use crate::domain::models::websocket_event_object::WebsocketEventObject;

pub async fn rename_session_file_route(State(app_state): State<AppState>,
                                       Query(get_file): Query<RenameSessionFile>,) ->Result<(StatusCode,), (StatusCode, Json<ErrorMessage>)>{




    if let Some(mut bundle) = app_state
        .sessions
        .get_mut(&get_file.session_number)
    {
        // let mut session = ;
        let  file =  bundle.files.get_mut(&get_file.file_id).unwrap().rename(get_file.new_name);

        let k = app_state.move_of(get_file.session_number).send(Message::Text(serde_json::to_string(&WebsocketEventObject { websocket_event_type: WebsocketEvent::FileRenamedEvent,folder:get_file.root_folder_id, data: file.clone() }).unwrap()));
        return Ok((StatusCode::OK, ));
    }
    Err((
        StatusCode::BAD_REQUEST,
        Json(ErrorMessage::new(String::from("Some error"))),
    ))
    // println!("Length of `{}` is {} bytes", name, data.len());

}