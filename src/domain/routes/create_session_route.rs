use axum::{extract::State, http::StatusCode, Json};
use nanoid::nanoid;
use crate::{app_state::AppState, domain::models::session_bundle::SessionBundle};

// use tokio::sync::Mutex;


pub async fn create_session_route(
    State(mut app_state): State<AppState>,
) -> (StatusCode, Json<SessionBundle>) {
    // let mut state_clone = Arc::clone(&mut app_state.sessions);
    //         tokio::time::sleep(Duration::from_secs(10)).await;

    let id = nanoid!(6,&nanoid::alphabet::SAFE);

    let session_bundle = SessionBundle::new(id.clone());
    let mut map = &app_state.sessions;
        map.insert(id,session_bundle.clone()) ;


    (StatusCode::OK, Json(session_bundle))
}

// fn random (size: usize) -> Vec<u8> {
//     let result: Vec<u8> = vec![0; size];
//
//     result
// }

