use std::{fs::File, io::Write, sync::Arc};

use axum::{
    debug_handler,
    extract::{Multipart, Query, State},
};
use tokio::sync::Mutex;

use crate::{
    app_state::AppState,
    models::{get_file::GetFile, session_file::SessionFile},
};

#[debug_handler]
pub async fn upload_file_by_id(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Query(get_file): Query<GetFile>,

    mut multipart: Multipart,
) {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.file_name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        let extension = name.split(".").last().unwrap();
        let local_filepath = format!("files/{}.{}", get_file.id, extension);
        let download_url = format!("/download/{}.{}", get_file.id, extension);
        let mut file = File::create(local_filepath.clone()).unwrap();
        file.write_all(data.as_ref());
        let state_clone = Arc::clone(&app_state);
        let mut guard = state_clone.lock().await;
        if let Some(index) = guard
            .sessions
            .iter()
            .position(|session| session.session_number == get_file.session_number)
        {
            // let mut session = ;
            let mut indexFile = guard.sessions[index]
                .files
                .iter()
                .position(|f| f.id == get_file.id)
                .unwrap();
            guard.sessions[index].files[indexFile].upload(local_filepath, download_url, data.len());
        }
        println!("Length of `{}` is {} bytes", name, data.len());
    }
}
