use std::net::SocketAddr;
use std::sync::Arc;
use axum::{Error, extract::ws::{WebSocket, Message}};
use axum::extract::{ConnectInfo, Query, State, WebSocketUpgrade};
use axum::response::Response;
use futures_util::{sink::SinkExt, stream::{StreamExt, SplitSink, SplitStream}};
use tokio::sync::Mutex;
use crate::app_state::AppState;
use crate::domain::entities::get_session::GetSession;

pub async fn ws_handler(Query(s): Query<GetSession>, ws: WebSocketUpgrade, state: State<Arc<Mutex<AppState>>>, ConnectInfo(addr): ConnectInfo<SocketAddr>) ->Response {
    println!("SESSION NUMBER #{}", s.session_number);
    return  ws.on_upgrade(move |socket| handle_socket(socket));
}

async fn handle_socket(mut socket: WebSocket) {
    let (mut sender, mut receiver) = socket.split();

    tokio::spawn(write(sender));
    tokio::spawn(read(receiver));
}

async fn read(receiver: SplitStream<WebSocket>) {
    // ...
}

async fn write(sender: SplitSink<WebSocket, Message>) {
    // ...
}