use std::fs::remove_file;

use axum::extract::{Query, State};
use axum::extract::ws::Message;
use axum::http::StatusCode;
use axum::Json;

use crate::app_state::AppState;
use crate::domain::entities::get_session_folder::GetSessionFolder;
use crate::domain::models::erroe_message::ErrorMessage;
use crate::domain::models::session_bundle::SessionBundle;
use crate::domain::models::session_folder::SessionFolder;
use crate::domain::models::websocket_event::WebsocketEvent;
use crate::domain::models::websocket_event_object::WebsocketEventObject;

pub async fn delete_session_folder_by_id_route(State(app_state): State<AppState>,
                                               Query(get_file): Query<GetSessionFolder>, ) -> Result<(StatusCode,), (StatusCode, Json<ErrorMessage>)> {
    if let Some(mut bundle) = app_state
        .sessions
        .get_mut( &get_file.session_number)
    {
        {
            // let  s = & guard.sessions[index];
            // let mut folder =  session.included_folders.get(&get_file.root_folder_id).unwrap();
            let mut folder = bundle.included_folders.get(&get_file.root_folder_id).unwrap();

            let (file_ids, mut folder_ids) = recursive_delete_folder(&folder, &bundle);
            file_ids.iter().for_each(|file_id| {
                let file = bundle.files.get(file_id).unwrap();
                if let Some(ref path) = file.local_filepath {
                    remove_file(path).unwrap();
                }
                bundle.files.remove(file_id);
            });
            folder_ids.iter_mut().for_each(|folder_id| {
                bundle.included_folders.remove(folder_id);
            });
        }

        let folder = bundle.included_folders.get(&get_file.root_folder_id).unwrap().clone();
        bundle.included_folders.remove(&get_file.root_folder_id);
        let parent = bundle.included_folders.get_mut(&folder.parent_id).unwrap();
        let pos = parent.included_folder_ids.iter().position(|s| s == &folder.id).unwrap();
        parent.included_folder_ids.remove(pos);


        let k = app_state.move_of(get_file.session_number).send(Message::Text(serde_json::to_string(&WebsocketEventObject { websocket_event_type: WebsocketEvent::FolderDeletedEvent, folder: folder.parent_id, data: folder.clone() }).unwrap()));


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