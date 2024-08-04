use std::fs;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use filetravel_backend::app_state::AppState;
use router::create_route;

mod router;

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel(32);
    let path = "files/";
    fs::remove_dir_all(path).unwrap();
    fs::create_dir(path).unwrap();
    let state = AppState { sessions: vec![], broadcast_tx:Arc::new(Mutex::new(tx))};
    let app = create_route(state);
    let address = SocketAddr::from(([192,168,0,27], 3000));
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}
// async fn hello_world() -> &'static str {
//     "hello world"
// }
// async fn create_user() -> (StatusCode, Json<User>) {
//     let user = User {
//         username: "Semion Khrispens".to_string(),
//     };
//     (StatusCode::OK, Json(user))
// }
