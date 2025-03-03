use std::fs;
use std::net::SocketAddr;
use std::path::Path;
use filetravel_backend::app_state::AppState;
use router::create_route;

mod router;

#[tokio::main]
async fn main() {
    let path = "files/";
    if Path::new(path).exists(){
        fs::remove_dir_all(path).unwrap();
    }
    fs::create_dir(path).unwrap();
    let url =String::from("192.168.3.8:3000");
    let address = tokio::net::TcpListener::bind(url.clone()).await.unwrap();
    let app = create_route(AppState::new(url));

    // let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    axum::serve(address, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
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
