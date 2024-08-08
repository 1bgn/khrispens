use axum::{
    debug_handler,
    extract::{Query, State},
    http::StatusCode,
    Json,
};

use crate::{
    app_state::AppState,
    domain::models::session_bundle::SessionBundle,
};
use crate::domain::entities::get_session::GetSession;
use crate::domain::models::erroe_message::ErrorMessage;

#[debug_handler]
pub async fn get_session_bundle_by_number_route(
    State(mut app_state): State<AppState>,
    Query(s): Query<GetSession>,
) -> Result<(StatusCode, Json<SessionBundle>), (StatusCode, Json<ErrorMessage>)> {
    let guard = app_state.sessions;
    if let Some(v) = guard
        .get(&s.session_number)
    {
        return Ok((StatusCode::OK, Json(v.clone())));
    };
    Err((StatusCode::BAD_REQUEST, Json(ErrorMessage::new(String::from("Сессия не найдена")))))
}
