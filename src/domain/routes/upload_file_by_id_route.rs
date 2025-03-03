use std::{fs::File, io::Write};
use std::time::Duration;
use axum::{
    extract::{Multipart, Query, State},
    http::StatusCode,
    Json,
};
use axum::extract::ws::Message;

use crate::{
    app_state::AppState,
    domain::models::{erroe_message::ErrorMessage, session_file::SessionFile},
};
use crate::domain::entities::get_session_file::GetSessionFile;
use crate::domain::models::websocket_event::WebsocketEvent;
use crate::domain::models::websocket_event_object::WebsocketEventObject;

pub async fn upload_file_by_id_route(
    State(mut app_state): State<AppState>,
    Query(get_file): Query<GetSessionFile>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<SessionFile>), (StatusCode, Json<ErrorMessage>)> {
    let sender =   app_state.move_of(get_file.session_number.clone());

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.file_name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        let extension = name.split(".").last().unwrap();
        let local_filepath = format!("files/{}.{}", get_file.file_id, extension);
        let download_url = format!("http://{}/download/{}/{}",app_state.address,get_file.session_number, get_file.file_id, );
        let mut file = File::create(local_filepath.clone()).unwrap();

        let Ok(_) = file.write_all(data.as_ref()) else {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorMessage::new(String::from("Ошибка записи файла"))),
            ));
        };
        if let Some(mut bundle) = app_state
            .sessions
            .get_mut(&get_file.session_number)
        {


            if let Some(mut ff) = bundle.files.get_mut(&get_file.file_id) {
                let f = (*ff).upload(local_filepath, download_url, data.len()).clone();
                sender.send(Message::Text(serde_json::to_string(&WebsocketEventObject { folder: get_file.root_folder_id, websocket_event_type: WebsocketEvent::FileEvent, data: ff }).unwrap()));


                return Ok((StatusCode::OK, Json(f)));
            }
        };
        // println!("Length of `{}` is {} bytes", name, data.len());
    }
    Err((
        StatusCode::BAD_REQUEST,
        Json(ErrorMessage::new(String::from("Some error"))),
    ))
}
