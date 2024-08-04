use std::sync::Arc;

use axum::body::Body;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use tokio::fs::File;
use tokio::sync::Mutex;
use tokio_util::io::ReaderStream;

use crate::app_state::AppState;
use crate::domain::entities::get_session_file::GetSessionFile;

pub async fn download_file_from_id(State(app_state): State<Arc<Mutex<AppState>>>,
                                   Query(get_file): Query<GetSessionFile>,
) -> Result<impl IntoResponse,()> {
    let state_clone = Arc::clone(&app_state);
    let  guard = state_clone.lock().await;
    if let Some(index) = guard
        .sessions
        .iter()
        .position(|session| session.session_number == get_file.session_number)
    {
        // let mut session = ;
        let index_file =
            guard.sessions[index]
                .files
                .iter()
                .position(|f| f.id == get_file.file_id).unwrap();


        let f = guard.sessions[index].files[index_file].clone();
        let file = File::open(f.local_filepath.unwrap()).await.unwrap();

        let body_reader = ReaderStream::new(file);

        let y = Body::from_stream(body_reader);
        return  Ok(y.into_response());
    }
    return Err(());
}