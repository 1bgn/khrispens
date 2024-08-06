use std::fs::{File, remove_file};
use std::sync::Arc;
use axum::extract::{Query, State};
use axum::extract::ws::Message;
use axum::http::StatusCode;
use axum::Json;
use tokio::sync::Mutex;
use crate::app_state::AppState;
use crate::domain::entities::get_session_file::GetSessionFile;
use crate::domain::entities::get_session_folder::GetSessionFolder;
use crate::domain::models::erroe_message::ErrorMessage;
use crate::domain::models::session_bundle::SessionBundle;
use crate::domain::models::session_file::SessionFile;
use crate::domain::models::session_folder::SessionFolder;
use crate::domain::models::websocket_event::WebsocketEvent;
use crate::domain::models::websocket_event_object::WebsocketEventObject;

pub async fn delete_session_folder_by_id(State(app_state): State<Arc<Mutex<AppState>>>,
                                         Query(get_file): Query<GetSessionFolder>, ) -> Result<(StatusCode,), (StatusCode, Json<ErrorMessage>)> {
    let state_clone = Arc::clone(&app_state);
    let mut guard = &mut state_clone.lock().await;
    if let Some(index) = guard
        .sessions
        .iter()
        .position(|session| session.session_number == get_file.session_number)
    {
        let mut session = &mut guard.sessions[index];
        {
            // let  s = & guard.sessions[index];
            // let mut folder =  session.included_folders.get(&get_file.root_folder_id).unwrap();
            let mut folder = session.included_folders.get(&get_file.root_folder_id).unwrap();

            let (file_ids, mut folder_ids) = recursive_delete_folder(&folder, &session);
            file_ids.iter().for_each(|file_id| {
                let file = session.files.get(file_id).unwrap();
                if let Some(ref path) = file.local_filepath {
                    remove_file(path).unwrap();
                }
                session.files.remove(file_id);
            });
            folder_ids.iter_mut().for_each(|folder_id| {
                session.included_folders.remove(folder_id);
            });
        }
        let folder = session.included_folders.get(&get_file.root_folder_id).unwrap().clone();
        let parent = session.included_folders.get_mut(&folder.parent_id).unwrap();
        let pos = parent.included_folder_ids.iter().position(|s| s == &folder.id).unwrap();
        parent.included_folder_ids.remove(pos);

        session.included_folders.remove(&get_file.root_folder_id);

        let k = guard.move_of(get_file.session_number).send(Message::Text(serde_json::to_string(&WebsocketEventObject { websocket_event_type: WebsocketEvent::FolderDeletedEvent, folder: folder.parent_id, data: folder.clone() }).unwrap()));


        return Ok((StatusCode::OK,));
    }
    Err((
        StatusCode::BAD_REQUEST,
        Json(ErrorMessage::new(String::from("Some error"))),
    ))
    // println!("Length of `{}` is {} bytes", name, data.len());
}
fn recursive_delete_folder(session_folder: &SessionFolder, session_bundle: &SessionBundle) -> (Vec<i64>, Vec<i64>) {
    let mut file_ids: Vec<i64> = recursive_complete_file_ids(vec![], &session_folder, session_bundle);
    let mut folder_ids: Vec<i64> = recursive_complete_folder_ids(vec![], &session_folder, session_bundle);
    ;
    ;
    println!("files: {:?}", &file_ids.clone());
    println!("folders: {:?}", &folder_ids.clone());


    return (file_ids, folder_ids);
}
fn recursive_complete_file_ids(mut file_ids: Vec<i64>, session_folder: &SessionFolder, session_bundle: &SessionBundle) -> Vec<i64> {
    session_folder.included_folder_ids.iter().for_each(|folder_id| {
        let res = &mut recursive_complete_file_ids(vec![], session_bundle.included_folders.get(folder_id).unwrap(), session_bundle);
        file_ids.append(res);
    });
    file_ids.append(&mut session_folder.included_file_ids.clone());
    return file_ids;
}
fn recursive_complete_folder_ids(mut folder_ids: Vec<i64>, session_folder: &SessionFolder, session_bundle: &SessionBundle) -> Vec<i64> {
    session_folder.included_folder_ids.iter().for_each(|folder_id| {
        let res = &mut recursive_complete_folder_ids(vec![], session_bundle.included_folders.get(folder_id).unwrap(), session_bundle);
        folder_ids.append(res);
    });
    folder_ids.append(&mut session_folder.included_folder_ids.clone());

    return folder_ids;
}