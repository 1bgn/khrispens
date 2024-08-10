use std::str::FromStr;
use std::time::Duration;

use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::http::header;
use axum::response::IntoResponse;
use mime_guess::Mime;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

use crate::app_state::AppState;
use crate::domain::entities::get_session_file::GetSessionFile;

pub async fn download_file_by_id_route(State(app_state): State<AppState>,
                                       Path((session_number,file_id)): Path<(String,i64)>,
) -> Result<impl IntoResponse,()> {

    if let Some(bundle) = app_state
        .sessions
        .get(&session_number)
    {
        // let mut session = ;


        let f = bundle.files.get(&file_id).unwrap().clone();
        let file = File::open(f.local_filepath.unwrap()).await.unwrap();
         let m = mime_guess::from_path(f.file_name).first_or(mime_guess::Mime::from_str("*/*").unwrap());
        let body_reader = ReaderStream::new(file);
        let header = [
            (header::CONTENT_TYPE, m.to_string())
        ];

        // let header = [
        //     (header::CONTENT_TYPE, "audio/mpeg"),
        //     (header::CONTENT_LENGTH, ""),
        //     (header::TRANSFER_ENCODING, "chunked"),
        // ];
        let y = Body::from_stream(body_reader);
        // tokio::time::sleep(Duration::from_secs(10)).await;
        return  Ok((header,y.into_response()));
    }
    return Err(());
}