use axum::{
    Router,
    routing::{get, post},
};
use axum::extract::DefaultBodyLimit;
use axum::routing::{delete, patch};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tower_http::services::ServeDir;
use tower_http::trace::DefaultMakeSpan;

use filetravel_backend::app_state::AppState;
use filetravel_backend::domain::routes::add_file_to_session_route::add_file_to_session_route;
use filetravel_backend::domain::routes::add_folder_to_session_route::add_folder_to_session_route;
use filetravel_backend::domain::routes::cancel_upload_session_file_route::cancel_upload_session_file_route;
use filetravel_backend::domain::routes::create_session_route::create_session_route;
use filetravel_backend::domain::routes::delete_session_file_by_id_route::delete_session_file_by_id_route;
use filetravel_backend::domain::routes::delete_session_folder_by_id_route::delete_session_folder_by_id_route;
use filetravel_backend::domain::routes::download_file_from_session_by_id_route::download_file_by_id_route;
use filetravel_backend::domain::routes::download_file_from_session_route::download_file_from_id_route;
use filetravel_backend::domain::routes::get_session_bundle_by_number_route::get_session_bundle_by_number_route;
use filetravel_backend::domain::routes::rename_session_file_route::rename_session_file_route;
use filetravel_backend::domain::routes::rename_session_folder_route::rename_session_folder_route;
use filetravel_backend::domain::routes::show_session_folder_route::show_session_folder_route;
use filetravel_backend::domain::routes::upload_file_by_id_route::upload_file_by_id_route;
use filetravel_backend::domain::routes::ws_handler_route::ws_handler_route;

pub fn create_route(app_state: AppState) -> Router {
    Router::new()
        .nest_service("/", ServeDir::new("../filetravel_frontend/build/web"))
        .route("/download-file-from-id", get(download_file_from_id_route))
        .route("/upload-file-by-id", post(upload_file_by_id_route))
        .layer(DefaultBodyLimit::max(1073741824))
        .route("/session-bundle", get(get_session_bundle_by_number_route))
        .route("/session-bundle", post(create_session_route))
        .route("/add-file-to-session", post(add_file_to_session_route))
        .route("/show-session-folder", get(show_session_folder_route))
        .route("/add-folder-to-session", post(add_folder_to_session_route))
        .route("/delete-session-file",delete(delete_session_file_by_id_route))
        .route("/delete-session-folder",delete(delete_session_folder_by_id_route))
        .route("/rename-session-folder",patch(rename_session_folder_route))
        .route("/rename-session-file",patch(rename_session_file_route))
        .route("/download/:session_number/:file_id",get(download_file_by_id_route))
        .route("/cancel-session-file", post(cancel_upload_session_file_route))
        .route("/session-bundle/ws",get(ws_handler_route))
        .with_state(app_state)

        // .with_state(Arc::new(Mutex::new(State(ws_state))))

        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http() .make_span_with(DefaultMakeSpan::default().include_headers(true)),)
}
