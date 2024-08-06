use std::{fs::File, io::Write, sync::Arc};
use std::sync::mpsc::SendError;
use axum::{
    debug_handler,
    extract::{Multipart, Query, State},
    http::StatusCode,
    Json,
};
use axum::extract::ws::Message;
use tokio::sync::Mutex;

use crate::{
    app_state::AppState,
    domain::models::{erroe_message::ErrorMessage, session_file::SessionFile},
};
use crate::domain::entities::get_session_file::GetSessionFile;
use crate::domain::models::websocket_event::WebsocketEvent;
use crate::domain::models::websocket_event_object::WebsocketEventObject;


pub async fn upload_file_by_id(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Query(get_file): Query<GetSessionFile>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<SessionFile>), (StatusCode, Json<ErrorMessage>)> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.file_name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        let extension = name.split(".").last().unwrap();
        let local_filepath = format!("files/{}.{}", get_file.file_id, extension);
        let download_url = format!("/download/{}.{}", get_file.file_id, extension);

        let state_clone = Arc::clone(&app_state);
        let  mut guard = state_clone.lock().await;
        if let Some(index) = guard
            .sessions
            .iter()
            .position(|session| session.session_number == get_file.session_number)
        {
            // let mut session = ;
            let Some(index_file) = guard.clone().sessions[index]
                .files
                .get(&get_file.file_id)
            else {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(ErrorMessage::new(String::from("File is not found"))),
                ));
            };
            let mut file = File::create(local_filepath.clone()).unwrap();

            let Ok(_) = file.write_all(data.as_ref()) else {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(ErrorMessage::new(String::from("Ошибка записи файла"))),
                ));
            };
           if let Some(ff) = (guard.sessions[index]).files.get_mut(&get_file.file_id){
               let  f =  ff.upload(local_filepath, download_url, data.len()).clone();
               {
                   // let sender =;
                   let _=  guard.move_of(index).send(Message::Text(serde_json::to_string(&WebsocketEventObject { folder:get_file.root_folder_id,websocket_event_type: WebsocketEvent::FileEvent, data: f.clone() }).unwrap()));
               }


               return Ok((StatusCode::OK, Json(f)));
           }


        }
        // println!("Length of `{}` is {} bytes", name, data.len());
    }
    Err((
        StatusCode::BAD_REQUEST,
        Json(ErrorMessage::new(String::from("Some error"))),
    ))
}
