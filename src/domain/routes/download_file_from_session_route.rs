use std::time::Duration;

use axum::body::Body;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

use crate::app_state::AppState;
use crate::domain::entities::get_session_file::GetSessionFile;

pub async fn download_file_from_id_route(State(app_state): State<AppState>,
                                         Query(get_file): Query<GetSessionFile>,
) -> Result<impl IntoResponse,()> {

    if let Some(bundle) = app_state
        .sessions
        .get(&get_file.session_number)
    {
        // let mut session = ;


        let f = bundle.files.get(& get_file.file_id).unwrap().clone();
        let file = File::open(f.local_filepath.unwrap()).await.unwrap();

        let body_reader = ReaderStream::new(file);

        let y = Body::from_stream(body_reader);
        // tokio::time::sleep(Duration::from_secs(10)).await;
        return  Ok(y.into_response());
    }
    return Err(());
}