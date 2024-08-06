use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use axum::extract::{DefaultBodyLimit, State};
use axum::routing::delete;
use tokio::sync::Mutex;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
// use tokio::sync::Mutex;
use tower_http::services::ServeDir;
use tower_http::trace::DefaultMakeSpan;
use filetravel_backend::{
    app_state::AppState,

};
use filetravel_backend::domain::routes::add_file_to_session::add_file_to_session;
use filetravel_backend::domain::routes::add_folder_to_session::add_folder_to_session;
use filetravel_backend::domain::routes::cancel_upload_session_file::cancel_upload_session_file;
use filetravel_backend::domain::routes::create_session::create_session;
use filetravel_backend::domain::routes::delete_session_file_by_id::delete_session_file_by_id;
use filetravel_backend::domain::routes::delete_session_folder_by_id::delete_session_folder_by_id;
use filetravel_backend::domain::routes::download_file_from_session::download_file_from_id;
use filetravel_backend::domain::routes::get_session_bundle_by_number::get_session_bundle_by_number;
use filetravel_backend::domain::routes::show_session_folder::show_session_folder;
use filetravel_backend::domain::routes::upload_file_by_id::upload_file_by_id;
use filetravel_backend::domain::routes::ws_handler::ws_handler;

pub fn create_route(app_state: AppState) -> Router {
    Router::new()
        .nest_service("/", ServeDir::new("../filetravel_frontend/build/web"))
        .route("/download-file-from-id", get(download_file_from_id))
        .route("/upload-file-by-id", post(upload_file_by_id))
        .layer(DefaultBodyLimit::max(1073741824))
        .route("/session-bundle", get(get_session_bundle_by_number))
        .route("/session-bundle", post(create_session))
        .route("/add-file-to-session", post(add_file_to_session))
        .route("/show-session-folder", get(show_session_folder))
        .route("/add-folder-to-session", post(add_folder_to_session))
        .route("/delete-session-file",delete(delete_session_file_by_id))
        .route("/delete-session-folder",delete(delete_session_folder_by_id))
        .route("/cancel-session-file", post(cancel_upload_session_file))
        .route("/session-bundle/ws",get(ws_handler))
        .with_state(app_state)

        // .with_state(Arc::new(Mutex::new(State(ws_state))))

        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http() .make_span_with(DefaultMakeSpan::default().include_headers(true)),)
}
